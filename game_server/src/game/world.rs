use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use crate::game::entity::Entity;
use crate::GameServer;

#[derive(Debug, thiserror::Error)]
pub enum WorldError {
    #[error(transparent)]
    ElapsedTimeout(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    RecvError(#[from]  oneshot::error::RecvError),
}

pub type WorldResult<T> =  Result<T, WorldError>;

const TICK_DURATION_MS: u64 = 32;
const DEFAULT_CMD_TIMEOUT_MS: u64 = 1000;

pub enum WorldManagerCmd {
    GetEntitiesCount
}

pub struct WorldManagerCmdWrapped {
    cmd: WorldManagerCmd,
    response: oneshot::Sender<WorldManagerCmdResult>,
}
pub enum WorldManagerCmdResult {
    EntitiesCount(usize),
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
                        world.tick().await;
                    },
                    cmd_wrapped = rx.recv() => match cmd_wrapped {
                        Some(cmd_wrapped) => {
                            let cmd_response = match cmd_wrapped.cmd {
                                WorldManagerCmd::GetEntitiesCount => WorldManagerCmdResult::EntitiesCount(world.entities.len()),
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
}


pub struct World {
    entities: Vec<Entity>
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: vec![]
        }
    }

    pub async fn tick(&mut self) {

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