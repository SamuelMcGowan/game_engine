use std::any::Any;
use std::cell::{Ref, RefMut};

use super::erased::{ErasedStorageIter, ErasedStorageIterMut, ErasedStorages};
use super::unique::UniqueStorage;
use crate::prelude::*;

#[derive(Default)]
pub(crate) struct AllComponentStorages(ErasedStorages);

impl AllComponentStorages {
    #[inline]
    pub fn insert_storage<C: Component>(&mut self) -> Option<()> {
        self.0.insert(ComponentStorage::<C>::default())
    }

    #[inline]
    pub fn borrow_ref<C: Component>(&self) -> BorrowResult<Ref<ComponentStorage<C>>> {
        self.0.borrow_ref()
    }

    #[inline]
    pub fn borrow_mut<C: Component>(&self) -> BorrowResult<RefMut<ComponentStorage<C>>> {
        self.0.borrow_mut()
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
        self.0.insert(UniqueStorage(unique))
    }

    #[inline]
    pub fn borrow_ref<T: Any>(&self) -> BorrowResult<Ref<UniqueStorage<T>>> {
        self.0.borrow_ref()
    }

    #[inline]
    pub fn borrow_mut<T: Any>(&self) -> BorrowResult<RefMut<UniqueStorage<T>>> {
        self.0.borrow_mut()
    }
}

#[derive(Default)]
pub(crate) struct AllStorages {
    entities: EntityStorage,
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

    #[inline]
    pub fn entity_storage(&self) -> &EntityStorage {
        &self.entities
    }
}
