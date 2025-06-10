use std::sync::Arc;

use crate::app_data::AppData;
use crate::handlers::*;
use axum::{
    routing::{get, post},
    Router,
};
use axum::routing::{delete, patch};
use tokio::sync::Mutex;

pub fn get_router(app_data: Arc<Mutex<AppData>>) -> Router {
    let api_routes = Router::new()
        .route(
            "/account/create",
            post(create_account),
        )
        .route(
            "/accounts/{username}",
            delete(delete_account)
        )
        .route(
            "/accounts/{username}/password",
            patch(update_account_password)
        )
        .with_state(app_data.clone());

    Router::new()
        .route("/", get(overall_status).with_state(app_data.clone()))
        .nest("/api", api_routes)
}
