use std::any::Any;
use std::cell::{Ref, RefMut};

use super::erased::ErasedStorages;
use super::unique::UniqueStorage;
use super::Storage;
use crate::prelude::*;

#[derive(Default)]
pub(crate) struct AllStorages {
    entities: EntityStorage,
    components: ErasedStorages,
    unique: ErasedStorages,
}

impl AllStorages {
    #[inline]
    pub fn spawn(&mut self) -> EntityMut {
        let entity = self.entities.spawn();

        EntityMut {
            all_storages: self,
            entity,
        }
    }

    #[inline]
    pub fn entity(&mut self, entity: EntityId) -> EntityMut {
        if !self.entities.is_alive(entity) {
            panic!("entity {entity:?} is dead");
        }

        EntityMut {
            all_storages: self,
            entity,
        }
    }

    #[inline]
    pub fn register_components<C: Component>(&mut self) -> Option<()> {
        self.components.insert(ComponentStorage::<C>::default())
    }

    #[inline]
    pub fn component_storage_ref<C: Component>(&self) -> BorrowResult<Ref<ComponentStorage<C>>> {
        self.components.borrow_ref()
    }

    #[inline]
    pub fn component_storage_mut<C: Component>(&self) -> BorrowResult<RefMut<ComponentStorage<C>>> {
        self.components.borrow_mut()
    }

    #[inline]
    pub fn insert_unique<T: Any>(&mut self, unique: T) -> Option<()> {
        self.unique.insert(UniqueStorage(unique))
    }

    #[inline]
    pub fn unique_ref<T: Any + Storage>(&self) -> BorrowResult<Ref<T>> {
        self.unique.borrow_ref()
    }

    #[inline]
    pub fn unique_mut<T: Any + Storage>(&self) -> BorrowResult<RefMut<T>> {
        self.unique.borrow_mut()
    }

    #[inline]
    pub fn entity_storage(&self) -> &EntityStorage {
        &self.entities
    }
}

/// A handle to mutate an entity.
pub struct EntityMut<'a> {
    all_storages: &'a mut AllStorages,
    entity: EntityId,
}

impl EntityMut<'_> {
    /// Add a component to the entity.
    ///
    /// Panics if the component type is not registered.
    pub fn insert<C: Component>(&mut self, component: C) -> &mut Self {
        let mut components = self.components_mut::<C>();
        components.insert(self.live(), component);
        drop(components);
        self
    }

    /// Remove a component from an entity.
    ///
    /// Panics if the component type is not registered.
    pub fn remove<C: Component>(&mut self) -> &mut Self {
        let mut components = self.components_mut::<C>();
        components.remove(self.live());
        drop(components);
        self
    }

    /// Get the entity's id.
    #[inline]
    pub fn id(&self) -> EntityId {
        self.entity
    }

    /// Panics if the component type is not registered.
    fn components_mut<C: Component>(&self) -> RefMut<ComponentStorage<C>> {
        self.all_storages
            .component_storage_mut::<C>()
            .expect("component type not registered")
    }

    fn live(&self) -> LiveEntity {
        self.all_storages.entities.entity_to_alive(self.entity)
    }
}
