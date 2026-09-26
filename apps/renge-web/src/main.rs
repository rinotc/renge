mod app;
mod config;
mod error;
mod handlers;
mod presenters;
mod state;
mod templates;
pub mod usecase;

use crate::{
    config::{AppConfig, LogFormat},
    state::AppState,
};
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env()?;
    init_tracing(config.log_format);

    let app = app::router(AppState::from_env().await?, config.trust_proxy_headers);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("http://localhost:3000 で蓮華を起動しました");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

fn init_tracing(log_format: LogFormat) {
    let env_filter = || {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "renge_web=debug,tower_http=info".into())
    };

    match log_format {
        LogFormat::Pretty => tracing_subscriber::fmt()
            .with_env_filter(env_filter())
            .init(),
        LogFormat::Json => tracing_subscriber::fmt()
            .json()
            .with_env_filter(env_filter())
            .init(),
    }
}
