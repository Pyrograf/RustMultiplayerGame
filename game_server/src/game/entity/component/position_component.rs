use std::any::Any;
use std::cell::RefCell;
use std::rc::Weak;
use crate::game::entity::EntityId;
use crate::game::entity::component::Component;
use crate::game::math::Vec2F;
use crate::game::world::World;

pub struct PositionComponent {
    entity_id: EntityId,
    position: Vec2F,
}

impl PositionComponent {
    pub fn new(entity_id: EntityId, position: Vec2F) -> Self {
        Self { entity_id, position }
    }

    pub fn get_position(&self) -> &Vec2F {
        &self.position
    }

    pub fn set_position(&mut self, position: Vec2F) {
        self.position = position;
    }
}

impl Component for PositionComponent {
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
