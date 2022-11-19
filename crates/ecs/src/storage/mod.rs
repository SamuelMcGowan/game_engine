pub mod components;
pub mod entities;
pub mod unique;

mod sparse_set;

use self::entities::LiveEntity;
pub use self::sparse_set::{Iter, IterMut};

pub trait StorageWithEntities: 'static {
    fn remove_entity(&mut self, entity: &LiveEntity);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowError {
    ResourceNotFound,
    StorageNotFound,
    InvalidBorrow,
}

pub type BorrowResult<T> = Result<T, BorrowError>;
