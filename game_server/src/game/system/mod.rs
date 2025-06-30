pub mod movement_system;
pub mod position_system;

pub use position_system::PositionSystem;
pub use movement_system::MovementSystem;

#[cfg(test)]
mod tests {
    use crate::game::entity::component::{MovementComponent, PositionComponent};
    use crate::game::math::Vec2F;
    use crate::game::system::movement_system::MovementSystemError;
    use crate::game::system::position_system::PositionSystemError;
    use super::*;

    #[test]
    fn test_moving_entity() {
        const DT: f32 = 0.25;
        const SPEED: f32 = 1.0;
        const EVERYTICK_TRANSLATION: f32 = DT * SPEED;
        let mut position_system = PositionSystem::new();
        let mut movement_system = MovementSystem::new();

        let target_position = Vec2F::new(1.0, 0.0);

        let entity_id = 1;

        position_system.add_component(entity_id, PositionComponent::new(entity_id, Vec2F::new(0.0, 0.0))).unwrap();
        movement_system.add_component(entity_id, MovementComponent::new(entity_id,1.0)).unwrap();

        movement_system.move_entity_to(entity_id, target_position.clone()).unwrap();

        let mut tick_idx = 0;
        {
            tick_idx += 1;
            movement_system.tick(&mut position_system, DT);
            let pc = position_system.get_component(&entity_id).unwrap();
            let mc = movement_system.get_component(&entity_id).unwrap();
            assert!(pc.get_position().approx_eq(&Vec2F::new(tick_idx as f32 * EVERYTICK_TRANSLATION, 0.0)));
            assert!(mc.is_moving());
        }

        {
            tick_idx += 1;
            movement_system.tick(&mut position_system, DT);
            let pc = position_system.get_component(&entity_id).unwrap();
            let mc = movement_system.get_component(&entity_id).unwrap();
            assert!(pc.get_position().approx_eq(&Vec2F::new(tick_idx as f32 * EVERYTICK_TRANSLATION, 0.0)));
            assert!(mc.is_moving());
        }

        {
            tick_idx += 1;
            movement_system.tick(&mut position_system, DT);
            let pc = position_system.get_component(&entity_id).unwrap();
            let mc = movement_system.get_component(&entity_id).unwrap();
            assert!(pc.get_position().approx_eq(&Vec2F::new(tick_idx as f32 * EVERYTICK_TRANSLATION, 0.0)));
            assert!(mc.is_moving());
        }

        {
            tick_idx += 1;
            movement_system.tick(&mut position_system, DT);
            let pc = position_system.get_component(&entity_id).unwrap();
            let mc = movement_system.get_component(&entity_id).unwrap();
            assert!(pc.get_position().approx_eq(&Vec2F::new(tick_idx as f32 * EVERYTICK_TRANSLATION, 0.0)));

            assert!(pc.get_position().approx_eq(&target_position));
            // Should stop here
            assert!(!mc.is_moving());
        }
    }

    #[test]
    fn test_moving_system_check_already_moving() {
        const DT: f32 = 0.25;
        const SPEED: f32 = 1.0;
        const EVERYTICK_TRANSLATION: f32 = DT * SPEED;
        let mut position_system = PositionSystem::new();
        let mut movement_system = MovementSystem::new();

        let entity_id = 1;

        position_system.add_component(entity_id, PositionComponent::new(entity_id, Vec2F::new(0.0, 0.0))).unwrap();
        movement_system.add_component(entity_id, MovementComponent::new(entity_id,1.0)).unwrap();

        movement_system.move_entity_to(entity_id, Vec2F::new(1.0, 0.0)).unwrap();
        assert!(movement_system.get_component(&entity_id).unwrap().is_moving());
        assert!(matches!(movement_system.move_entity_to(entity_id, Vec2F::new(1.0, 0.0)), Err(MovementSystemError::AlreadyMoving)));

        movement_system.tick(&mut position_system, DT); // 0.25
        assert!(movement_system.get_component(&entity_id).unwrap().is_moving());
        assert!(matches!(movement_system.move_entity_to(entity_id, Vec2F::new(1.0, 0.0)), Err(MovementSystemError::AlreadyMoving)));

        movement_system.tick(&mut position_system, DT); // 0.5
        movement_system.tick(&mut position_system, DT); // 0.75
        movement_system.tick(&mut position_system, DT); // 1.0 - stopped
        assert!(!movement_system.get_component(&entity_id).unwrap().is_moving());
        movement_system.move_entity_to(entity_id, Vec2F::new(1.0, 0.0)).unwrap();
    }

    #[test]
    fn test_entity_move_without_component_added() {
        const DT: f32 = 0.25;
        const SPEED: f32 = 1.0;
        const EVERYTICK_TRANSLATION: f32 = DT * SPEED;
        let mut position_system = PositionSystem::new();
        let mut movement_system = MovementSystem::new();

        let entity_id = 1;

        assert!(matches!(movement_system.move_entity_to(entity_id, Vec2F::new(1.0, 0.0)), Err(MovementSystemError::NoMoveComponent)));
    }

    #[test]
    fn test_component_already_added() {
        const DT: f32 = 0.25;
        const SPEED: f32 = 1.0;
        const EVERYTICK_TRANSLATION: f32 = DT * SPEED;
        let mut position_system = PositionSystem::new();
        let mut movement_system = MovementSystem::new();

        let entity_id = 1;

        position_system.add_component(entity_id, PositionComponent::new(entity_id, Vec2F::new(0.0, 0.0))).unwrap();
        movement_system.add_component(entity_id, MovementComponent::new(entity_id,1.0)).unwrap();

        let ps_result = position_system.add_component(entity_id, PositionComponent::new(entity_id, Vec2F::new(0.0, 0.0)));
        assert!(matches!(ps_result, Err(PositionSystemError::ComponentAlreadyAdded(_))));
        let ms_result = movement_system.add_component(entity_id, MovementComponent::new(entity_id,1.0));
        assert!(matches!(ms_result, Err(MovementSystemError::ComponentAlreadyAdded(_))));
    }
}