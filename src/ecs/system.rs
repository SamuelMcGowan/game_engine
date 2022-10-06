use std::cell::{Ref, RefMut};

use super::components::{Component, ComponentStorage};
use super::sparse_set::{Iter, IterMut};
use super::storage::{BorrowError, BorrowResult};
use super::world::{EntityId, World};

#[derive(Debug)]
pub enum SystemError<E> {
    BorrowError(BorrowError),
    ExecutionError(E),
}

impl<T> From<BorrowError> for SystemError<T> {
    fn from(err: BorrowError) -> Self {
        Self::BorrowError(err)
    }
}

pub trait System<'a, T> {
    type Error;

    fn run(&mut self, world: &'a World) -> Result<(), SystemError<Self::Error>>;
}

pub trait SystemParam<'a>: Sized {
    fn borrow<'b: 'a>(world: &'b World) -> BorrowResult<Self>;
}

pub struct Query<'a, C: Component> {
    storage: Ref<'a, ComponentStorage<C>>,
}

impl<'a, C: Component> SystemParam<'a> for Query<'a, C> {
    fn borrow<'b: 'a>(world: &'b World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.component_storage()?,
        })
    }
}

impl<'a, C: Component> Query<'a, C> {
    pub fn get(&self, entity: EntityId) -> Option<&C> {
        self.storage.get(entity)
    }

    pub fn iter(&self) -> Iter<C> {
        self.storage.iter()
    }
}

pub struct QueryMut<'a, C: Component> {
    storage: RefMut<'a, ComponentStorage<C>>,
}

impl<'a, C: Component> SystemParam<'a> for QueryMut<'a, C> {
    fn borrow<'b: 'a>(world: &'b World) -> BorrowResult<Self> {
        Ok(Self {
            storage: world.component_storage_mut()?,
        })
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

macro_rules! impl_system {
    ($($P:ident $p:ident),*) => {
        impl<'a, F, $($P: SystemParam<'a>),*>
        System<'a, ($($P,)*)> for F
        where F: FnMut($($P),*) {
            type Error = ();

            #[allow(unused_variables)]
            fn run(&mut self, world: &'a World) -> Result<(), SystemError<Self::Error>> {
                $(let $p = $P::borrow(world)?;)*
                (self)($($p),*);
                Ok(())
            }
        }
    };
}

impl_system!();
impl_system!(Q0 q0);
impl_system!(Q0 q0, Q1 q1);
