use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::slice::{Iter, IterMut};

use crate::prelude::{BorrowError, BorrowResult};
use crate::storage::StorageWithEntities;

// THANKS TO: https://lucumr.pocoo.org/2022/1/7/as-any-hack/

trait AnyStorage: StorageWithEntities + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_storage(&self) -> &dyn StorageWithEntities;
    fn as_storage_mut(&mut self) -> &mut dyn StorageWithEntities;
}

impl<T: StorageWithEntities + Any> AnyStorage for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_storage(&self) -> &dyn StorageWithEntities {
        self
    }

    fn as_storage_mut(&mut self) -> &mut dyn StorageWithEntities {
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

    pub fn as_storage(&self) -> &dyn StorageWithEntities {
        (*self.0).as_storage()
    }

    pub fn as_storage_mut(&mut self) -> &mut dyn StorageWithEntities {
        (*self.0).as_storage_mut()
    }
}

pub struct StorageIdx<T: Any + StorageWithEntities> {
    idx: usize,
    phantom_data: PhantomData<T>,
}

// By storing the storages inside a `Vec`, storage insertion doesn't
// invalidate previous lookups, so lookups only have to be performed
// once.
#[derive(Default)]
pub(crate) struct ErasedStorages {
    lookup: HashMap<TypeId, usize>,
    storages: Vec<RefCell<ErasedStorage>>,
}

impl ErasedStorages {
    /// Panics if the new storage capacity exceeds `isize::MAX` bytes.
    pub fn insert<T: Any + StorageWithEntities>(&mut self, storage: T) -> Option<usize> {
        let type_id = TypeId::of::<T>();

        match self.lookup.entry(type_id) {
            Entry::Vacant(vacant) => {
                let idx = self.storages.len();
                let storage = RefCell::new(ErasedStorage::new(storage));

                self.storages.push(storage);
                vacant.insert(idx);

                Some(idx)
            }
            Entry::Occupied(_) => None,
        }
    }

    pub fn lookup<T: Any + StorageWithEntities>(&self) -> BorrowResult<StorageIdx<T>> {
        let type_id = TypeId::of::<T>();
        let idx = self
            .lookup
            .get(&type_id)
            .copied()
            .ok_or(BorrowError::StorageNotFound)?;
        Ok(StorageIdx {
            idx,
            phantom_data: PhantomData,
        })
    }

    pub fn lookup_or_insert<T: Any + StorageWithEntities + Default>(&mut self) -> StorageIdx<T> {
        let type_id = TypeId::of::<T>();
        let idx = match self.lookup.entry(type_id) {
            Entry::Vacant(vacant) => {
                let idx = self.storages.len();
                let storage = RefCell::new(ErasedStorage::new(T::default()));

                self.storages.push(storage);
                vacant.insert(idx);

                idx
            }
            Entry::Occupied(occupied) => *occupied.get(),
        };
        StorageIdx {
            idx,
            phantom_data: PhantomData,
        }
    }

    pub fn borrow_ref<T: Any + StorageWithEntities>(&self, idx: StorageIdx<T>) -> BorrowResult<Ref<T>> {
        let erased_storage_ref = self.storages[idx.idx]
            .try_borrow()
            .map_err(|_| BorrowError::InvalidBorrow)?;

        let storage = Ref::map(erased_storage_ref, |any| {
            any.as_any().downcast_ref().unwrap()
        });

        Ok(storage)
    }

    pub fn borrow_mut<T: Any + StorageWithEntities>(&self, idx: StorageIdx<T>) -> BorrowResult<RefMut<T>> {
        let erased_storage_mut = self.storages[idx.idx]
            .try_borrow_mut()
            .map_err(|_| BorrowError::InvalidBorrow)?;

        let storage = RefMut::map(erased_storage_mut, |any| {
            any.as_any_mut().downcast_mut().unwrap()
        });

        Ok(storage)
    }

    pub fn iter_refs(&self) -> ErasedStorageIter {
        ErasedStorageIter(self.storages.iter())
    }

    pub fn iter_muts(&mut self) -> ErasedStorageIterMut {
        ErasedStorageIterMut(self.storages.iter_mut())
    }
}

pub struct ErasedStorageIter<'a>(Iter<'a, RefCell<ErasedStorage>>);

impl<'a> Iterator for ErasedStorageIter<'a> {
    type Item = BorrowResult<Ref<'a, dyn StorageWithEntities>>;

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

pub struct ErasedStorageIterMut<'a>(IterMut<'a, RefCell<ErasedStorage>>);

impl<'a> Iterator for ErasedStorageIterMut<'a> {
    type Item = BorrowResult<RefMut<'a, dyn StorageWithEntities>>;

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
