use std::sync::Arc;

use crate::app_data::AppData;
use crate::handlers::*;
use axum::{
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;

pub fn get_router(app_data: Arc<Mutex<AppData>>) -> Router {
    let api_routes = Router::new().route(
        "/account/create",
        post(create_account).with_state(app_data.clone()),
    );

    Router::new()
        .route("/", get(overall_status).with_state(app_data.clone()))
        .nest("/api", api_routes)
}
