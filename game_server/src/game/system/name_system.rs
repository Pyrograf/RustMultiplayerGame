use crate::game::entity::component::{NameComponent, PositionComponent};
use crate::game::entity::EntityId;
use crate::game::math::Vec2F;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum NameSystemError {
    #[error("Component already added")]
    ComponentAlreadyAdded(NameComponent)
}

pub type NameSystemResult<T> = Result<T, NameSystemError>;

pub struct NameSystem {
    components: HashMap<EntityId, NameComponent>,
}

impl NameSystem {
    pub fn new() -> Self {
        NameSystem {
            components: HashMap::new(),
        }
    }

    pub fn get_component(&self, entity: &EntityId) -> Option<&NameComponent> {
        self.components.get(entity)
    }

    pub fn get_name(&self, entity: &EntityId) -> Option<&str> {
        self.components.get(entity).map(|nc| nc.get_name())
    }

    pub fn add_component(&mut self, entity: EntityId, component: NameComponent) -> NameSystemResult<()> {
        if self.components.contains_key(&entity) {
            return Err(NameSystemError::ComponentAlreadyAdded(component))
        } else {
            self.components.insert(entity, component);
            Ok(())
        }
    }
}
