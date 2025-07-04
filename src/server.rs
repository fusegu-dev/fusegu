//! HTTP server and routing

use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, Method, header},
    middleware::Next,
    response::Response,
    routing::{get, post, patch},
};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{
        errors::ErrorResponse,
        health::health_check,
        transactions::{create_transaction, get_transaction, list_transactions, TransactionListQuery},
        users::{create_user, get_user, list_users, update_user, UserListQuery},
        account::get_account,
    },
    auth::middleware::auth_middleware,
    models::{
        requests::{TransactionRequest, UserRequest, UserUpdate},
        responses::{TransactionResponse, TransactionList, UserResponse, UserList, AccountInfo},
        health::HealthResponse,
    },
    services::TransactionService,
    config::Config,
};

/// OpenAPI documentation for Fusegu API
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::health::health_check,
        crate::api::transactions::create_transaction,
        crate::api::transactions::get_transaction,
        crate::api::transactions::list_transactions,
        crate::api::users::create_user,
        crate::api::users::get_user,
        crate::api::users::list_users,
        crate::api::users::update_user,
        crate::api::account::get_account,
    ),
    components(
        schemas(
            TransactionRequest,
            TransactionResponse,
            TransactionList,
            TransactionListQuery,
            UserRequest,
            UserResponse,
            UserList,
            UserListQuery,
            UserUpdate,
            AccountInfo,
            HealthResponse,
            ErrorResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoint"),
        (name = "Transactions", description = "Transaction fraud detection and risk assessment"),
        (name = "Users", description = "User profile management and cross-transaction risk analysis"),
        (name = "Account", description = "Account management and usage information"),
    ),
    info(
        title = "Fusegu API", 
        version = "1.0.0",
        description = "A comprehensive RESTful API for transaction fraud detection and risk assessment with advanced user tracking and cross-transaction analysis capabilities."
    )
)]
pub struct ApiDoc;

/// Create the main application with routes and middleware
pub async fn create_app(config: Config) -> anyhow::Result<Router> {
    // CORS for browser frontend
    let mut cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH])
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
        // Swagger UI - merge the SwaggerUi router (handles /openapi.json internally)
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
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
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // Transaction endpoints
        .route("/transactions", post(create_transaction))
        .route("/transactions", get(list_transactions))
        .route("/transactions/{transaction_id}", get(get_transaction))
        // User endpoints
        .route("/users", post(create_user))
        .route("/users", get(list_users))
        .route("/users/{user_id}", get(get_user))
        .route("/users/{user_id}", patch(update_user))
        // Account endpoints
        .route("/account", get(get_account))
        // TODO: Add authentication middleware when implemented
        // .layer(axum::middleware::from_fn_with_state(config.clone(), auth_middleware))
}

/// Root handler
async fn root_handler() -> &'static str {
    "Fusegu API v1.0.0 - Transaction Fraud Detection and Risk Assessment"
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
