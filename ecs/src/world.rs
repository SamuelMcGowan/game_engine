use std::any::Any;

use crate::storage::all_storages::{AllStorages, BorrowResult, EntityMut};
use crate::storage::components::*;
use crate::storage::entities::EntityId;
use crate::storage::erased::StorageOccupied;
use crate::system::*;

/// Central container for ECS data.
#[derive(Default)]
pub struct World {
    all_storages: AllStorages,
}

impl World {
    /// Spawn a new entity and create a handle for it.
    #[inline]
    pub fn spawn(&mut self) -> EntityMut {
        self.all_storages.spawn()
    }

    /// Get a handle for an entity.
    #[inline]
    pub fn entity(&mut self, entity: EntityId) -> EntityMut {
        self.all_storages.entity(entity)
    }

    #[inline]
    pub fn insert_unique<T: Any>(&mut self, unique: T) -> Result<(), StorageOccupied> {
        self.all_storages.insert_unique(unique)
    }

    pub(crate) fn all_storages(&self) -> &AllStorages {
        &self.all_storages
    }

    /// Get a query.
    ///
    /// Panics upon failure.
    #[inline]
    pub fn get<'a, P: SystemParam<'a>>(&'a self) -> P {
        P::borrow(self).unwrap_or_else(|err| {
            panic!("borrow error: {err:?}");
        })
    }

    /// Try to get a query.
    pub fn try_get<'a, P: SystemParam<'a>>(&'a self) -> BorrowResult<P> {
        P::borrow(self)
    }

    /// Register component type.
    pub fn register_components<C: Component>(&mut self) -> Result<(), StorageOccupied> {
        self.all_storages.register_components::<C>()
    }

    /// Run a system.
    pub fn run<'a, S: System<'a, Params, Output>, Params, Output: SystemOutput>(
        &'a mut self,
        mut system: S,
    ) -> SystemResult<Output::Success, Output::Error> {
        system.run(self)
    }
}
