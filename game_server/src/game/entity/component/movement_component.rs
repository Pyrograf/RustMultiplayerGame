use std::any::Any;
use std::cell::RefCell;
use std::rc::Weak;
use crate::game::entity::EntityId;
use crate::game::entity::component::Component;
use crate::game::math::Vec2F;
use crate::game::world::World;

#[derive(Debug)]
pub struct MovementState {
    pub start: Vec2F,
    pub duration: f32,
    pub elapsed: f32,
}

impl MovementState {
    pub fn new(start: Vec2F, duration: f32) -> Self {
        Self { start, duration, elapsed: 0.0 }
    }
}

#[derive(Debug)]
pub struct MovementComponent {
    entity_id: EntityId,
    pub target: Option<(Vec2F, Option<MovementState>)>,
    pub speed: f32,
}

impl MovementComponent {
    pub fn new(entity_id: EntityId, speed: f32) -> Self {
        Self {
            entity_id,
            target: None,
            speed
        }
    }

    pub fn is_moving(&self) -> bool {
        self.target.is_some()
    }
}

impl Component for MovementComponent {
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