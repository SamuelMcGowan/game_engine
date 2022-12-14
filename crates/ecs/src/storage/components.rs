use super::sparse_set::SparseSet;
use super::StorageWithEntities;
use crate::prelude::*;

pub trait Component: 'static {}

pub struct ComponentStorage<C: Component>(SparseSet<C>);

impl<C: Component> Default for ComponentStorage<C> {
    fn default() -> Self {
        ComponentStorage(SparseSet::default())
    }
}

impl<C: Component> ComponentStorage<C> {
    #[inline]
    pub fn get(&self, entity: &LiveEntity) -> Option<&C> {
        self.0.get(entity.index())
    }

    #[inline]
    pub fn get_mut(&mut self, entity: &LiveEntity) -> Option<&mut C> {
        self.0.get_mut(entity.index())
    }

    #[inline]
    pub fn insert(&mut self, entity: &LiveEntity, element: C) -> Option<C> {
        self.0.insert(entity.index(), element)
    }

    #[inline]
    pub fn remove(&mut self, entity: &LiveEntity) -> Option<C> {
        self.0.remove(entity.index())
    }

    #[inline]
    pub fn iter(&self) -> Iter<C> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<C> {
        self.0.iter_mut()
    }

    #[inline]
    pub fn contains(&self, entity: LiveEntity) -> bool {
        self.0.contains(entity.index())
    }
}

impl<C: Component> StorageWithEntities for ComponentStorage<C> {
    fn remove_entity(&mut self, entity: &LiveEntity) {
        self.remove(entity);
    }
}
