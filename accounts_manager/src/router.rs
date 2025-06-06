use std::sync::Arc;

use axum::{
    routing::{
        get,
        post,
    },
    Router
};

use crate::app_data::AppData;
use crate::handlers::*;

pub fn get_router(app_data: Arc<AppData>) -> Router {
    let api_routes = Router::new()
        .route("/account/create", post(create_account)
            .with_state(app_data.clone())
        );

    Router::new()
        .route("/", get(overall_status))
        .nest("/api", api_routes)
}