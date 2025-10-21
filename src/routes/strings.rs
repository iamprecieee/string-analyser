use axum::{
    Json,
    extract::{Path, Query, State, rejection::QueryRejection},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{SecondsFormat, Utc};
use serde_json::{Value, json};

use crate::{
    models::{
        filters::StringFilters,
        nlp::{InterpretedQuery, NlpResponse},
        properties::AnalysedString,
        requests::{CreateStringRequest, NlpQuery},
        responses::{ApiErrorResponse, GetStringsResponse},
        state::AppState,
    },
    utils::{
        analyser::{analyse_string, compute_sha256},
        nlp::parse_natural_language,
    },
};

#[utoipa::path(
    post,
    path = "/strings",
    request_body = CreateStringRequest,
    responses(
        (status = 201, description = "String created successfully", body = AnalysedString),
        (status = 400, description = "Bad request - missing value field", body = ApiErrorResponse),
        (status = 409, description = "Conflict - string already exists", body = ApiErrorResponse),
        (status = 422, description = "Unprocessable entity - invalid data type", body = ApiErrorResponse)
    ),
    tag = "Strings"
)]
pub async fn create_string(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    match payload.get("value") {
        Some(Value::String(s)) if !s.trim().is_empty() => {
            let properties = analyse_string(s.trim());

            match state.repository.exists_by_id(&properties.sha256_hash).await {
                Ok(true) => {
                    return (
                        StatusCode::CONFLICT,
                        Json(ApiErrorResponse::conflict(
                            "String already exists in the system".to_string(),
                            None,
                        )),
                    )
                        .into_response();
                }
                Ok(false) => {}
                Err(e) => {
                    tracing::error!("String existence check failed: {:?}", e);

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
                value: s.trim().to_string(),
                properties,
                created_at,
            };

            match state.repository.create(&analysed_string).await {
                Ok(_) => {
                    let cache_clone = state.cache.clone();
                    let analysed_string_clone = analysed_string.clone();

                    tokio::spawn(async move {
                        let _ = cache_clone.set(&analysed_string_clone).await;
                        let _ = cache_clone.invalidate().await;
                    });

                    (StatusCode::CREATED, Json(analysed_string)).into_response()
                }
                Err(e) => {
                    tracing::error!("String creation failed: {:?}", e);

                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiErrorResponse::internal_error(
                            "A server error occurred. Try again later".to_string(),
                            None,
                        )),
                    )
                        .into_response()
                }
            }
        }

        Some(Value::String(s)) if s.trim().is_empty() => (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorResponse::invalid_input(
                "Invalid request body of missing \"value\" field".to_string(),
                None,
            )),
        )
            .into_response(),

        _ => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiErrorResponse::validaton_error(
                "Invalid data type for \"value\"(must be string)".to_string(),
                None,
            )),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/strings/{string_value}",
    params(
        ("string_value" = String, Path, description = "The exact string value to retrieve")
    ),
    responses(
        (status = 200, description = "String found", body = AnalysedString),
        (status = 404, description = "String not found", body = ApiErrorResponse)
    ),
    tag = "Strings"
)]
pub async fn get_string(
    State(state): State<AppState>,
    Path(string_value): Path<String>,
) -> impl IntoResponse {
    let normalised_string_value = string_value.trim();

    let id = compute_sha256(&normalised_string_value);

    if let Ok(Some(analysed_string_cache)) = state.cache.get(&id).await {
        return (StatusCode::OK, Json(analysed_string_cache)).into_response();
    }

    match state
        .repository
        .get_by_value(&normalised_string_value)
        .await
    {
        Ok(Some(analysed_string)) => {
            let cache_clone = state.cache.clone();
            let analysed_string_clone = analysed_string.clone();

            tokio::spawn(async move {
                let _ = cache_clone.set(&analysed_string_clone).await;
            });
            (StatusCode::OK, Json(analysed_string)).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiErrorResponse::not_found(
                "String does not exist in the system".to_string(),
                None,
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("String retrieval failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::internal_error(
                    "A server error occurred. Try again later".to_string(),
                    None,
                )),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/strings",
    params(
        ("is_palindrome" = Option<bool>, Query, description = "Filter by palindrome status"),
        ("min_length" = Option<i32>, Query, description = "Minimum string length"),
        ("max_length" = Option<i32>, Query, description = "Maximum string length"),
        ("word_count" = Option<i32>, Query, description = "Exact word count"),
        ("contains_character" = Option<String>, Query, description = "Filter by character presence")
    ),
    responses(
        (status = 200, description = "List of strings matching filters", body = GetStringsResponse),
        (status = 400, description = "Invalid query parameters", body = ApiErrorResponse)
    ),
    tag = "Strings"
)]
pub async fn get_all_strings(
    state: State<AppState>,
    query: Query<StringFilters>,
) -> impl IntoResponse {
    let filters = query.0;

    match state.repository.filter(&filters).await {
        Ok(data) => {
            let count = data.len();
            let mut filters_applied = serde_json::to_value(&filters).unwrap_or(json!({}));

            if let Some(obj) = filters_applied.as_object_mut() {
                obj.retain(|_, v| !v.is_null());
            }

            (
                StatusCode::OK,
                Json(GetStringsResponse {
                    data,
                    count,
                    filters_applied,
                }),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("String filter-retrieval failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::internal_error(
                    "A server error occurred. Try again later".to_string(),
                    None,
                )),
            )
                .into_response()
        }
    }
}

pub async fn get_all_strings_wrapper(
    state: State<AppState>,
    query_result: Result<Query<StringFilters>, QueryRejection>,
) -> impl IntoResponse {
    match query_result {
        Ok(query) => get_all_strings(state, query).await.into_response(),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorResponse::invalid_input(
                "Invalid query parameter values or types".to_string(),
                None,
            )),
        )
            .into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/strings/{string_value}",
    params(
        ("string_value" = String, Path, description = "The exact string value to delete")
    ),
    responses(
        (status = 204, description = "String deleted successfully"),
        (status = 404, description = "String not found")
    ),
    tag = "Strings"
)]
pub async fn delete_string(
    State(state): State<AppState>,
    Path(string_value): Path<String>,
) -> impl IntoResponse {
    let normalised_string_value = string_value.trim();

    match state
        .repository
        .delete_by_value(&normalised_string_value)
        .await
    {
        Ok(true) => {
            let id = compute_sha256(&normalised_string_value);

            let cache_clone = state.cache.clone();
            tokio::spawn(async move {
                let _ = cache_clone.delete(&id).await;
                let _ = cache_clone.invalidate().await;
            });

            (StatusCode::NO_CONTENT).into_response()
        }
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ApiErrorResponse::not_found(
                "String does not exist in the system".to_string(),
                None,
            )),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("String deletion failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::internal_error(
                    "A server error occurred. Try again later".to_string(),
                    None,
                )),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/strings/filter-by-natural-language",
    params(
        ("query" = String, Query, description = "Natural language query string")
    ),
    responses(
        (status = 200, description = "Strings matching natural language query", body = NlpResponse),
        (status = 400, description = "Unable to parse query", body = ApiErrorResponse),
        (status = 422, description = "Conflicting filters detected", body = ApiErrorResponse)
    ),
    tag = "Strings"
)]
pub async fn get_by_natural_language(
    State(state): State<AppState>,
    Query(query): Query<NlpQuery>,
) -> impl IntoResponse {
    let parsed_query = match parse_natural_language(&query.query) {
        Ok(query) => query,
        Err(e) => {
            if e.contains("Conflicting") {
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ApiErrorResponse::conflict(
                        "Query parsed but resulted in conflicting filters".to_string(),
                        None,
                    )),
                )
                    .into_response();
            } else {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiErrorResponse::invalid_input(
                        "Unable to parse natural language query".to_string(),
                        None,
                    )),
                )
                    .into_response();
            }
        }
    };

    match state.repository.filter(&parsed_query.filters).await {
        Ok(data) => {
            let count = data.len();
            let mut filters_applied =
                serde_json::to_value(&parsed_query.filters).unwrap_or(json!({}));

            if let Some(obj) = filters_applied.as_object_mut() {
                obj.retain(|_, v| !v.is_null());
            }

            (
                StatusCode::OK,
                Json(NlpResponse {
                    data,
                    count,
                    interpreted_query: InterpretedQuery {
                        original: parsed_query.original,
                        parsed_filters: filters_applied,
                    },
                }),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("String nlp-retrieval failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::internal_error(
                    "A server error occurred. Try again later".to_string(),
                    None,
                )),
            )
                .into_response()
        }
    }
}
