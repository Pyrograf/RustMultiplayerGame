use serde::{Deserialize, Serialize};
use crate::game::entity::EntityId;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseResult {
    Success,
    Error {
        message: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameServerResponse {
    Status {
        info: String,
    },
    EntitiesCount {
        count: usize
    },
    AttachToCharacter {
        result: ResponseResult,
    },
}