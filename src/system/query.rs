use std::cell::{Ref, RefMut};

use super::SystemParam;
use crate::storage::*;
use crate::world::{EntityId, World};

pub struct Query<'a, C: Component> {
    storage: Ref<'a, ComponentStorage<C>>,
}

pub struct QueryMut<'a, C: Component> {
    storage: RefMut<'a, ComponentStorage<C>>,
}

impl<'a, C: Component> Query<'a, C> {
    pub fn get(&self, entity: EntityId) -> Option<&C> {
        self.storage.get(entity)
    }

    pub fn iter(&self) -> Iter<C> {
        self.storage.iter()
    }
}

impl<'a, C: Component> QueryMut<'a, C> {
    pub fn get(&self, entity: EntityId) -> Option<&C> {
        self.storage.get(entity)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut C> {
        self.storage.get_mut(entity)
    }

    pub fn iter(&self) -> Iter<C> {
        self.storage.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<C> {
        self.storage.iter_mut()
    }
}

impl<'a, C: Component> SystemParam<'a> for Query<'a, C> {
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.component_storage()?,
        })
    }
}

impl<'a, C: Component> SystemParam<'a> for QueryMut<'a, C> {
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.component_storage_mut()?,
        })
    }
}
