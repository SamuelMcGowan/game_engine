use std::ops::{Deref, DerefMut};

use super::components::Component;
use super::storage::{AllStorages, Storage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityId(pub(super) usize);

#[derive(Default)]
pub struct World {
    entity_count: usize,
    storage_lookup: AllStorages,
}

impl World {
    pub fn spawn(&mut self) -> EntityBuilder {
        let id = self.entity_count;
        self.entity_count += 1;

        EntityBuilder {
            world: self,
            entity: EntityId(id),
        }
    }
}

impl Deref for World {
    type Target = AllStorages;

    fn deref(&self) -> &Self::Target {
        &self.storage_lookup
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage_lookup
    }
}

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity: EntityId,
}

impl EntityBuilder<'_> {
    pub fn with<C: Component>(&mut self, component: C) -> &mut Self {
        let components = self.world.components_mut::<C>().expect("component type not registered");
        components.insert(self.entity, component);
        self
    }

    pub fn id(&self) -> EntityId {
        self.entity
    }
}
