pub mod system;
mod sparse_set;

pub mod components;
pub mod storage;
pub mod world;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum EcsError {
    StorageAlreadyAdded,
}
