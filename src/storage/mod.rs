mod components;
mod sparse_set;

use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use super::world::EntityId;
use super::EcsError;

pub use components::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowError {
    InvalidBorrow,
    StorageNotFound,
    ComponentNotFound,
}

pub type BorrowResult<T> = Result<T, BorrowError>;

#[derive(Default)]
pub struct AllStorages {
    components: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl AllStorages {
    pub fn register_components<C: Component>(&mut self) -> Result<(), EcsError> {
        let type_id = TypeId::of::<C>();

        match self.components.entry(type_id) {
            Entry::Vacant(vacant) => {
                vacant.insert(RefCell::new(Box::new(ComponentStorage::<C>::default())));
                Ok(())
            }
            Entry::Occupied(_) => Err(EcsError::StorageAlreadyAdded),
        }
    }

    pub fn component_storage<C: Component>(&self) -> BorrowResult<Ref<ComponentStorage<C>>> {
        let type_id = TypeId::of::<C>();

        let erased_storage = self
            .components
            .get(&type_id)
            .ok_or(BorrowError::StorageNotFound)?;

        let erased_storage_ref = erased_storage
            .try_borrow()
            .map_err(|_| BorrowError::InvalidBorrow)?;

        let storage_ref = Ref::map(erased_storage_ref, |any| {
            any.downcast_ref::<ComponentStorage<C>>().unwrap()
        });

        Ok(storage_ref)
    }

    pub fn component_storage_mut<C: Component>(&self) -> BorrowResult<RefMut<ComponentStorage<C>>> {
        let type_id = TypeId::of::<C>();

        let erased_storage = self
            .components
            .get(&type_id)
            .ok_or(BorrowError::StorageNotFound)?;

        let erased_storage_mut = erased_storage
            .try_borrow_mut()
            .map_err(|_| BorrowError::InvalidBorrow)?;

        let storage_mut = RefMut::map(erased_storage_mut, |any| {
            any.downcast_mut::<ComponentStorage<C>>().unwrap()
        });

        Ok(storage_mut)
    }

    pub fn component<C: Component>(&self, entity: EntityId) -> BorrowResult<Ref<C>> {
        let storage = self.component_storage::<C>()?;
        Ref::filter_map(storage, |storage| storage.get(entity))
            .map_err(|_| BorrowError::ComponentNotFound)
    }

    pub fn component_mut<C: Component>(&self, entity: EntityId) -> BorrowResult<RefMut<C>> {
        let storage = self.component_storage_mut::<C>()?;
        RefMut::filter_map(storage, |storage| storage.get_mut(entity))
            .map_err(|_| BorrowError::ComponentNotFound)
    }
}
