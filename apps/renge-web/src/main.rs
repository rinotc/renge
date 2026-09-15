mod app;
mod error;
mod handlers;
mod state;
mod views;

use crate::state::AppState;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "renge=debug,tower_http=info".into()),
        )
        .init();

    let app = app::router(AppState::from_env().await?);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("http://localhost:3000 で蓮華を起動しました");
    axum::serve(listener, app).await?;
    Ok(())
}
