use std::any::Any;
use std::cell::{Ref, RefMut};

use crate::storage::components::{Component, ComponentStorage};
use crate::storage::entities::LiveEntity;
use crate::storage::entities::{EntityId, EntityStorage};
use crate::storage::erased::ErasedStorage;
use crate::storage::erased::StorageOccupied;

#[derive(Default)]
pub(crate) struct AllStorages {
    entities: EntityStorage,
    components: ErasedStorage,
    unique: ErasedStorage,
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
    pub fn register_components<C: Component>(&mut self) -> Result<(), StorageOccupied> {
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
    pub fn insert_unique<T: Any>(&mut self, unique: T) -> Result<(), StorageOccupied> {
        self.unique.insert(unique)
    }

    #[inline]
    pub fn unique_ref<T: Any>(&self) -> BorrowResult<Ref<T>> {
        self.unique.borrow_ref()
    }

    #[inline]
    pub fn unique_mut<T: Any>(&self) -> BorrowResult<RefMut<T>> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowError {
    InvalidBorrow,
    StorageNotFound,
    ValueNotFound,
}

pub type BorrowResult<T> = Result<T, BorrowError>;
