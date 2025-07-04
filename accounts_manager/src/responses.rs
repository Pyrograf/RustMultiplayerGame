use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use database_adapter::DatabaseAdapterError;

#[derive(Debug, Serialize, Deserialize)]

pub struct AccountsServerStatus {
    pub motd: String,
    pub accounts_count: usize,
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize, PartialEq, Clone)]
pub enum ApiError {
    #[error(transparent)]
    DatabaseAdapterError(#[from] DatabaseAdapterError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            ApiError::DatabaseAdapterError(err) => match err {
                DatabaseAdapterError::UsernameNotFound => StatusCode::BAD_REQUEST,
                DatabaseAdapterError::UsernameAlreadyExists => StatusCode::BAD_REQUEST,
                DatabaseAdapterError::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DatabaseAdapterError::BadPassword => StatusCode::BAD_REQUEST,
                DatabaseAdapterError::CharacterIdNotFound => StatusCode::BAD_REQUEST,
                DatabaseAdapterError::CharacterAlreadyExists => StatusCode::BAD_REQUEST,
            }
        };
        (status_code, Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::{to_bytes, Body};
    use database_adapter::DatabaseAdapterError;

    #[test]
    fn test_serialization_deserialization_api_error() {
        let error = ApiError::DatabaseAdapterError(DatabaseAdapterError::UsernameNotFound);
        let error_str =  serde_json::to_string(&error).unwrap();
        assert_eq!(error_str, "{\"DatabaseAdapterError\":\"UsernameNotFound\"}");

        let error_recreated =  serde_json::from_str::<ApiError>(&error_str).unwrap();
        assert_eq!(error, error_recreated);
    }
    
    #[tokio::test]
    async fn test_api_error_response_decoding() {
        let error = ApiError::DatabaseAdapterError(DatabaseAdapterError::UsernameNotFound);
        let error_to_response = error.clone().into_response();
        assert_eq!(error_to_response.status(), StatusCode::BAD_REQUEST);
        let response_body = error_to_response.into_body();

        // Use `usize::MAX` if you don't care about the maximum size.
        let body_bytes = to_bytes(response_body, usize::MAX).await.unwrap();
        let body_str =  std::str::from_utf8(&body_bytes).unwrap();
        assert_eq!(body_str, "{\"DatabaseAdapterError\":\"UsernameNotFound\"}");

        let error_recreated =  serde_json::from_str::<ApiError>(&body_str).unwrap();
        assert_eq!(error, error_recreated);
    }
}