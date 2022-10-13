use std::cell::RefMut;

use crate::prelude::*;
use crate::storage::all_storages::AllStorages;

/// A handle to mutate an entity.
pub struct EntityMut<'a> {
    all_storages: &'a mut AllStorages,
    entity: EntityId,
}

impl<'a> EntityMut<'a> {
    pub(crate) fn new(entity: EntityId, all_storages: &'a mut AllStorages) -> Self {
        Self {
            entity,
            all_storages,
        }
    }

    /// Despawn an entity.
    ///
    /// Panics if the entity is dear or any storage is borrowed.
    pub fn despawn(self) {
        self.all_storages.despawn_entity(self.entity);
    }

    /// Add a component to the entity.
    ///
    /// Panics if the component type is not registered.
    pub fn insert<C: Component>(&mut self, component: C) -> &mut Self {
        let mut components = self.components_mut::<C>();
        components.insert(&self.live(), component);
        drop(components);
        self
    }

    /// Remove a component from an entity.
    ///
    /// Panics if the component type is not registered.
    pub fn remove<C: Component>(&mut self) -> &mut Self {
        let mut components = self.components_mut::<C>();
        components.remove(&self.live());
        drop(components);
        self
    }

    /// Get the entity's id.
    #[inline]
    pub fn id(&self) -> EntityId {
        self.entity
    }

    /// Panics if the component type is not registered.
    fn components_mut<C: Component>(&self) -> RefMut<ComponentStorage<C>> {
        self.all_storages
            .component_storage_mut::<C>()
            .expect("component type not registered")
    }

    fn live(&self) -> LiveEntity {
        self.all_storages
            .entity_storage()
            .entity_to_alive(self.entity)
    }
}
