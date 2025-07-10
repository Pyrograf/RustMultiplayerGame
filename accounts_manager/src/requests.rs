use serde::{Deserialize, Serialize};
use database_adapter::character::CharacterId;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAccountRequestBody {
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub password_old: String,
    pub password_new: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCharacterRequest {
    pub password: String,
    pub character_name: String,
}