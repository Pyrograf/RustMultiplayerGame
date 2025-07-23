use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_jwt_auth::Claims;
use serde::Serialize;
use crate::app_data::{AccountManagerClaims, AppData};
use crate::requests::{CreateAccountRequest, LoginAccountRequest, NewCharacterRequest, UpdatePasswordRequest};
use crate::responses::{AccountDetails, AccountsServerStatus, ApiError};
use crate::services;

pub async fn overall_status(State(app_data): State<AppData>) -> Json<AccountsServerStatus> {
    let accounts_count = {
        app_data.database_adapter.get_accounts_count().await.unwrap_or_default()
    };

    let status = AccountsServerStatus {
        motd: String::from("Accounts manager is running!"),
        accounts_count,
    };

    Json(status)
}

pub async fn create_account(
    State(app_data): State<AppData>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<impl IntoResponse, ApiError> {
    match services::create_account(payload.username, payload.password, app_data.database_adapter.clone()).await {
        Ok(account) => Ok((StatusCode::CREATED, "Account created")),
        Err(err) => Err(err.into()),
    }
}

pub async fn login_to_account(
    State(app_data): State<AppData>,
    Path(username): Path<String>,
    Json(payload): Json<LoginAccountRequest>,
) -> Result<impl IntoResponse, ApiError> {
    match services::login_to_account(username, payload.password, app_data.database_adapter.clone()).await {
        Ok(account) => Ok(Json(account).into_response()),
        Err(err) => Err(err.into()),
    }
}

pub async fn logout_account(
    State(_app_data): State<AppData>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    Ok((StatusCode::OK, format!("Logout account {}", username)))
}

pub async fn get_account_details(
    Claims(_claims): Claims<AccountManagerClaims>,
    Path(username): Path<String>,
    State(app_data): State<AppData>,
) -> Result<impl IntoResponse, ApiError> {
    let characters_count = app_data.database_adapter.get_characters_of_account(&username).await?.len();

    let details = AccountDetails {
        characters_count
    };

    Ok(Json(details).into_response())
}

pub async fn delete_account(
    Claims(claims): Claims<AccountManagerClaims>,
    State(app_data): State<AppData>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    match services::delete_account(username, app_data.database_adapter.clone()).await {
        Ok(account)  => Ok((StatusCode::OK, "Account deleted")),
        Err(err) => Err(err.into()),
    }
}

pub async fn update_account_password(
    Claims(claims): Claims<AccountManagerClaims>,
    State(app_data): State<AppData>,
    Path(username): Path<String>,
    Json(payload): Json<UpdatePasswordRequest>,
) -> Result<impl IntoResponse, ApiError> {
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

pub async fn get_characters_of_account(
    Claims(claims): Claims<AccountManagerClaims>,
    State(app_data): State<AppData>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    match services::get_characters_of_account(
        username,
        app_data.database_adapter.clone()
    ).await {
        Ok(characters)  => Ok(Json(characters)),
        Err(err) => Err(err.into()),
    }
}

pub async fn create_character_for_account(
    Claims(claims): Claims<AccountManagerClaims>,
    State(app_data): State<AppData>,
    Path(username): Path<String>,
    Json(payload): Json<NewCharacterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    match services::create_character_for_account(
        username,
        payload.character_name,
        app_data.database_adapter.clone()
    ).await {
        Ok(new_character_id)  => Ok(Json(new_character_id)),
        Err(err) => Err(err.into()),
    }
}
