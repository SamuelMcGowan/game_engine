use std::any::Any;
use std::cell::{Ref, RefMut};

use crate::storage::components::*;
use crate::storage::entities::{EntityId, EntityStorage, LiveEntity};
use crate::storage::erased::{ErasedStorage, StorageOccupied};
use crate::system::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowError {
    InvalidBorrow,
    StorageNotFound,
    ValueNotFound,
}

pub type BorrowResult<T> = Result<T, BorrowError>;

/// Central container for ECS data.
#[derive(Default)]
pub struct World {
    entities: EntityStorage,
    components: ErasedStorage,
    unique: ErasedStorage,
}

impl World {
    /// Spawn a new entity and create a builder for it.
    #[inline]
    pub fn spawn(&mut self) -> EntityBuilder {
        let entity = self.entities.spawn();

        EntityBuilder {
            world: self,
            entity,
        }
    }

    /// Get a builder for an entity.
    #[inline]
    pub fn entity(&mut self, entity: EntityId) -> EntityBuilder {
        EntityBuilder {
            world: self,
            entity,
        }
    }

    /// Get a query.
    /// 
    /// Panics upon failure.
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
        self.components.insert(ComponentStorage::<C>::default())
    }

    pub(crate) fn component_storage_ref<C: Component>(
        &self,
    ) -> BorrowResult<Ref<ComponentStorage<C>>> {
        self.components.borrow_ref()
    }

    pub(crate) fn component_storage_mut<C: Component>(
        &self,
    ) -> BorrowResult<RefMut<ComponentStorage<C>>> {
        self.components.borrow_mut()
    }

    pub(crate) fn entity_storage(&self) -> &EntityStorage {
        &self.entities
    }

    /// Insert a unique value.
    pub fn insert_unique<T: Any>(&mut self, unique: T) -> Result<(), StorageOccupied> {
        self.unique.insert(unique)
    }

    pub(crate) fn unique_ref<T: Any>(&self) -> BorrowResult<Ref<T>> {
        self.unique.borrow_ref()
    }

    pub(crate) fn unique_mut<T: Any>(&self) -> BorrowResult<RefMut<T>> {
        self.unique.borrow_mut()
    }

    /// Run a system.
    pub fn run<'a, S: System<'a, Params, Output>, Params, Output: SystemOutput>(
        &'a mut self,
        mut system: S,
    ) -> SystemResult<Output::Success, Output::Error> {
        system.run(self)
    }
}

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity: EntityId,
}

impl EntityBuilder<'_> {
    /// Add a component to the entity.
    pub fn insert<C: Component>(&mut self, component: C) -> &mut Self {
        let mut components = self
            .world
            .component_storage_mut::<C>()
            .expect("component type not registered");
        components.insert(self.live(), component);
        drop(components);
        self
    }

    /// Get the entity's id.
    #[inline]
    pub fn id(&self) -> EntityId {
        self.entity
    }

    fn live(&self) -> LiveEntity {
        self.world.entities.entity_to_alive(self.entity)
    }
}
