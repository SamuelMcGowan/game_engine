use crate::storage::erased::{BorrowError, BorrowResult};
use crate::world::World;

pub trait System<'a, Params, Output: SystemOutput> {
    fn run(&mut self, world: &'a World) -> SystemResult<Output::Success, Output::Error>;
}

pub trait SystemParam<'a>: Sized {
    fn borrow(world: &'a World) -> BorrowResult<Self>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemError<Error> {
    BorrowError(BorrowError),
    ExecutionError(Error),
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
        impl<'a, Func, Output, $($param: SystemParam<'a>),*>
        System<'a, ($($param,)*), Output>
        for Func
        where Func: FnMut($($param),*) -> Output, Output: SystemOutput
        {
            #[allow(unused_variables, non_snake_case)]
            fn run(&mut self, world: &'a World) -> SystemResult<Output::Success, Output::Error> {
                $(let $param = match $param::borrow(world) {
                    Ok(param) => param,
                    Err(err) => return Err(SystemError::BorrowError(err)),
                };)*
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
