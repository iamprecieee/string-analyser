use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{SecondsFormat, Utc};
use serde_json::Value;

use crate::{
    models::{properties::AnalysedString, responses::ApiErrorResponse, state::AppState},
    utils::analyser::analyse_string,
};

pub async fn create_string(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    match payload.get("value") {
        Some(Value::String(s)) if !s.trim().is_empty() => {
            let properties = analyse_string(s);

            match state.repository.exists_by_id(&properties.sha256_hash).await {
                Ok(true) => {
                    return (
                        StatusCode::CONFLICT,
                        Json(ApiErrorResponse::conflict(
                            "string with this value already exists".to_string(),
                            None,
                        )),
                    )
                        .into_response();
                }

                Ok(false) => {}
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiErrorResponse::internal_error(
                            "A server error occurred. Try again later".to_string(),
                            None,
                        )),
                    )
                        .into_response();
                }
            }

            let created_at = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

            let analysed_string = AnalysedString {
                id: properties.sha256_hash.clone(),
                value: s.to_string(),
                properties,
                created_at,
            };

            match state.repository.create(&analysed_string).await {
                Ok(_) => (StatusCode::CREATED, Json(analysed_string)).into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::internal_error(
                        "A server error occurred. Try again later".to_string(),
                        None,
                    )),
                )
                    .into_response(),
            }
        }

        Some(Value::String(s)) if s.trim().is_empty() => (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorResponse::invalid_input(
                "String value cannot be empty".to_string(),
                None,
            )),
        )
            .into_response(),

        _ => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiErrorResponse::validaton_error(
                "Field 'value' must be a non-empty string".to_string(),
                None,
            )),
        )
            .into_response(),
    }
}
