//! Fusegu

use fusegu::{config::Config, server::create_app};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration from .env
    let config = Config::load()?;

    // Initialize structured logging with JSON format for production
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "fusegu=info,tower_http=debug,axum::rejection=trace".to_string());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| log_level.into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        host = %config.server.host,
        port = config.server.port,
        environment = %config.server.environment,
        "Starting Fusegu API server"
    );

    // Create the application
    let app = create_app(config.clone()).await?;
    let addr = format!("{}:{}", config.server.host, config.server.port);

    tracing::info!(address = %addr, "Server listening");

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
}
