use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::slice::Iter;

use crate::prelude::{BorrowError, BorrowResult};

pub(crate) trait ErasableStorage: Any + Sized {
    type ErasedStorage;

    fn from_erased_ref(erased: &Self::ErasedStorage) -> &Self;
    fn from_erased_mut(erased: &mut Self::ErasedStorage) -> &mut Self;

    fn to_erased(self) -> Self::ErasedStorage;
}

pub(crate) struct StorageIdx<S: ErasableStorage> {
    idx: usize,
    phantom_data: PhantomData<S>,
}

// By storing the storages inside a `Vec`, storage insertion doesn't
// invalidate previous lookups, so lookups only have to be performed
// once.
pub(crate) struct ErasedStorages<ErasedStorage> {
    lookup: HashMap<TypeId, usize>,
    storages: Vec<RefCell<ErasedStorage>>,
}

impl<ErasedStorage> Default for ErasedStorages<ErasedStorage> {
    fn default() -> Self {
        Self {
            lookup: HashMap::new(),
            storages: vec![],
        }
    }
}

impl<ErasedStorage> ErasedStorages<ErasedStorage> {
    /// Panics if the new storage capacity exceeds `isize::MAX` bytes.
    pub fn insert<S: ErasableStorage<ErasedStorage = ErasedStorage>>(
        &mut self,
        storage: S,
    ) -> Option<usize> {
        let type_id = storage.type_id();

        match self.lookup.entry(type_id) {
            Entry::Vacant(vacant) => {
                let idx = self.storages.len();
                let storage = RefCell::new(storage.to_erased());

                self.storages.push(storage);
                vacant.insert(idx);

                Some(idx)
            }
            Entry::Occupied(_) => None,
        }
    }

    pub fn lookup<S: ErasableStorage<ErasedStorage = ErasedStorage>>(
        &self,
    ) -> BorrowResult<StorageIdx<S>> {
        let type_id = TypeId::of::<S>();
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

    pub fn lookup_or_insert<S: ErasableStorage<ErasedStorage = ErasedStorage> + Default>(
        &mut self,
    ) -> StorageIdx<S> {
        let type_id = TypeId::of::<S>();
        let idx = match self.lookup.entry(type_id) {
            Entry::Vacant(vacant) => {
                let idx = self.storages.len();
                let storage = RefCell::new(S::default().to_erased());

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

    pub fn borrow_ref<S: ErasableStorage<ErasedStorage = ErasedStorage>>(
        &self,
        idx: StorageIdx<S>,
    ) -> BorrowResult<Ref<S>> {
        let erased_storage_ref = self.storages[idx.idx]
            .try_borrow()
            .map_err(|_| BorrowError::InvalidBorrow)?;

        let storage = Ref::map(erased_storage_ref, S::from_erased_ref);

        Ok(storage)
    }

    pub fn borrow_mut<S: ErasableStorage<ErasedStorage = ErasedStorage>>(
        &self,
        idx: StorageIdx<S>,
    ) -> BorrowResult<RefMut<S>> {
        let erased_storage_mut = self.storages[idx.idx]
            .try_borrow_mut()
            .map_err(|_| BorrowError::InvalidBorrow)?;

        let storage = RefMut::map(erased_storage_mut, S::from_erased_mut);

        Ok(storage)
    }

    pub fn iter_refs(&self) -> ErasedStorageIter<ErasedStorage> {
        ErasedStorageIter(self.storages.iter())
    }

    pub fn iter_muts(&self) -> ErasedStorageIterMut<ErasedStorage> {
        ErasedStorageIterMut(self.storages.iter())
    }
}

pub struct ErasedStorageIter<'a, ErasedStorage>(Iter<'a, RefCell<ErasedStorage>>);

impl<'a, S> Iterator for ErasedStorageIter<'a, S> {
    type Item = BorrowResult<Ref<'a, S>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|erased_storage| {
            erased_storage
                .try_borrow()
                .map_err(|_| BorrowError::InvalidBorrow)
        })
    }
}

pub struct ErasedStorageIterMut<'a, ErasedStorage>(Iter<'a, RefCell<ErasedStorage>>);

impl<'a, S> Iterator for ErasedStorageIterMut<'a, S> {
    type Item = BorrowResult<RefMut<'a, S>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|erased_storage| {
            erased_storage
                .try_borrow_mut()
                .map_err(|_| BorrowError::InvalidBorrow)
        })
    }
}
