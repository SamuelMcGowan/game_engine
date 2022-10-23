use std::cell::RefMut;

use crate::all_storages::{AllComponentStorages, AllStorages};
use crate::prelude::*;

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
    pub fn insert<C: Component>(self, component: C) -> Self {
        let mut components = lookup_or_insert_and_borrow_mut(&mut self.all_storages.components);
        let entity = self.all_storages.entities.entity_to_alive(self.entity);

        components.insert(&entity, component);

        drop(components);

        self
    }

    /// Remove a component from an entity.
    ///
    /// Panics if the component type is not registered.
    pub fn remove<C: Component>(self) -> Self {
        if let Some(mut components) = lookup_and_borrow_mut::<C>(&mut self.all_storages.components)
        {
            let entity = self.all_storages.entities.entity_to_alive(self.entity);
            components.remove(&entity);
        }
        self
    }

    /// Get the entity's id.
    #[inline]
    pub fn id(&self) -> EntityId {
        self.entity
    }
}

fn lookup_and_borrow_mut<C: Component>(
    all_components: &mut AllComponentStorages,
) -> Option<RefMut<ComponentStorage<C>>> {
    let idx = all_components.lookup().ok()?;
    let components = all_components
        .borrow_mut(idx)
        .expect("component type not registered");
    Some(components)
}

fn lookup_or_insert_and_borrow_mut<C: Component>(
    all_components: &mut AllComponentStorages,
) -> RefMut<ComponentStorage<C>> {
    let idx = all_components.lookup_or_insert();
    all_components
        .borrow_mut(idx)
        .expect("component type not registered")
}
