use bevy_utils::Parallel;
use crate::component::ComponentId;
use crate::entity::Entity;

/// A Collection of EntityChange storages
/// Can be accessed via Storages
#[derive(Default)]
pub struct Changes {
    list: Parallel<Vec<EntityChange>>,
}

impl Changes {

    /// Returns a default `Changes`

    pub fn new() -> Self {
        Self::default()
    }

    /// Add a change record into `Changes`
    pub fn push(&self, change: EntityChange) {
        self.list.scope(|scope| {
            scope.push(change);
        })
    }

    /// Traverse all changes.
    /// One a Change has been accessed, it will be removed from `Changes`
    pub fn take_all(&mut self) -> Vec<EntityChange> {
        self.list.drain().collect()
    }
}

/// A Record hint which entity's component has changed
#[derive(Copy, Clone, Debug)]
pub struct EntityChange {
    entity:    Entity,
    component: ComponentId,
}

impl EntityChange {
    /// Return a new EntityChange
    pub fn new(entity: Entity, component: ComponentId) -> Self {
        EntityChange { entity, component }
    }
    /// Access change's entity
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Access change's component id
    pub fn component(&self) -> ComponentId {
        self.component
    }
}

mod tests {
    #![allow(unused_imports)]
    use crate::component::ComponentId;
    use crate::entity::Entity;
    use crate::storage::{Changes, EntityChange};

    #[test]
    fn changes() {
        let mut storage = Changes::new();
        let entity = Entity::PLACEHOLDER;
        let component = ComponentId::new(1);
        storage.push(EntityChange::new(entity, component));
        storage.push(EntityChange::new(entity, component));

        let changes = storage.take_all();
        assert_eq!(changes.len(), 2);
        assert_eq!(storage.take_all().len(), 0);

        changes.iter().for_each(|change| {
            assert_eq!(change.component, component);
            assert_eq!(change.entity, entity);
        })
    }
}