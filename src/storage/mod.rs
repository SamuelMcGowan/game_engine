mod components;
mod erased;
mod sparse_set;

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
}

impl AllStorages {
    pub fn register_components<C: Component>(&mut self) -> Result<(), StorageOccupied> {
        self.components.insert(ComponentStorage::<C>::default())
    }

    pub fn all_components_ref<C: Component>(&self) -> BorrowResult<Ref<ComponentStorage<C>>> {
        self.components.borrow_ref()
    }

    pub fn all_components_mut<C: Component>(&self) -> BorrowResult<RefMut<ComponentStorage<C>>> {
        self.components.borrow_mut()
    }

    pub fn component_ref<C: Component>(&self, entity: EntityId) -> BorrowResult<Ref<C>> {
        let storage = self.all_components_ref::<C>()?;
        Ref::filter_map(storage, |storage| storage.get(entity))
            .map_err(|_| BorrowError::ValueNotFound)
    }

    pub fn component_mut<C: Component>(&self, entity: EntityId) -> BorrowResult<RefMut<C>> {
        let storage = self.all_components_mut::<C>()?;
        RefMut::filter_map(storage, |storage| storage.get_mut(entity))
            .map_err(|_| BorrowError::ValueNotFound)
    }
}
