//! Fusegu

use fusegu::{config::Config, server::create_app};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Application exit codes following Unix conventions
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // Some variants may be used for future error handling
enum ExitCode {
    /// Successful execution
    Success = 0,
    /// General application error
    GeneralError = 1,
    /// Configuration error (invalid config, missing .env, etc.)
    ConfigError = 2,
    /// Network binding error (port in use, permission denied, etc.)
    NetworkError = 3,
    /// Application initialization error
    InitializationError = 4,
    /// Unexpected server runtime error
    ServerError = 5,
}

impl ExitCode {
    /// Get the exit code description for logging
    fn description(self) -> &'static str {
        match self {
            ExitCode::Success => "Success",
            ExitCode::GeneralError => "General application error",
            ExitCode::ConfigError => "Configuration error",
            ExitCode::NetworkError => "Network binding error",
            ExitCode::InitializationError => "Application initialization error",
            ExitCode::ServerError => "Server runtime error",
        }
    }
}

/// Professional exit with specific code and no backtrace for operational errors
fn exit_gracefully(code: ExitCode) -> ! {
    tracing::info!(
        exit_code = code as i32,
        description = code.description(),
        "Application terminating"
    );
    std::process::exit(code as i32);
}

#[tokio::main]
async fn main() {
    // Disable backtraces by default for cleaner error output
    // Set RUST_BACKTRACE=1 or FUSEGU_DEBUG=1 to enable for debugging
    if std::env::var("RUST_BACKTRACE").is_err() && std::env::var("FUSEGU_DEBUG").is_err() {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "0");
        }
    }

    run_server().await;
}

async fn run_server() {
    // Load configuration from .env
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!();
            eprintln!("❌ Error: Failed to load configuration");
            eprintln!("   Reason: {}", e);
            eprintln!();
            eprintln!("💡 Solutions:");
            eprintln!("   1. Check your .env file format");
            eprintln!("   2. Verify environment variables are set correctly");
            eprintln!("   3. See env.sample for reference");
            eprintln!();
            exit_gracefully(ExitCode::ConfigError);
        },
    };

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
    let app = match create_app(config.clone()).await {
        Ok(app) => app,
        Err(e) => {
            tracing::error!(error = %e, "Failed to create application");
            eprintln!();
            eprintln!("❌ Error: Failed to initialize application");
            eprintln!("   Reason: {}", e);
            eprintln!();
            exit_gracefully(ExitCode::InitializationError);
        },
    };
    let addr = format!("{}:{}", config.server.host, config.server.port);

    // Try to bind to the address with better error handling
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            tracing::info!(address = %addr, "Server listening");
            listener
        },
        Err(e) => match e.kind() {
            std::io::ErrorKind::AddrInUse => {
                tracing::error!(
                    address = %addr,
                    "Address already in use. Another server may be running."
                );
                eprintln!();
                eprintln!("❌ Error: Address {} is already in use", addr);
                eprintln!();
                eprintln!("💡 Solutions:");
                eprintln!(
                    "   1. Kill existing process: lsof -i :{} | grep LISTEN",
                    config.server.port
                );
                eprintln!("   2. Use a different port: PORT=3000 cargo run");
                eprintln!("   3. Wait a moment and try again (port may be releasing)");
                eprintln!();
                exit_gracefully(ExitCode::NetworkError);
            },
            std::io::ErrorKind::PermissionDenied => {
                tracing::error!(
                    address = %addr,
                    "Permission denied. Cannot bind to this address."
                );
                eprintln!();
                eprintln!("❌ Error: Permission denied to bind to {}", addr);
                eprintln!();
                eprintln!("💡 Solutions:");
                eprintln!("   1. Use a port above 1024 (e.g., PORT=8080 cargo run)");
                eprintln!("   2. Run with elevated privileges if needed");
                eprintln!();
                exit_gracefully(ExitCode::NetworkError);
            },
            _ => {
                tracing::error!(
                    address = %addr,
                    error = %e,
                    "Failed to bind to address"
                );
                eprintln!();
                eprintln!("❌ Error: Failed to bind to address {}", addr);
                eprintln!("   Reason: {}", e);
                eprintln!();
                exit_gracefully(ExitCode::NetworkError);
            },
        },
    };

    // Start the server with graceful shutdown handling
    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    tracing::info!("🚀 Fusegu server started successfully");
    tracing::info!(
        "📋 OpenAPI documentation available at: http://{}/openapi.json",
        addr
    );
    tracing::info!("❤️  Health check available at: http://{}/health", addr);
    tracing::info!("🛑 Press Ctrl+C to shut down");

    if let Err(e) = server.await {
        tracing::error!(error = %e, "Server error occurred");
        eprintln!();
        eprintln!("❌ Error: Server encountered an unexpected error");
        eprintln!("   Reason: {}", e);
        eprintln!();
        eprintln!("💡 This may be a bug. Please check:");
        eprintln!("   1. Server logs for more details");
        eprintln!("   2. System resources (memory, file handles)");
        eprintln!("   3. Network connectivity");
        eprintln!();
        exit_gracefully(ExitCode::ServerError);
    }

    tracing::info!("🛑 Server shut down gracefully");
    exit_gracefully(ExitCode::Success);
}

/// Handle graceful shutdown on Ctrl+C
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("🛑 Received Ctrl+C, starting graceful shutdown...");
        },
        _ = terminate => {
            tracing::info!("🛑 Received terminate signal, starting graceful shutdown...");
        },
    }
}
