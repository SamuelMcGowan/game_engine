use std::any::Any;
use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

use super::SystemParam;
use crate::storage::*;
use crate::world::World;

// DEFINITIONS

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

// DEREF IMPLEMENTATIONS

impl<'a, C: Component> Deref for Query<'a, C> {
    type Target = ComponentStorage<C>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl<'a, C: Component> Deref for QueryMut<'a, C> {
    type Target = ComponentStorage<C>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl<'a, C: Component> DerefMut for QueryMut<'a, C> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}

impl<'a, T: Any> Deref for Unique<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.storage.deref()
    }
}

impl<'a, T: Any> Deref for UniqueMut<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.storage.deref()
    }
}

impl<'a, T: Any> DerefMut for UniqueMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.storage.deref_mut()
    }
}

// SYSTEMPARAM IMPLEMENTATIONS

impl<'a, C: Component> SystemParam<'a> for Query<'a, C> {
    #[inline]
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.component_storage_ref()?,
        })
    }
}

impl<'a, C: Component> SystemParam<'a> for QueryMut<'a, C> {
    #[inline]
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.component_storage_mut()?,
        })
    }
}

impl<'a, T: Any> SystemParam<'a> for Unique<'a, T> {
    #[inline]
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.unique_ref()?,
        })
    }
}

impl<'a, T: Any> SystemParam<'a> for UniqueMut<'a, T> {
    #[inline]
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.unique_mut()?,
        })
    }
}
