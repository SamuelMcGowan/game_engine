use std::any::Any;
use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

use crate::system::*;

pub struct Unique<'a, T: Any> {
    storage: Ref<'a, T>,
}

pub struct UniqueMut<'a, T: Any> {
    storage: RefMut<'a, T>,
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
