use std::ops::{Deref, DerefMut};

use super::storage::{AllStorages, Component};
use super::system::{System, SystemOutput, SystemResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityId(pub(super) usize);

#[derive(Default)]
pub struct World {
    entity_count: usize,
    all_storages: AllStorages,
}

impl World {
    /// Spawn a new entity and create a builder for it.
    pub fn spawn(&mut self) -> EntityBuilder {
        let id = self.entity_count;
        self.entity_count += 1;

        EntityBuilder {
            world: self,
            entity: EntityId(id),
        }
    }

    /// Get a builder for an entity.
    pub fn entity(&mut self, entity: EntityId) -> EntityBuilder {
        EntityBuilder {
            world: self,
            entity,
        }
    }

    pub fn run<'a, S: System<'a, Params, Output>, Params, Output: SystemOutput>(
        &'a mut self,
        mut system: S,
    ) -> SystemResult<Output::Success, Output::Error> {
        system.run(self)
    }
}

impl Deref for World {
    type Target = AllStorages;

    fn deref(&self) -> &Self::Target {
        &self.all_storages
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.all_storages
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
    pub fn id(&self) -> EntityId {
        self.entity
    }
}
