use std::collections::HashMap;
use crate::game::entity::component::movement_component::MovementState;
use crate::game::entity::component::{MovementComponent, PositionComponent};
use crate::game::entity::EntityId;
use crate::game::math::Vec2F;
use crate::game::system::position_system::{PositionSystemError, PositionSystemResult};
use crate::game::system::PositionSystem;
use crate::game::tile_math::align_vec2f_to_tile;

#[derive(Debug, thiserror::Error)]
pub enum MovementSystemError {
    #[error("No move component")]
    NoMoveComponent,

    #[error("Already moving")]
    AlreadyMoving,

    #[error("Component already added")]
    ComponentAlreadyAdded(MovementComponent)
}

pub type MovementSystemResult<T> = Result<T, MovementSystemError>;

pub struct MovementSystem {
    components: HashMap<EntityId, MovementComponent>,
}

impl MovementSystem {
    pub fn new() -> Self {
        MovementSystem {
            components: HashMap::new(),
        }
    }

    pub fn tick(&mut self, position_system: &mut PositionSystem, dt: f32) {
        for (eid, mc) in self.components.iter_mut() {
            if let Some((target_position, movement_state)) = &mut mc.target {
                let mut pc = match position_system.get_component_mut(eid) {
                    Some(p) => p,
                    None => {
                        tracing::error!("Skipped entity {eid}: missing position component, cannot move!");
                        continue;
                    }
                };

                // In the same tick movement must be started and updated

                // Attempt to start moving if not moving
                if movement_state.is_none() {
                    // Attempt to start moving
                    if pc.get_position() == target_position {
                        // Already reached target
                        mc.target = None;
                        continue;
                    } else {
                        // Start moving
                        let translation = *target_position - *pc.get_position();
                        let duration = translation.get_length() / mc.speed;
                        *movement_state = Some(MovementState::new(*pc.get_position(), duration));
                    }
                }

                // Stop or update moving
                let state = movement_state.as_mut().unwrap();
                // Already moving
                state.elapsed += dt;
                let t = (state.elapsed / state.duration).min(1.0);
                if t >=  1.0 {
                    // Target reached
                    pc.set_position(*target_position);
                    mc.target = None;
                } else {
                    // Translate
                    let position = Vec2F::lerp(&state.start, &target_position, t);
                    pc.set_position(position);
                };
            }
        }
    }

    pub fn move_entity_to(&mut self, entity_id: EntityId, target: Vec2F) -> MovementSystemResult<()> {
        let target = align_vec2f_to_tile(target);

        let mc = match self.components.get_mut(&entity_id) {
            Some(mc) => mc,
            None => {
                return Err(MovementSystemError::NoMoveComponent);
            },
        };

        if mc.target.is_some() {
            return Err(MovementSystemError::AlreadyMoving);
        }

        mc.target = Some((target, None));

        Ok(())
    }


    pub fn get_component(&self, entity: &EntityId) -> Option<&MovementComponent> {
        self.components.get(entity)
    }

    pub fn get_component_mut(&mut self, entity: &EntityId) -> Option<&mut MovementComponent> {
        self.components.get_mut(entity)
    }

    pub fn add_component(&mut self, entity: EntityId, component: MovementComponent) -> MovementSystemResult<()> {
        if self.components.contains_key(&entity) {
            return Err(MovementSystemError::ComponentAlreadyAdded(component))
        } else {
            self.components.insert(entity, component);
            Ok(())
        }
    }
}