use axum::{
    response::Html
};

pub async fn overall_status() -> Html<&'static str> {
    Html("<h1>Diamonds imager is running!</h1>")
}

pub async fn create_account() -> Html<&'static str> {
    Html("<h1>create_account!</h1>")
}