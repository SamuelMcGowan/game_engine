mod components;
mod erased;
mod sparse_set;

use std::any::Any;
use std::cell::{Ref, RefMut};

use super::world::EntityId;

pub use components::*;
use erased::{ErasedStorage, StorageOccupied};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowError {
    InvalidBorrow,
    StorageNotFound,
    ValueNotFound,
}

pub type BorrowResult<T> = Result<T, BorrowError>;

#[derive(Default)]
pub struct AllStorages {
    components: ErasedStorage,
    unique: ErasedStorage,
}

impl AllStorages {
    pub fn register_components<C: Component>(&mut self) -> Result<(), StorageOccupied> {
        self.components.insert(ComponentStorage::<C>::default())
    }

    pub fn component_storage_ref<C: Component>(&self) -> BorrowResult<Ref<ComponentStorage<C>>> {
        self.components.borrow_ref()
    }

    pub fn component_storage_mut<C: Component>(&self) -> BorrowResult<RefMut<ComponentStorage<C>>> {
        self.components.borrow_mut()
    }

    pub fn component_ref<C: Component>(&self, entity: EntityId) -> BorrowResult<Ref<C>> {
        let storage = self.component_storage_ref::<C>()?;
        Ref::filter_map(storage, |storage| storage.get(entity))
            .map_err(|_| BorrowError::ValueNotFound)
    }

    pub fn component_mut<C: Component>(&self, entity: EntityId) -> BorrowResult<RefMut<C>> {
        let storage = self.component_storage_mut::<C>()?;
        RefMut::filter_map(storage, |storage| storage.get_mut(entity))
            .map_err(|_| BorrowError::ValueNotFound)
    }

    pub fn insert_unique<T: Any>(&mut self, unique: T) -> Result<(), StorageOccupied> {
        self.unique.insert(unique)
    }

    pub fn unique_ref<T: Any>(&self) -> BorrowResult<Ref<T>> {
        self.unique.borrow_ref()
    }

    pub fn unique_mut<T: Any>(&self) -> BorrowResult<RefMut<T>> {
        self.unique.borrow_mut()
    }
}
