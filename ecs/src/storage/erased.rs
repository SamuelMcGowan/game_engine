use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::{Entry, Values, ValuesMut};
use std::collections::HashMap;

use super::{BorrowError, BorrowResult, Storage};

// THANKS TO: https://lucumr.pocoo.org/2022/1/7/as-any-hack/

trait AnyStorage: Storage + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_storage(&self) -> &dyn Storage;
    fn as_storage_mut(&mut self) -> &mut dyn Storage;
}

impl<T: Storage + Any> AnyStorage for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_storage(&self) -> &dyn Storage {
        self
    }

    fn as_storage_mut(&mut self) -> &mut dyn Storage {
        self
    }
}

struct ErasedStorage(Box<dyn AnyStorage>);

impl ErasedStorage {
    pub fn new<T: AnyStorage>(storage: T) -> Self {
        Self(Box::new(storage))
    }

    pub fn as_any(&self) -> &dyn Any {
        (*self.0).as_any()
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        (*self.0).as_any_mut()
    }

    pub fn as_storage(&self) -> &dyn Storage {
        (*self.0).as_storage()
    }

    pub fn as_storage_mut(&mut self) -> &mut dyn Storage {
        (*self.0).as_storage_mut()
    }
}

#[derive(Default)]
pub(crate) struct ErasedStorages {
    storages: HashMap<TypeId, RefCell<ErasedStorage>>,
}

impl ErasedStorages {
    pub fn insert<T: Any + Storage>(&mut self, storage: T) -> Option<()> {
        let type_id = TypeId::of::<T>();

        match self.storages.entry(type_id) {
            Entry::Vacant(vacant) => {
                vacant.insert(RefCell::new(ErasedStorage::new(storage)));
                Some(())
            }
            Entry::Occupied(_) => None,
        }
    }

    pub fn borrow_ref<T: Any + Storage>(&self) -> BorrowResult<Ref<T>> {
        let type_id = TypeId::of::<T>();

        let erased_storage = self
            .storages
            .get(&type_id)
            .ok_or(BorrowError::StorageNotFound)?;

        let erased_storage_ref = erased_storage
            .try_borrow()
            .map_err(|_| BorrowError::InvalidBorrow)?;

        let storage = Ref::map(erased_storage_ref, |any| {
            any.as_any().downcast_ref().unwrap()
        });

        Ok(storage)
    }

    pub fn borrow_mut<T: Any + Storage>(&self) -> BorrowResult<RefMut<T>> {
        let type_id = TypeId::of::<T>();

        let erased_storage = self
            .storages
            .get(&type_id)
            .ok_or(BorrowError::StorageNotFound)?;

        let erased_storage_mut = erased_storage
            .try_borrow_mut()
            .map_err(|_| BorrowError::InvalidBorrow)?;

        let storage = RefMut::map(erased_storage_mut, |any| {
            any.as_any_mut().downcast_mut().unwrap()
        });

        Ok(storage)
    }

    pub fn iter_refs(&self) -> ErasedStorageIter {
        ErasedStorageIter(self.storages.values())
    }

    pub fn iter_muts(&mut self) -> ErasedStorageIterMut {
        ErasedStorageIterMut(self.storages.values_mut())
    }
}

pub(crate) struct ErasedStorageIter<'a>(Values<'a, TypeId, RefCell<ErasedStorage>>);

impl<'a> Iterator for ErasedStorageIter<'a> {
    type Item = BorrowResult<Ref<'a, dyn Storage>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|erased_storage| {
            let erased_storage_mut = erased_storage
                .try_borrow()
                .map_err(|_| BorrowError::InvalidBorrow)?;

            let storage = Ref::map(erased_storage_mut, |storage| storage.as_storage());

            Ok(storage)
        })
    }
}

pub(crate) struct ErasedStorageIterMut<'a>(ValuesMut<'a, TypeId, RefCell<ErasedStorage>>);

impl<'a> Iterator for ErasedStorageIterMut<'a> {
    type Item = BorrowResult<RefMut<'a, dyn Storage>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|erased_storage| {
            let erased_storage_mut = erased_storage
                .try_borrow_mut()
                .map_err(|_| BorrowError::InvalidBorrow)?;

            let storage = RefMut::map(erased_storage_mut, |storage| storage.as_storage_mut());

            Ok(storage)
        })
    }
}
