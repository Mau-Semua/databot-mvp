mod config;
mod db;

use axum::{extract::State, Router, routing::get};
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;
use db::PgPool;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: Arc<PgPool>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logging / tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = config::load()?;
    tracing::info!("Loaded config: {:?}", cfg);

    let pool = db::create_pool(&cfg.database_url).await?;
    let state = AppState { db: Arc::new(pool) };

    let app = Router::new()
        .route("/health", get(health_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.app_port));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_handler(
    State(state): State<AppState>,
) -> Result<String, axum::http::StatusCode> {
    // Simple DB ping
    if let Err(e) = sqlx::query("SELECT 1")
        .execute(state.db.as_ref())
        .await
    {
        tracing::error!("DB health check failed: {:?}", e);
        return Err(axum::http::StatusCode::SERVICE_UNAVAILABLE);
    }

    Ok("ok".to_string())
}
