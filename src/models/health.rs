//! Health check data models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

/// Health check response model
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "HealthResponse",
    description = "Health check endpoint response",
    example = json!({
        "status": "healthy",
        "timestamp": "2025-01-21T10:00:00Z"
    })
)]
pub struct HealthResponse {
    /// Service health status
    #[schema(example = "healthy")]
    pub status: String,
    /// Response timestamp in ISO 8601 format
    #[schema(example = "2025-01-21T10:00:00Z")]
    pub timestamp: DateTime<Utc>,
}
