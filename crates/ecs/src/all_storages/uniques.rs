use std::any::Any;
use std::cell::{Ref, RefMut};

use super::erased::*;
use crate::prelude::*;
use crate::storage::unique::UniqueStorage;

impl<T: Any> ErasableStorage for UniqueStorage<T> {
    type ErasedStorage = Box<dyn Any>;

    fn erase(self) -> Self::ErasedStorage {
        Box::new(self)
    }

    fn downcast_ref(erased: &Self::ErasedStorage) -> Option<&Self> {
        erased.downcast_ref()
    }

    fn downcast_mut(erased: &mut Self::ErasedStorage) -> Option<&mut Self> {
        erased.downcast_mut()
    }
}

#[derive(Default)]
pub struct AllUniqueStorages(ErasedStorages<Box<dyn Any>>);

impl AllUniqueStorages {
    #[inline]
    pub fn insert<T: Any>(&mut self, unique: T) -> Option<()> {
        self.0.insert(UniqueStorage(unique)).map(|_| {})
    }

    #[inline]
    pub fn lookup<T: Any>(&self) -> BorrowResult<StorageIdx<UniqueStorage<T>>> {
        self.0.lookup()
    }

    #[inline]
    pub fn borrow_ref<T: Any>(
        &self,
        idx: StorageIdx<UniqueStorage<T>>,
    ) -> BorrowResult<Ref<UniqueStorage<T>>> {
        self.0.borrow_ref(idx)
    }

    #[inline]
    pub fn borrow_mut<T: Any>(
        &self,
        idx: StorageIdx<UniqueStorage<T>>,
    ) -> BorrowResult<RefMut<UniqueStorage<T>>> {
        self.0.borrow_mut(idx)
    }
}
