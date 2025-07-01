use serde::{Deserialize, Serialize};
use crate::game::character::CharacterId;

#[derive(Debug, Serialize, Deserialize)]
pub enum GameServerRequest {
    Status,
    EntitiesCount,
    AttachToCharacter {
        character_id: CharacterId,
    },
}