use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;

use crate::{models::state::AppState, routes::strings::create_string};

pub async fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/strings", post(create_string))
        .with_state(state)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "OK"}))).into_response()
}
