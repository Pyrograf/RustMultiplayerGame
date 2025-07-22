use serde::{Deserialize, Serialize};
use database_adapter::character::CharacterId;

#[derive(Debug, Serialize, Deserialize)]
pub enum GameServerRequest {
    Status,
    EntitiesCount,
    AttachToCharacter {
        character_id: CharacterId,
    },
}