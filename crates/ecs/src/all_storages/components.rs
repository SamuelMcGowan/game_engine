use std::cell::{Ref, RefMut};

use super::erased::{ErasedStorageIter, ErasedStorageIterMut, ErasedStorages, StorageIdx};
use crate::prelude::*;

#[derive(Default)]
pub struct AllComponentStorages(ErasedStorages);

impl AllComponentStorages {
    #[inline]
    pub fn lookup<C: Component>(&self) -> BorrowResult<StorageIdx<ComponentStorage<C>>> {
        self.0.lookup()
    }

    #[inline]
    pub fn lookup_or_insert<C: Component>(&mut self) -> StorageIdx<ComponentStorage<C>> {
        self.0.lookup_or_insert()
    }

    #[inline]
    pub fn borrow_ref<C: Component>(
        &self,
        idx: StorageIdx<ComponentStorage<C>>,
    ) -> BorrowResult<Ref<ComponentStorage<C>>> {
        self.0.borrow_ref(idx)
    }

    #[inline]
    pub fn borrow_mut<C: Component>(
        &self,
        idx: StorageIdx<ComponentStorage<C>>,
    ) -> BorrowResult<RefMut<ComponentStorage<C>>> {
        self.0.borrow_mut(idx)
    }

    #[inline]
    pub fn iter_refs(&self) -> ErasedStorageIter {
        self.0.iter_refs()
    }

    #[inline]
    pub fn iter_muts(&mut self) -> ErasedStorageIterMut {
        self.0.iter_muts()
    }
}
