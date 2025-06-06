use axum::{response::Html, Json};
use crate::requests::AccountsServerStatus;

pub async fn overall_status() -> Json<AccountsServerStatus> {
    let status = AccountsServerStatus {
        motd: "Accounts manager is running! Nothing interesting so far...".to_string(),
    };
    Json(status)
}

pub async fn create_account() -> Html<&'static str> {
    Html("<h1>create_account!</h1>")
}