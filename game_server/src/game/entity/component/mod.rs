pub mod movement_component;
pub mod position_component;
pub mod name_component;

pub use movement_component::MovementComponent;
pub use position_component::PositionComponent;
pub use name_component::NameComponent;

use std::any::Any;
use crate::game::entity::EntityId;
use crate::game::world::World;

pub trait Component {
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    /// Force attaching to Entity
    fn get_entity_id(&self) -> EntityId;
}
