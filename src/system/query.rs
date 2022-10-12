use std::cell::{Ref, RefMut};

use crate::storage::components::*;
use crate::storage::entities::{EntityId, EntityStorage};
use crate::system::*;

pub struct Query<'a, C: Component> {
    storage: Ref<'a, ComponentStorage<C>>,
    entities: &'a EntityStorage,
}

pub struct QueryMut<'a, C: Component> {
    storage: RefMut<'a, ComponentStorage<C>>,
    entities: &'a EntityStorage,
}

impl<'a, C: Component> Query<'a, C> {
    #[inline]
    pub fn get(&self, entity: EntityId) -> Option<&C> {
        self.storage.get(self.entities.entity_to_alive(entity))
    }

    #[inline]
    pub fn iter(&self) -> Iter<C> {
        self.storage.iter()
    }

    #[inline]
    pub fn contains(&self, entity: EntityId) -> bool {
        self.storage.contains(self.entities.entity_to_alive(entity))
    }
}

impl<'a, C: Component> QueryMut<'a, C> {
    #[inline]
    pub fn get(&self, entity: EntityId) -> Option<&C> {
        self.storage.get(self.entities.entity_to_alive(entity))
    }

    #[inline]
    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut C> {
        self.storage.get_mut(self.entities.entity_to_alive(entity))
    }

    #[inline]
    pub fn insert(&mut self, entity: EntityId, element: C) -> Option<C> {
        self.storage
            .insert(self.entities.entity_to_alive(entity), element)
    }

    #[inline]
    pub fn remove(&mut self, entity: EntityId) -> Option<C> {
        self.storage.remove(self.entities.entity_to_alive(entity))
    }

    #[inline]
    pub fn iter(&self) -> Iter<C> {
        self.storage.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<C> {
        self.storage.iter_mut()
    }

    #[inline]
    pub fn contains(&self, entity: EntityId) -> bool {
        self.storage.contains(self.entities.entity_to_alive(entity))
    }
}

impl<'a, C: Component> SystemParam<'a> for Query<'a, C> {
    #[inline]
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.component_storage_ref()?,
            entities: world.entity_storage(),
        })
    }
}

impl<'a, C: Component> SystemParam<'a> for QueryMut<'a, C> {
    #[inline]
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.component_storage_mut()?,
            entities: world.entity_storage(),
        })
    }
}
