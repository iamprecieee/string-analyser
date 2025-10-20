use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use chrono::Utc;

use crate::models::{responses::ApiErrorResponse, state::AppState};

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let ip_addr = get_client_ip(&request);

    let current_minute = Utc::now().timestamp() / 60;

    let cache_key = format!("ratelimit:{}:{}", ip_addr, current_minute);

    let mut redis_cache = state.cache.clone_redis();

    let limit = 20;

    let script = r#"
        local current = redis.call('INCR', KEYS[1])
        if current == 1 then
            redis.call('EXPIRE', KEYS[1], ARGV[1])
        end
        return current
    "#;

    let count: i32 = match redis::Script::new(script)
        .key(&cache_key)
        .arg(60)
        .invoke_async(&mut redis_cache)
        .await
    {
        Ok(val) => val,
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
    };

    if count >= limit {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(ApiErrorResponse::throttled(
                "Rate limit exceeded".to_string(),
                None,
            )),
        )
            .into_response();
    }

    next.run(request).await
}

fn get_client_ip(request: &Request) -> String {
    request
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .unwrap_or("unknown")
        .to_string()
}
