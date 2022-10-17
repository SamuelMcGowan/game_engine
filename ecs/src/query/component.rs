use std::cell::{Ref, RefMut};

use crate::prelude::*;

pub struct Comp<'a, C: Component> {
    storage: Ref<'a, ComponentStorage<C>>,
    entities: &'a EntityStorage,
}

pub struct CompMut<'a, C: Component> {
    storage: RefMut<'a, ComponentStorage<C>>,
    entities: &'a EntityStorage,
}

impl<'a, C: Component> Comp<'a, C> {
    #[inline]
    pub fn get(&self, entity: EntityId) -> Option<&C> {
        self.storage.get(&self.entities.entity_to_alive(entity))
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

impl<'a, C: Component> CompMut<'a, C> {
    #[inline]
    pub fn get(&self, entity: EntityId) -> Option<&C> {
        self.storage.get(&self.entities.entity_to_alive(entity))
    }

    #[inline]
    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut C> {
        self.storage.get_mut(&self.entities.entity_to_alive(entity))
    }

    #[inline]
    pub fn insert(&mut self, entity: EntityId, element: C) -> Option<C> {
        self.storage
            .insert(&self.entities.entity_to_alive(entity), element)
    }

    #[inline]
    pub fn remove(&mut self, entity: EntityId) -> Option<C> {
        self.storage.remove(&self.entities.entity_to_alive(entity))
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

impl<'a, C: Component> Query<'a> for Comp<'a, C> {
    #[inline]
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.all_storages.components.borrow_ref()?,
            entities: world.all_storages.entity_storage(),
        })
    }
}

impl<'a, C: Component> Query<'a> for CompMut<'a, C> {
    #[inline]
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.all_storages.components.borrow_mut()?,
            entities: world.all_storages.entity_storage(),
        })
    }
}
