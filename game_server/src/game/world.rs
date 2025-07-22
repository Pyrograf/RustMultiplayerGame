use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use database_adapter::character::CharacterData;
use crate::game::entity::component::{MovementComponent, NameComponent, PositionComponent};
use crate::game::entity::EntityId;
use crate::game::math::Vec2F;
use crate::game::system::{MovementSystem, NameSystem, PositionSystem};

#[derive(Debug, thiserror::Error)]
pub enum WorldError {
    #[error(transparent)]
    ElapsedTimeout(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    RecvError(#[from]  oneshot::error::RecvError),
}

pub type WorldResult<T> =  Result<T, WorldError>;

const TICK_DURATION_MS: u64 = 32;
const TICK_DT_SEC: f32 = TICK_DURATION_MS as f32 / 1000.0;

const DEFAULT_CMD_TIMEOUT_MS: u64 = 1000;

pub enum WorldManagerCmd {
    GetEntitiesCount,
    SpawnCharacter {
        character_data: CharacterData,
    }
}

pub struct WorldManagerCmdWrapped {
    cmd: WorldManagerCmd,
    response: oneshot::Sender<WorldManagerCmdResult>,
}
pub enum WorldManagerCmdResult {
    EntitiesCount(usize),
    SpawnCharacter(EntityId),
}
pub struct WorldManager {
    handle: JoinHandle<()>,
    tx: mpsc::Sender<WorldManagerCmdWrapped>,
}

impl WorldManager {
    pub async fn run() -> Self {
        let (tx, mut rx) = mpsc::channel::<WorldManagerCmdWrapped>(128);

        let handle = tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_millis(TICK_DURATION_MS));

            let mut world = World::new();

            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        world.tick(TICK_DT_SEC);
                    },
                    cmd_wrapped = rx.recv() => match cmd_wrapped {
                        Some(cmd_wrapped) => {
                            let cmd_response = match cmd_wrapped.cmd {
                                WorldManagerCmd::GetEntitiesCount => WorldManagerCmdResult::EntitiesCount(world.entities.len()),
                                WorldManagerCmd::SpawnCharacter { character_data } => {
                                    let entity_id = world.generate_new_entity();
                                    
                                    // Safe unwraps - newly created entity
                                    let character_position = Vec2F::new(character_data.position_x, character_data.position_y);
                                    world.position_system.add_component(entity_id, PositionComponent::new(entity_id, character_position)).unwrap();
                                    world.movement_system.add_component(entity_id, MovementComponent::new(entity_id, character_data.speed)).unwrap();
                                    world.name_system.add_component(entity_id, NameComponent::new(entity_id, character_data.name)).unwrap();
                                    
                                    WorldManagerCmdResult::SpawnCharacter(entity_id)
                                }
                            };
                            if cmd_wrapped.response.send(cmd_response).is_err() {
                                tracing::warn!("Cmd response dropped")
                            }
                        },
                        None => {
                            tracing::info!("Shutting world manager");
                            break;
                        }
                    }
                }
            }
        });

        Self { handle, tx }
    }

    pub async fn request_cmd_with_timeout(&self, cmd: WorldManagerCmd, timeout_time: Duration) -> WorldResult<WorldManagerCmdResult> {
        let result = tokio::time::timeout(timeout_time, async move {
            let (resp_tx, resp_rx) = oneshot::channel();
                let _ = self.tx.send(WorldManagerCmdWrapped {cmd, response: resp_tx}).await;
                resp_rx.await
        }).await??;
        Ok(result)
    }

    pub async fn request_cmd_with_default_timeout(&self, cmd: WorldManagerCmd) -> WorldResult<WorldManagerCmdResult> {
        self.request_cmd_with_timeout(cmd, Duration::from_millis(DEFAULT_CMD_TIMEOUT_MS)).await
    }

    pub async fn get_entities_count(&self) -> usize {
        match self.request_cmd_with_default_timeout(WorldManagerCmd::GetEntitiesCount).await {
            Ok(WorldManagerCmdResult::EntitiesCount(count)) => count,
            _ => 0,
        }
    }

    pub async fn spawn_character_entity(&self, character_data: CharacterData) -> WorldResult<EntityId> {
        match self.request_cmd_with_default_timeout(WorldManagerCmd::SpawnCharacter { character_data }).await {
            Ok(WorldManagerCmdResult::SpawnCharacter(entity_id)) => Ok(entity_id),
            Err(err) => Err(err),
            Ok(_) => panic!("Failed to spawn character entity - bad WorldManagerCmdResult"),
        }
    }
}


pub struct World {
    entities: Vec<EntityId>,
    next_entity_id: EntityId,
    position_system: PositionSystem,
    movement_system: MovementSystem,
    name_system: NameSystem,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            next_entity_id: 0,
            position_system: PositionSystem::new(),
            movement_system: MovementSystem::new(),
            name_system: NameSystem::new(),
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.movement_system.tick(&mut self.position_system, dt);
    }

    pub fn generate_new_entity(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity_id);
        entity_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_empty_world_count_entities() {
        let world_manager = WorldManager::run().await;
        assert_eq!(world_manager.get_entities_count().await, 0);
    }
}