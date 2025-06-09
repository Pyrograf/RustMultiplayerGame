use std::sync::Arc;
use axum::{response::Html, Json};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::sync::Mutex;
use crate::account::{Account, AccountError};
use crate::app_data::AppData;
use crate::requests::CreateAccountRequest;
use crate::responses::AccountsServerStatus;
use crate::services;

pub async fn overall_status(State(app_data): State<Arc<Mutex<AppData>>>) -> Json<AccountsServerStatus> {
    let accounts_count = {
        let app_data = app_data.lock().await;
        app_data.accounts_manager.count()
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
) -> Result<impl IntoResponse, AccountError> {
    let mut app_data = app_data.lock().await;
    match services::create_account(payload.username, payload.password, &mut app_data.accounts_manager) {
        Ok(account) => Ok((StatusCode::CREATED, "Account created")),
        Err(err) => Err(err),
    }
}
