use std::net::SocketAddr;

use anyhow::{anyhow, Ok, Result};
use string_analyser::{
    api::build_app,
    cache::{connection::create_redis_client, service::CacheService},
    db::{pool::create_pool, repositories::StringRepository},
    models::state::AppState,
    utils::config::load_config,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let config_data = load_config()?;

    let pool = create_pool(&config_data.0, *&config_data.2, *&config_data.3)
        .await
        .map_err(|e| anyhow!("Database connection error {}", e))?;

    let redis = create_redis_client(&config_data.1)
        .await
        .map_err(|e| anyhow!("Redis connection error {}", e))?;

    let repository = StringRepository::new(pool);

    let cache = CacheService::new(redis);

    let state = AppState { repository, cache };

    let app = build_app(state).await;

    let listener = TcpListener::bind(format!("{}:{}", &config_data.4, &config_data.5))
        .await
        .unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .map_err(|e| anyhow!("Server error {}", e))?;

    Ok(())
}
