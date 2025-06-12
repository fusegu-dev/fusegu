//! HTTP server and routing

use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, Method, header},
    middleware::Next,
    response::Response,
    routing::get,
};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};

use crate::{api::health::health_check, config::Config};

/// Create the main application with routes and middleware
pub async fn create_app(config: Config) -> anyhow::Result<Router> {
    // CORS for browser frontend
    let mut cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT]);

    // Add each origin individually
    for origin in &config.cors.origins {
        if let Ok(header_value) = origin.parse::<HeaderValue>() {
            cors = cors.allow_origin(header_value);
        }
    }

    // Create the main router
    let app = Router::new()
        // Single health endpoint - all you need for MVP
        .route("/health", get(health_check))
        // API v1 routes
        .nest("/v1", api_v1_routes())
        // Root endpoint
        .route("/", get(root_handler))
        // Add shared state
        .with_state(config.clone())
        // Middleware stack for browser frontend
        .layer(
            ServiceBuilder::new()
                // Request tracing for debugging
                .layer(TraceLayer::new_for_http())
                // Request timeout for reliability
                .layer(TimeoutLayer::new(Duration::from_secs(
                    config.server.request_timeout_seconds,
                )))
                // Security headers (important for browsers)
                .layer(axum::middleware::from_fn(security_headers))
                // CORS (required for browser frontend)
                .layer(cors),
        );

    Ok(app)
}

/// API v1 routes
fn api_v1_routes() -> Router<Config> {
    Router::new().route("/health", get(health_check))
    // Future API endpoints will be added here
}

/// Root handler
async fn root_handler() -> &'static str {
    "Fusegu API"
}

/// Security headers middleware
async fn security_headers(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();

    // Prevent MIME type sniffing
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );

    // Prevent embedding in frames (clickjacking protection)
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));

    // XSS protection
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );

    // HSTS header for production
    headers.insert(
        "Strict-Transport-Security",
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );

    // Hide server information
    headers.remove("server");

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let config = Config::default();
        let app = create_app(config).await.unwrap();

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_root_endpoint() {
        let config = Config::default();
        let app = create_app(config).await.unwrap();

        let request = Request::builder().uri("/").body(Body::empty()).unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), 200);
    }
}
