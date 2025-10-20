use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde_json::json;

use crate::{
    models::state::AppState,
    routes::strings::{
        create_string, delete_string, get_all_strings_wrapper, get_by_natural_language, get_string,
    },
};

pub async fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/strings", post(create_string))
        .route("/strings", get(get_all_strings_wrapper))
        .route(
            "/strings/filter-by-natural-language",
            get(get_by_natural_language),
        )
        .route("/strings/{string_value}", get(get_string))
        .route("/strings/{string_value}", delete(delete_string))
        .with_state(state)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "OK"}))).into_response()
}
