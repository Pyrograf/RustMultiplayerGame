use crate::game::math::Vec2F;

pub struct Entity {
    id: EntityId,
    position: Vec2F,
}

pub type EntityId = u32;