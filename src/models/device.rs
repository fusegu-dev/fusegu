use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::net::IpAddr;
use utoipa::ToSchema;
use uuid::Uuid;

/// Device model for fingerprinting and tracking
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Device {
    pub id: Uuid,
    pub account_id: Uuid,
    pub fingerprint_hash: String,
    #[schema(value_type = String)]
    pub ip_address: IpAddr,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub transaction_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub risk_score: Option<rust_decimal::Decimal>,
    pub is_suspicious: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonValue>,
}

/// Device insights for detailed analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeviceInsights {
    pub id: Uuid,
    #[schema(value_type = f64)]
    pub confidence: rust_decimal::Decimal,
    pub last_seen: DateTime<Utc>,
    pub local_time: DateTime<Utc>,
}

impl Device {
    /// Check if device has been seen recently
    pub fn is_recently_active(&self) -> bool {
        let hours_since_last_seen = Utc::now()
            .signed_duration_since(self.last_seen)
            .num_hours();
        hours_since_last_seen < 24
    }

    /// Calculate device reputation score
    pub fn reputation_score(&self) -> f64 {
        if self.transaction_count == 0 {
            return 50.0; // Neutral for new devices
        }
        
        let base_score = if self.is_suspicious { 20.0 } else { 80.0 };
        let transaction_bonus = (self.transaction_count as f64 * 2.0).min(20.0);
        
        (base_score + transaction_bonus).min(100.0)
    }
} 