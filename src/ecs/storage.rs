use std::any::{Any, TypeId};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use super::components::{Component, ComponentStorage};
use super::world::EntityId;
use super::EcsError;

pub trait Storage {
    type Item;

    fn get(&self, entity: EntityId) -> Option<&Self::Item>;
    fn get_mut(&mut self, entity: EntityId) -> Option<&mut Self::Item>;

    fn insert(&mut self, entity: EntityId, element: Self::Item) -> Option<Self::Item>;
    fn remove(&mut self, entity: EntityId) -> Option<Self::Item>;
}

#[derive(Default)]
pub struct AllStorages {
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl AllStorages {
    pub fn register_components<C: Component>(&mut self) -> Result<(), EcsError> {
        let type_id = TypeId::of::<C>();

        match self.components.entry(type_id) {
            Entry::Vacant(vacant) => {
                vacant.insert(Box::new(ComponentStorage::<C>::default()));
                Ok(())
            }
            Entry::Occupied(_) => Err(EcsError::StorageAlreadyAdded),
        }
    }

    pub fn components<C: Component>(&self) -> Option<&ComponentStorage<C>> {
        let type_id = TypeId::of::<C>();
        let erased_storage = self.components.get(&type_id)?;
        Some(erased_storage.downcast_ref().unwrap())
    }

    pub fn components_mut<C: Component>(&mut self) -> Option<&mut ComponentStorage<C>> {
        let type_id = TypeId::of::<C>();
        let erased_storage = self.components.get_mut(&type_id)?;
        Some(erased_storage.downcast_mut().unwrap())
    }

    pub fn component<C: Component>(&self, entity: EntityId) -> Option<&C> {
        self.components::<C>()?.get(entity)
    }

    pub fn component_mut<C: Component>(&mut self, entity: EntityId) -> Option<&mut C> {
        self.components_mut::<C>()?.get_mut(entity)
    }
}
