use super::sparse_set::SparseSet;
pub use super::sparse_set::{Iter, IterMut};
use crate::world::EntityId;

pub trait Component: 'static {}

pub struct ComponentStorage<C: Component>(SparseSet<C>);

impl<C: Component> Default for ComponentStorage<C> {
    fn default() -> Self {
        ComponentStorage(SparseSet::default())
    }
}

impl<C: Component> ComponentStorage<C> {
    pub fn get(&self, entity: EntityId) -> Option<&C> {
        self.0.get(entity.0)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut C> {
        self.0.get_mut(entity.0)
    }

    pub fn insert(&mut self, entity: EntityId, element: C) -> Option<C> {
        self.0.insert(entity.0, element)
    }

    pub fn remove(&mut self, entity: EntityId) -> Option<C> {
        self.0.remove(entity.0)
    }

    pub fn iter(&self) -> Iter<C> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<C> {
        self.0.iter_mut()
    }

    pub fn contains(&self, entity: EntityId) -> bool {
        self.0.contains(entity.0)
    }
}
