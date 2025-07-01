use std::any::Any;
use crate::game::entity::component::Component;
use crate::game::entity::EntityId;
use crate::game::math::Vec2F;

#[derive(Debug)]
pub struct NameComponent {
    entity_id: EntityId,
    name: String,
}

impl NameComponent {
    pub fn new(entity_id: EntityId, name: String) -> Self {
        Self { entity_id, name }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Component for NameComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn get_entity_id(&self) -> EntityId {
        self.entity_id
    }
}
