use std::any::Any;
use std::cell::{Ref, RefMut};

use super::erased::ErasedStorages;
use super::unique::UniqueStorage;
use super::Storage;
use crate::prelude::*;

#[derive(Default)]
pub(crate) struct AllStorages {
    pub(crate) entities: EntityStorage,
    components: ErasedStorages,
    unique: ErasedStorages,
}

impl AllStorages {
    #[inline]
    pub fn spawn(&mut self) -> EntityMut {
        let entity = self.entities.spawn();
        EntityMut::new(entity, self)
    }

    #[inline]
    pub fn entity(&mut self, entity: EntityId) -> EntityMut {
        if !self.entities.is_alive(entity) {
            panic!("entity {entity:?} is dead");
        }

        EntityMut::new(entity, self)
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
