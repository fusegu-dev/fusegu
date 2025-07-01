use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Email address model for analysis and tracking
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct EmailAddress {
    pub id: Uuid,
    pub account_id: Uuid,
    pub email_hash: String, // Hash of email for privacy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_free_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disposable: Option<bool>,
    pub is_high_risk: bool,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub transaction_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub risk_score: Option<rust_decimal::Decimal>,
}

/// Email insights for detailed analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EmailInsights {
    pub is_free: bool,
    pub is_disposable: bool,
    pub is_high_risk: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<EmailDomainInsights>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EmailDomainInsights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<NaiveDate>,
}

impl EmailAddress {
    /// Check if email is from a known risky domain
    pub fn is_risky(&self) -> bool {
        self.is_high_risk || 
        self.is_disposable.unwrap_or(false)
    }

    /// Calculate email reputation score
    pub fn reputation_score(&self) -> f64 {
        let mut score: f32 = 80.0; // Start with good reputation
        
        if self.is_high_risk {
            score -= 40.0;
        }
        
        if self.is_disposable.unwrap_or(false) {
            score -= 30.0;
        }
        
        if self.is_free_provider.unwrap_or(false) {
            score -= 10.0;
        }
        
        // Bonus for established emails
        if self.transaction_count > 10 {
            score += 10.0;
        }
        
        score.max(0.0).min(100.0)
    }
} 