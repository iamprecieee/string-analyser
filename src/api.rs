use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    middleware::rate_limiter::rate_limit_middleware,
    models::{
        nlp::{InterpretedQuery, NlpResponse},
        properties::{AnalysedString, StringProperties},
        requests::CreateStringRequest,
        responses::{ApiErrorResponse, GetStringsResponse},
        state::AppState,
    },
    routes::strings::{
        create_string, delete_string, get_all_strings_wrapper, get_by_natural_language, get_string,
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::strings::create_string,
        crate::routes::strings::get_string,
        crate::routes::strings::get_all_strings,
        crate::routes::strings::get_by_natural_language,
        crate::routes::strings::delete_string,
    ),
    components(
        schemas(
            AnalysedString,
            StringProperties,
            CreateStringRequest,
            GetStringsResponse,
            NlpResponse,
            InterpretedQuery,
            ApiErrorResponse,
        )
    ),
    tags(
        (name = "Strings", description = "String analysis API endpoints")
    ),
    info(
        title = "String Analysis API",
        version = "1.0.0",
        description = "A REST API service that analyses strings and stores their computed properties"
    )
)]
pub struct ApiDoc;

pub async fn build_app(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/strings", post(create_string))
        .route("/strings", get(get_all_strings_wrapper))
        .route(
            "/strings/filter-by-natural-language",
            get(get_by_natural_language),
        )
        .route("/strings/{string_value}", get(get_string))
        .route("/strings/{string_value}", delete(delete_string))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware,
        ));

    Router::new()
        .route("/", get(health_check))
        .merge(api_routes)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "OK"}))).into_response()
}
