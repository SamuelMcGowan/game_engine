use crate::prelude::*;
use crate::storage::StorageWithEntities;

pub mod components;
pub mod erased;
pub mod uniques;

pub use erased::{ErasedStorageIter, ErasedStorageIterMut};

pub(crate) use components::AllComponentStorages;
pub(crate) use uniques::AllUniqueStorages;

#[derive(Default)]
pub struct AllStorages {
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
    /// Panics if the entity is dead or any storage is borrowed.
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
