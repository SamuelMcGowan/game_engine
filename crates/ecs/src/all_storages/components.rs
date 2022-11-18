use std::cell::{Ref, RefMut};

use super::erased::*;
use crate::prelude::*;

impl<C: Component> ErasableStorage for ComponentStorage<C> {
    type ErasedStorage = ErasedStorageWithEntities;

    fn erase(self) -> Self::ErasedStorage {
        ErasedStorageWithEntities::new(self)
    }

    fn downcast_ref(erased: &Self::ErasedStorage) -> Option<&Self> {
        erased.downcast_ref()
    }

    fn downcast_mut(erased: &mut Self::ErasedStorage) -> Option<&mut Self> {
        erased.downcast_mut()
    }
}

#[derive(Default)]
pub struct AllComponentStorages(ErasedStorages<ErasedStorageWithEntities>);

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
    pub fn iter_refs(&self) -> ErasedStorageIter<ErasedStorageWithEntities> {
        self.0.iter_refs()
    }

    #[inline]
    pub fn iter_muts(&mut self) -> ErasedStorageIterMut<ErasedStorageWithEntities> {
        self.0.iter_muts()
    }
}
