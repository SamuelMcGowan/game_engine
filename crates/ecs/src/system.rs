use paste::paste;

use crate::prelude::*;

pub trait System<'a, Params, Output: SystemOutput> {
    fn run(&mut self, world: &'a mut World) -> SystemResult<Output::Success, Output::Error>;
}

pub trait Query<'a>: Sized {
    type Index;

    fn lookup(world: &mut World) -> Self::Index;
    fn borrow(world: &'a World, idx: Self::Index) -> BorrowResult<Self>;

    fn lookup_and_borrow(world: &'a mut World) -> BorrowResult<Self> {
        let idx = Self::lookup(world);
        Self::borrow(world, idx)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemError<Error> {
    BorrowError(BorrowError),
    ExecutionError(Error),
}

impl<Error> From<BorrowError> for SystemError<Error> {
    fn from(err: BorrowError) -> Self {
        Self::BorrowError(err)
    }
}

pub type SystemResult<T, Error> = Result<T, SystemError<Error>>;

pub trait SystemOutput {
    type Success;
    type Error;

    fn to_result(self) -> SystemResult<Self::Success, Self::Error>;
}

impl SystemOutput for () {
    type Success = ();
    type Error = ();

    fn to_result(self) -> SystemResult<Self::Success, Self::Error> {
        Ok(())
    }
}

impl<S, E> SystemOutput for Result<S, E> {
    type Success = S;
    type Error = E;

    #[inline]
    fn to_result(self) -> SystemResult<Self::Success, Self::Error> {
        self.map_err(SystemError::ExecutionError)
    }
}

macro_rules! impl_system {
    ($($param:ident),*) => {
        impl<'a, Func, Output, $($param: Query<'a>),*>
        System<'a, ($($param,)*), Output>
        for Func
        where Func: FnMut($($param),*) -> Output, Output: SystemOutput
        {
            #[allow(unused_variables, non_snake_case)]
            fn run(&mut self, world: &'a mut World) -> SystemResult<Output::Success, Output::Error> {
                $(let paste!([<$param _idx>]) = $param::lookup(world);)*
                $(let $param = $param::borrow(world, paste!([<$param _idx>]))?;)*
                (self)($($param,)*).to_result()
            }
        }
    };
}

impl_system!();
impl_system!(Q0);
impl_system!(Q0, Q1);
impl_system!(Q0, Q1, Q2);
impl_system!(Q0, Q1, Q2, Q3);
impl_system!(Q0, Q1, Q2, Q3, Q4);
impl_system!(Q0, Q1, Q2, Q3, Q4, Q5);
impl_system!(Q0, Q1, Q2, Q3, Q4, Q5, Q6);
impl_system!(Q0, Q1, Q2, Q3, Q4, Q5, Q6, Q7);
