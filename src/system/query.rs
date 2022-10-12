use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

use crate::storage::components::*;
use crate::system::*;

pub struct Query<'a, C: Component> {
    storage: Ref<'a, ComponentStorage<C>>,
}

pub struct QueryMut<'a, C: Component> {
    storage: RefMut<'a, ComponentStorage<C>>,
}

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
