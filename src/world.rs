use std::any::Any;
use std::cell::{Ref, RefMut};

use crate::system::{System, SystemResult, SystemOutput};

pub use crate::storage::components::*;
use crate::storage::erased::{ErasedStorage, StorageOccupied};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowError {
    InvalidBorrow,
    StorageNotFound,
    ValueNotFound,
}

pub type BorrowResult<T> = Result<T, BorrowError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityId(pub(crate) usize);

#[derive(Default)]
pub struct World {
    entity_count: usize,
    components: ErasedStorage,
    unique: ErasedStorage,
}

impl World {
    /// Spawn a new entity and create a builder for it.
    #[inline]
    pub fn spawn(&mut self) -> EntityBuilder {
        let id = self.entity_count;
        self.entity_count += 1;

        EntityBuilder {
            world: self,
            entity: EntityId(id),
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

    pub fn register_components<C: Component>(&mut self) -> Result<(), StorageOccupied> {
        self.components.insert(ComponentStorage::<C>::default())
    }

    pub fn component_storage_ref<C: Component>(&self) -> BorrowResult<Ref<ComponentStorage<C>>> {
        self.components.borrow_ref()
    }

    pub fn component_storage_mut<C: Component>(&self) -> BorrowResult<RefMut<ComponentStorage<C>>> {
        self.components.borrow_mut()
    }

    pub fn component_ref<C: Component>(&self, entity: EntityId) -> BorrowResult<Ref<C>> {
        let storage = self.component_storage_ref::<C>()?;
        Ref::filter_map(storage, |storage| storage.get(entity))
            .map_err(|_| BorrowError::ValueNotFound)
    }

    pub fn component_mut<C: Component>(&self, entity: EntityId) -> BorrowResult<RefMut<C>> {
        let storage = self.component_storage_mut::<C>()?;
        RefMut::filter_map(storage, |storage| storage.get_mut(entity))
            .map_err(|_| BorrowError::ValueNotFound)
    }

    pub fn insert_unique<T: Any>(&mut self, unique: T) -> Result<(), StorageOccupied> {
        self.unique.insert(unique)
    }

    pub fn unique_ref<T: Any>(&self) -> BorrowResult<Ref<T>> {
        self.unique.borrow_ref()
    }

    pub fn unique_mut<T: Any>(&self) -> BorrowResult<RefMut<T>> {
        self.unique.borrow_mut()
    }

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
    pub fn with<C: Component>(&mut self, component: C) -> &mut Self {
        let mut components = self
            .world
            .component_storage_mut::<C>()
            .expect("component type not registered");
        components.insert(self.entity, component);
        drop(components);
        self
    }

    /// Get the entity's id.
    #[inline]
    pub fn id(&self) -> EntityId {
        self.entity
    }
}