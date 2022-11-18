use std::any::Any;

use super::entities::LiveEntity;
use super::StorageWithEntities;

pub struct UniqueStorage<T: Any>(pub T);

impl<T: Any> StorageWithEntities for UniqueStorage<T> {
    fn remove_entity(&mut self, _entity: &LiveEntity) {}
}
