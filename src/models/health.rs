//! Health check data models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Health check response model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Current timestamp
    pub timestamp: DateTime<Utc>,
}
