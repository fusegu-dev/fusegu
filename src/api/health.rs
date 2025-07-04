//! Health check endpoint for monitoring and load balancers

use axum::Json;

use super::ApiResult;
use crate::models::health::HealthResponse;

/// Health check endpoint handler
#[utoipa::path(
    get,
    path = "/health",
    tags = ["Health"],
    summary = "Health Check",
    description = "Returns the current health status of the Fusegu API service. This endpoint is designed for use by load balancers, monitoring systems, and orchestration platforms.",
    responses(
        (status = 200, description = "Service is healthy and operational", body = HealthResponse,
            example = json!({
                "status": "healthy",
                "timestamp": "2025-01-21T10:00:00Z"
            })
        ),
        (status = 500, description = "Service is experiencing issues", body = crate::api::errors::ErrorResponse,
            example = json!({
                "error": "internal_error",
                "message": "Service temporarily unavailable"
            })
        )
    )
)]
pub async fn health_check() -> ApiResult<Json<HealthResponse>> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
    };
    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let result = health_check().await;
        assert!(result.is_ok());

        let response = result.unwrap().0;
        assert_eq!(response.status, "healthy");
    }
}
