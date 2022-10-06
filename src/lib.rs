pub mod storage;
pub mod system;
pub mod world;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum EcsError {
    StorageAlreadyAdded,
}
