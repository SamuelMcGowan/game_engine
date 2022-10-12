use crate::storage::all_storages::BorrowResult;
use std::any::Any;
use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

use crate::system::*;
use crate::world::World;

pub struct Unique<'a, T: Any> {
    storage: Ref<'a, T>,
}

pub struct UniqueMut<'a, T: Any> {
    storage: RefMut<'a, T>,
}

impl<'a, T: Any> Unique<'a, T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.storage
    }
}

impl<'a, T: Any> UniqueMut<'a, T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.storage
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
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

impl<'a, T: Any> SystemParam<'a> for Unique<'a, T> {
    #[inline]
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.all_storages().unique_ref()?,
        })
    }
}

impl<'a, T: Any> SystemParam<'a> for UniqueMut<'a, T> {
    #[inline]
    fn borrow(world: &'a World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.all_storages().unique_mut()?,
        })
    }
}
