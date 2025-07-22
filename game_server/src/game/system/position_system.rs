use crate::game::entity::component::PositionComponent;
use crate::game::entity::EntityId;
use crate::game::math::Vec2F;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum PositionSystemError {
    #[error("Component already added")]
    ComponentAlreadyAdded(PositionComponent)
}

pub type PositionSystemResult<T> = Result<T, PositionSystemError>;

pub struct PositionSystem {
    components: HashMap<EntityId, PositionComponent>,
}

impl PositionSystem {
    pub fn new() -> Self {
        PositionSystem {
            components: HashMap::new(),
        }
    }

    pub fn get_component(&self, entity: &EntityId) -> Option<&PositionComponent> {
        self.components.get(entity)
    }

    pub fn get_component_mut(&mut self, entity: &EntityId) -> Option<&mut PositionComponent> {
        self.components.get_mut(entity)
    }

    pub fn get_position(&self, entity: &EntityId) -> Option<&Vec2F> {
        self.components.get(entity).map(|pc| pc.get_position())
    }

    pub fn add_component(&mut self, entity: EntityId, component: PositionComponent) -> PositionSystemResult<()> {
        if self.components.contains_key(&entity) {
            return Err(PositionSystemError::ComponentAlreadyAdded(component))
        } else {
            self.components.insert(entity, component);
            Ok(())
        }
    }
}
