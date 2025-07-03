use std::sync::Arc;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::sync::Mutex;
use crate::app_data::AppData;
use crate::requests::{CreateAccountRequest, DeleteAccountRequestBody, UpdatePasswordRequest};
use crate::responses::{AccountsServerStatus, ApiError};
use crate::services;

pub async fn overall_status(State(app_data): State<Arc<Mutex<AppData>>>) -> Json<AccountsServerStatus> {
    let accounts_count = {
        let app_data = app_data.lock().await;
        app_data.database_adapter.get_accounts_count().await.unwrap_or_default()
    };

    let status = AccountsServerStatus {
        motd: String::from("Accounts manager is running!"),
        accounts_count,
    };

    Json(status)
}

pub async fn create_account(
    State(app_data): State<Arc<Mutex<AppData>>>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut app_data = app_data.lock().await;
    match services::create_account(payload.username, payload.password, app_data.database_adapter.clone()).await {
        Ok(account) => Ok((StatusCode::CREATED, "Account created")),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_account(
    State(app_data): State<Arc<Mutex<AppData>>>,
    Path(username): Path<String>,
    Json(payload): Json<DeleteAccountRequestBody>,
) -> Result<impl IntoResponse, ApiError> {
    let mut app_data = app_data.lock().await;
    match services::delete_account(username, payload.password, app_data.database_adapter.clone()).await {
        Ok(account)  => Ok((StatusCode::OK, "Account deleted")),
        Err(err) => Err(err.into()),
    }
}

pub async fn update_account_password(
    State(app_data): State<Arc<Mutex<AppData>>>,
    Path(username): Path<String>,
    Json(payload): Json<UpdatePasswordRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut app_data = app_data.lock().await;
    match services::update_account_password(
        username, 
        payload.password_old, 
        payload.password_new, 
        app_data.database_adapter.clone()
    ).await {
        Ok(account)  => Ok((StatusCode::OK, "Password changed")),
        Err(err) => Err(err.into()),
    }
}