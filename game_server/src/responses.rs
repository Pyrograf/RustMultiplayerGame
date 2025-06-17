use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GameServerResponse {
    Status {
        info: String,
    },
}