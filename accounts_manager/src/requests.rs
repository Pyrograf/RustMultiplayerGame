use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub username: String,
    pub password: String,
}
