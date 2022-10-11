use std::any::Any;
use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

use super::SystemParam;
use crate::storage::*;
use crate::world::{EntityId, World};

pub struct Query<'a, C: Component> {
    storage: Ref<'a, ComponentStorage<C>>,
}

pub struct QueryMut<'a, C: Component> {
    storage: RefMut<'a, ComponentStorage<C>>,
}

pub struct Unique<'a, T: Any> {
    storage: Ref<'a, T>,
}

pub struct UniqueMut<'a, T: Any> {
    storage: RefMut<'a, T>,
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

impl<'a, T: Any> Unique<'a, T> {
    pub fn get(&self) -> &T {
        self.storage.deref()
    }
}

impl<'a, T: Any> UniqueMut<'a, T> {
    pub fn get(&self) -> &T {
        self.storage.deref()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.storage.deref_mut()
    }
}

impl<'a, C: Component> SystemParam<'a> for Query<'a, C> {
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.component_storage_ref()?,
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

impl<'a, T: Any> SystemParam<'a> for Unique<'a, T> {
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.unique_ref()?,
        })
    }
}

impl<'a, T: Any> SystemParam<'a> for UniqueMut<'a, T> {
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.unique_mut()?,
        })
    }
}
