//! Health check endpoint for monitoring and load balancers

use axum::Json;

use super::ApiResult;

/// Health check endpoint handler
pub async fn health_check() -> ApiResult<Json<serde_json::Value>> {
    use crate::models::HealthResponse;
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
    };
    Ok(Json(serde_json::to_value(response)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let result = health_check().await;
        assert!(result.is_ok());

        let response = result.unwrap().0;
        assert_eq!(response["status"], "healthy");
    }
}
