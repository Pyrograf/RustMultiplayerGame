use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::account::AccountError;

#[derive(Debug, Serialize, Deserialize)]

pub struct AccountsServerStatus {
    pub motd: String,
    pub accounts_count: usize,
}

impl IntoResponse for AccountError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            AccountError::UsernameNotFound => StatusCode::BAD_REQUEST,
            AccountError::UsernameAlreadyExists => StatusCode::BAD_REQUEST,
            AccountError::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, Json(self)).into_response()
    }
}