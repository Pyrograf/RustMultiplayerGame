use serde::{Deserialize, Serialize};

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
pub struct ChangePasswordRequest {
    pub username: String,
    pub password_old: String,
    pub password_new: String,
}