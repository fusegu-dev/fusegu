use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use super::common::{Links, RiskLevel};

/// User model for cross-transaction tracking and risk analysis
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub account_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub risk_score: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<RiskLevel>,
    pub total_transactions: i32,
    pub successful_transactions: i32,
    pub failed_transactions: i32,
    pub chargeback_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_transaction_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transaction_at: Option<DateTime<Utc>>,
    pub is_verified: bool,
    pub is_flagged: bool,
    pub flags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User information included in transaction responses
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserInfo {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub risk_score: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<RiskLevel>,
    pub total_transactions: i32,
    pub is_flagged: bool,
}

impl From<&User> for UserInfo {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            external_user_id: user.external_user_id.clone(),
            risk_score: user.risk_score,
            risk_level: user.risk_level,
            total_transactions: user.total_transactions,
            is_flagged: user.is_flagged,
        }
    }
}

impl User {
    /// Calculate success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_transactions == 0 {
            return 100.0;
        }
        (self.successful_transactions as f64 / self.total_transactions as f64) * 100.0
    }

    /// Check if user has concerning patterns
    pub fn has_risk_indicators(&self) -> bool {
        self.is_flagged || 
        self.chargeback_count > 0 ||
        self.success_rate() < 50.0
    }
} 