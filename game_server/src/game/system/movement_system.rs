use std::collections::HashMap;
use crate::game::entity::component::movement_component::MovementState;
use crate::game::entity::component::MovementComponent;
use crate::game::entity::EntityId;
use crate::game::math::Vec2F;
use crate::game::system::PositionSystem;
use crate::game::tile_math::align_vec2f_to_tile;

#[derive(Debug, thiserror::Error)]
pub enum EntityMoveError {
    #[error("No move component")]
    NoMoveComponent,

    #[error("Already moving")]
    AlreadyMoving,
}

pub type EntityMoveResult<T> = Result<T, EntityMoveError>;

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

                match movement_state {
                    Some(state) => {
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
                    },
                    None => {
                        // Attempt to start moving
                        if pc.get_position() == target_position {
                            // Already reached target
                            mc.target = None;
                        } else {
                            // Start moving
                            let translation = *target_position - *pc.get_position();
                            let duration = translation.get_length() / mc.speed;
                            *movement_state = Some(MovementState::new(*pc.get_position(), duration));
                        }
                    }
                }

            }
        }
    }

    pub fn move_entity_to(&mut self, entity_id: EntityId, target: Vec2F) -> EntityMoveResult<()> {
        let target = align_vec2f_to_tile(target);

        let mc = match self.components.get_mut(&entity_id) {
            Some(mc) => mc,
            None => {
                return Err(EntityMoveError::NoMoveComponent);
            },
        };

        if mc.target.is_some() {
            return Err(EntityMoveError::AlreadyMoving);
        }

        mc.target = Some((target, None));

        Ok(())
    }
}