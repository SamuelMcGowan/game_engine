use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::storage::all_storages::{BorrowError, BorrowResult};

#[derive(Debug)]
pub struct StorageOccupied;

#[derive(Default)]
pub struct ErasedStorage {
    storage: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl ErasedStorage {
    pub fn insert<T: Any>(&mut self, element: T) -> Result<(), StorageOccupied> {
        let type_id = TypeId::of::<T>();

        match self.storage.entry(type_id) {
            Entry::Vacant(vacant) => {
                vacant.insert(RefCell::new(Box::new(element)));
                Ok(())
            }
            Entry::Occupied(_) => Err(StorageOccupied),
        }
    }

    pub fn borrow_ref<T: Any>(&self) -> BorrowResult<Ref<T>> {
        let type_id = TypeId::of::<T>();

        let erased_storage = self
            .storage
            .get(&type_id)
            .ok_or(BorrowError::StorageNotFound)?;

        let erased_storage_ref = erased_storage
            .try_borrow()
            .map_err(|_| BorrowError::InvalidBorrow)?;

        let storage_ref = Ref::map(erased_storage_ref, |any| any.downcast_ref::<T>().unwrap());

        Ok(storage_ref)
    }

    pub fn borrow_mut<T: Any>(&self) -> BorrowResult<RefMut<T>> {
        let type_id = TypeId::of::<T>();

        let erased_storage = self
            .storage
            .get(&type_id)
            .ok_or(BorrowError::StorageNotFound)?;

        let erased_storage_mut = erased_storage
            .try_borrow_mut()
            .map_err(|_| BorrowError::InvalidBorrow)?;

        let storage_mut = RefMut::map(erased_storage_mut, |any| any.downcast_mut::<T>().unwrap());

        Ok(storage_mut)
    }
}
