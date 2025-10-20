use std::net::SocketAddr;

use anyhow::{Ok, Result, anyhow};
use string_analyser::{
    api::build_app,
    db::{pool::create_pool, repositories::StringRepository},
    models::state::AppState,
    utils::config::load_config,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let config_data = load_config()?;

    let pool = create_pool(&config_data.0, config_data.1, config_data.2)
        .await
        .map_err(|e| anyhow!("Database connection error {}", e))?;

    let repository = StringRepository::new(pool);

    let state = AppState { repository };

    let app = build_app(state).await;

    let listener = TcpListener::bind("0.0.0.0:8001").await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .map_err(|e| anyhow!("Server error {}", e))?;

    Ok(())
}
