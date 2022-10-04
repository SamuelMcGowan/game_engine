use super::sparse::SparseSet;
use super::storage::Storage;
use super::world::EntityId;

pub trait Component: 'static {}

pub struct ComponentStorage<C: Component>(SparseSet<C>);

impl<C: Component> Default for ComponentStorage<C> {
    fn default() -> Self {
        ComponentStorage(SparseSet::default())
    }
}

impl<C: Component> Storage for ComponentStorage<C> {
    type Item = C;
    
    fn get(&self, entity: EntityId) -> Option<&C> {
        self.0.get(entity.0)
    }

    fn get_mut(&mut self, entity: EntityId) -> Option<&mut C> {
        self.0.get_mut(entity.0)
    }

    fn insert(&mut self, entity: EntityId, element: C) -> Option<C> {
        self.0.insert(entity.0, element)
    }

    fn remove(&mut self, entity: EntityId) -> Option<C> {
        self.0.remove(entity.0)
    }
}