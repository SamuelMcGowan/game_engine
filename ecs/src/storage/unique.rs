use std::any::Any;

use super::entities::LiveEntity;
use super::Storage;

pub struct UniqueStorage<T: Any>(pub T);

impl<T: Any> Storage for UniqueStorage<T> {
    fn remove_entity(&mut self, _entity: &LiveEntity) {}
}
