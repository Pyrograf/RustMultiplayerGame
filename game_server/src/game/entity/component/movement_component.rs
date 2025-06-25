use std::any::Any;
use std::cell::RefCell;
use std::rc::Weak;
use crate::game::entity::EntityId;
use crate::game::entity::component::Component;
use crate::game::math::Vec2F;
use crate::game::world::World;

pub enum MoveDirection {
    North,
    South,
    West,
    East,
}

impl MoveDirection {
    pub const fn as_vec2f_normal(&self) -> Vec2F {
        match self {
            MoveDirection::North => Vec2F::new(0.0, 1.0),
            MoveDirection::South => Vec2F::new(0.0, -1.0),
            MoveDirection::West => Vec2F::new(-1.0, 0.0),
            MoveDirection::East => Vec2F::new(1.0, 0.0),
        }
    }
}

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


pub struct MovementComponent {
    entity_id: EntityId,
    pub target: Option<(Vec2F, Option<MovementState>)>,
    pub speed: f32,
}

impl MovementComponent {
    pub fn new(speed: f32, entity_id: EntityId) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_direction_getting_normal_vector() {
        assert_eq!(MoveDirection::North.as_vec2f_normal(), Vec2F::new(0.0, 1.0));
    }
}







//////////////////////////////////


// pub trait EntityMove {
//     fn move_to(&mut self, target: Vec2F) -> EntityMoveResult<()> {
//         if self.is_moving() {
//             Err(EntityMoveError::AlreadyMoving)
//         } else {
//             let start = *self.get_position();
//             let target = align_vec2f_to_tile(target);
//
//             if start != target {
//                 let translation = target - start;
//                 let duration = translation.get_length() / self.get_speed();
//                 self.set_movement(start, target, duration, 0.0);
//             }
//             Ok(())
//         }
//     }
//
//     fn tick(&mut self, dt: f32) {
//         if let Some((start, target, duration, mut elapsed)) = self.get_movement() {
//             elapsed += dt;
//             let t = (elapsed / duration).min(1.0);
//             if t >=  1.0 {
//                 // Target reached
//                 self.set_position(target);
//                 self.stop_moving();
//             } else {
//                 let position = Vec2F::lerp(&start, &target, t);
//                 self.set_position(position);
//                 self.set_movement(start, target, duration, elapsed)
//             };
//         }
//     }
//
//     fn is_moving(&self) -> bool {
//         self.get_movement().is_some()
//     }
//
//     fn get_movement(&self) -> Option<(Vec2F, Vec2F, f32, f32)>;
//
//     fn set_movement(&mut self, start: Vec2F, target: Vec2F, duration: f32, elapsed: f32);
//
//     fn stop_moving(&mut self);
//
//     fn get_speed(&self) -> f32;
//
//     fn get_position(&self) -> &Vec2F;
//
//     fn set_position(&mut self, position: Vec2F);
// }
