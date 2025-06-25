pub mod movement_component;
pub mod position_component;

pub use movement_component::MovementComponent;
pub use position_component::PositionComponent;

use std::any::Any;
use crate::game::entity::EntityId;
use crate::game::world::World;

pub trait Component {
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn get_entity_id(&self) -> EntityId;
}
