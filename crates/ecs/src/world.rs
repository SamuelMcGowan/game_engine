use std::any::Any;

use crate::all_storages::AllStorages;
use crate::prelude::*;

/// Central container for ECS data.
#[derive(Default)]
pub struct World {
    pub(crate) all_storages: AllStorages,
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
    pub fn insert_unique<T: Any>(&mut self, unique: T) -> Option<()> {
        self.all_storages.uniques.insert(unique)
    }

    /// Get a query.
    ///
    /// Panics upon failure.
    #[inline]
    pub fn get<'a, P: Query<'a>>(&'a mut self) -> P {
        P::lookup_and_borrow(self).unwrap_or_else(|err| {
            panic!("borrow error: {err:?}");
        })
    }

    /// Try to get a query.
    pub fn try_get<'a, P: Query<'a>>(&'a mut self) -> BorrowResult<P> {
        P::lookup_and_borrow(self)
    }

    /// Run a system.
    pub fn run<'a, S: System<'a, Params, Output>, Params, Output: SystemOutput>(
        &'a mut self,
        mut system: S,
    ) -> SystemResult<Output::Success, Output::Error> {
        system.run(self)
    }
}
