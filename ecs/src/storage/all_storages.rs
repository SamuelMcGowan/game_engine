use std::any::Any;
use std::cell::{Ref, RefMut};

use super::erased::{ErasedStorageIter, ErasedStorageIterMut, ErasedStorages, StorageIdx};
use super::unique::UniqueStorage;
use crate::prelude::*;

#[derive(Default)]
pub(crate) struct AllComponentStorages(ErasedStorages);

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

#[derive(Default)]
pub(crate) struct AllUniqueStorages(ErasedStorages);

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

#[derive(Default)]
pub(crate) struct AllStorages {
    pub(crate) entities: EntityStorage,
    pub(crate) components: AllComponentStorages,
    pub(crate) uniques: AllUniqueStorages,
}

impl AllStorages {
    #[inline]
    pub fn spawn(&mut self) -> EntityMut {
        let entity = self.entities.spawn();
        EntityMut::new(entity, self)
    }

    /// Despawn an entity.
    ///
    /// Panics if the entity is dear or any storage is borrowed.
    pub(crate) fn despawn_entity(&mut self, entity: EntityId) {
        let live_entity = self.entities.entity_to_alive(entity);

        for storage in self.components.iter_muts() {
            let mut storage = storage.expect("couldn't borrow storage");
            storage.remove_entity(&live_entity);
        }

        self.entities.despawn(entity);
    }

    #[inline]
    pub fn entity(&mut self, entity: EntityId) -> EntityMut {
        if !self.entities.is_alive(entity) {
            panic!("entity {entity:?} is dead");
        }

        EntityMut::new(entity, self)
    }
}
