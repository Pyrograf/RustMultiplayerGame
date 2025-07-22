use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use database_adapter::character::CharacterId;
use database_adapter::{DatabaseAdapter, DatabaseAdapterError};
use crate::game::entity::EntityId;
use crate::game::world::{WorldError, WorldManager};
use crate::session::ConnectionSessionId;

pub mod world;
pub mod player;
pub mod entity;

mod math;
mod tile_math;
mod system;
/// Ideas
/// - client is not directly related to player
/// - player is entity with some addiytional data
/// - client should be able to access world, example: viewer to look at world content
/// - maybe client should have permissions: regular client should have attached player, admin cleint can freely access API


#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Session already attached to entity")]
    SessionAlreadyAttachedToEntity{
        entity_id: EntityId,
    },

    // #[error("Character not found")]
    // CharacterNotFound,
    
    #[error(transparent)]
    DatabaseAdapterError(#[from] DatabaseAdapterError),

    #[error(transparent)]
    WorldError(#[from] WorldError),
}

pub type GameResult<T> =  Result<T, GameError>;

pub struct Game {
    pub world_manager: WorldManager,
    pub database_adapter: Arc<dyn DatabaseAdapter>,
    sessions_entities: Mutex<HashMap<ConnectionSessionId, EntityId>>,
}

impl Game {
    pub async fn new(database_adapter: Arc<dyn DatabaseAdapter>) -> Self {
        let world_manager = WorldManager::run().await;
        
        Self {
            world_manager,
            database_adapter,
            sessions_entities:  Mutex::new(HashMap::new()),
        }
    }

    async fn get_entity_id_of_session(&self, session_id: ConnectionSessionId) -> Option<EntityId> {
        self.sessions_entities.lock().await.get(&session_id).copied()
    }

    async fn attach_entity_id_to_session(&self, connection_id: ConnectionSessionId, entity_id: EntityId) -> Result<(), EntityId> {
        match self.sessions_entities.lock().await.insert(connection_id, entity_id) {
            Some(entity_id) => Err(entity_id),
            None => Ok(())
        }
    }

    pub async fn spawn_character_entity(&self, connection_id: ConnectionSessionId, character_id: CharacterId) -> GameResult<EntityId> {
        if let Some(entity_id) = self.get_entity_id_of_session(connection_id).await {
            return Err(GameError::SessionAlreadyAttachedToEntity { entity_id });
        }

        let character_data = self.database_adapter.get_character_by_id(character_id).await?;

        match self.world_manager.spawn_character_entity(character_data).await {
            Ok(spawned_entity_id) => {
                if self.attach_entity_id_to_session(connection_id, spawned_entity_id).await.is_err() {
                    tracing::error!("Could not attach entity to session id: {spawned_entity_id}");
                }
                Ok(spawned_entity_id)
            },
            Err(e) => Err(e.into())
        }
        // Note: try refactoring,
        // check if storing ids in hashmap is really needed,
        // take care of cleaning upon disconnection
    }
}


