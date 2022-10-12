use crate::storage::sparse_set::SparseSet;

// THANKS TO: https://skypjack.github.io/2019-05-06-ecs-baf-part-3/

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct EntityId {
    entity: u16,
    version: u16,
}

// This is to avoid references to EntityId
// fields (because it's a packed struct).
impl EntityId {
    pub fn entity(self) -> u16 {
        self.entity
    }

    pub fn version(self) -> u16 {
        self.version
    }
}

impl std::fmt::Debug for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}v{}", self.entity(), self.version())
    }
}

#[derive(Default)]
pub struct EntityStorage {
    next_recycled: u16,
    num_recycled: usize,

    next: u16,

    storage: SparseSet<EntityId>,
}

impl EntityStorage {
    /// Spawn a new entity.
    ///
    /// Panics if out of entities.
    pub fn spawn(&mut self) -> EntityId {
        if self.num_recycled > 0 {
            // There is an entity waiting for us to use.

            let stored = self.storage.get_mut(self.next_recycled as usize).unwrap();

            // Remove it from the implicit linked list.
            std::mem::swap(&mut self.next_recycled, &mut stored.entity());
            self.num_recycled -= 1;

            *stored
        } else {
            // There are no entities we can reuse, so we need to
            // assign a new one.

            if self.next == u16::MAX {
                panic!("out of entities.");
            }

            let entity = self.next;
            self.next += 1;

            let entity = EntityId { entity, version: 0 };

            self.storage.insert(entity.entity as usize, entity);

            entity
        }
    }

    /// Despawn an entity.
    ///
    /// Panics if the entity isn't in this storage, or this entity was already
    /// despawned.
    pub fn despawn(&mut self, entity: EntityId) {
        let stored = self.storage.get_mut(entity.entity as usize).unwrap();

        // Make sure this isn't a dead entity.
        if stored.version != entity.version {
            panic!("tried to despawn entity {entity:?} twice");
        }

        // Increment the version.
        // Version will not be greater than `u16::MAX` - 1, so it won't wrap.
        stored.version += 1;

        // Recycle this id if possible by adding it to the implicit linked list.
        // It can't be reused if its new version is `u16::MAX`, because its
        // version wouldn't be incrementable when it was despawned.
        if stored.version < u16::MAX {
            std::mem::swap(&mut self.next_recycled, &mut stored.entity());
            self.num_recycled += 1;
        }
    }

    /// Check if an entity is alive.
    ///
    /// Panics if the entity isn't in this storage.
    pub fn is_alive(&self, entity: EntityId) -> bool {
        self.storage.get(entity.entity as usize).unwrap().version == entity.version
    }

    /// Try to convert this to a live entity.
    pub fn get_alive(&self, entity: EntityId) -> Option<LiveEntity> {
        if self.is_alive(entity) {
            Some(LiveEntity {
                entity,
                storage: self,
            })
        } else {
            None
        }
    }
}

/// An alive entity.
pub struct LiveEntity<'a> {
    entity: EntityId,
    storage: &'a EntityStorage,
}

impl<'a> LiveEntity<'a> {
    pub(super) fn index(&self) -> usize {
        self.entity.entity() as usize
    }
}
