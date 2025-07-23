use std::sync::Arc;

use crate::app_data::AppData;
use crate::handlers::*;
use axum::{
    routing::{get, post},
    Router,
};
use axum::routing::{delete, patch};
use tokio::sync::Mutex;

pub fn get_router(app_data: AppData) -> Router {
    let api_routes = Router::new()
        .route(
            "/account/create",
            post(create_account),
        )
        .route(
            "/accounts/{username}/login",
            post(login_to_account),
        )
        .route(
            "/accounts/{username}/logout",
            post(logout_account),
        )
        .route(
            "/accounts/{username}",
            get(get_account_details).delete(delete_account)
        )
        .route(
            "/accounts/{username}/password",
            patch(update_account_password)
        )
        .route(
            "/accounts/{username}/characters",
            get(get_characters_of_account)
        )
        .route(
            "/accounts/{username}/character/new",
            post(create_character_for_account)
        );

    Router::new()
        .route("/", get(overall_status))
        .nest("/api", api_routes)
        .with_state(app_data)
}
