use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use super::common::SubscriptionTier;

/// Account model for multi-tenant architecture
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub subscription_tier: SubscriptionTier,
    #[schema(value_type = f64)]
    pub funds_remaining: rust_decimal::Decimal,
    pub monthly_quota: i32,
    pub queries_used_this_month: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_start: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_end: Option<NaiveDate>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Account {
    /// Check if account has remaining queries
    pub fn has_remaining_queries(&self) -> bool {
        self.queries_used_this_month < self.monthly_quota
    }

    /// Get remaining queries count
    pub fn queries_remaining(&self) -> i32 {
        (self.monthly_quota - self.queries_used_this_month).max(0)
    }

    /// Check if account can perform operation based on subscription tier
    pub fn can_access_feature(&self, feature: &str) -> bool {
        match (feature, &self.subscription_tier) {
            ("insights", SubscriptionTier::Pro | SubscriptionTier::Enterprise) => true,
            ("factors", SubscriptionTier::Enterprise) => true,
            ("webhooks", SubscriptionTier::Pro | SubscriptionTier::Enterprise) => true,
            ("batch_processing", SubscriptionTier::Pro | SubscriptionTier::Enterprise) => true,
            _ => true, // Basic features available to all tiers
        }
    }
} 