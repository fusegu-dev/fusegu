use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use super::common::CreditCardType;

/// Credit card model for payment analysis
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CreditCard {
    pub id: Uuid,
    pub account_id: Uuid,
    pub card_hash: String, // Hash of BIN + last 4 digits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_id_number: Option<String>, // BIN (first 6-8 digits)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_digits: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<CreditCardType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>, // Country where card was issued
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_prepaid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_business: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_virtual: Option<bool>,
    pub transaction_count: i32,
    pub chargeback_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub risk_score: Option<rust_decimal::Decimal>,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

/// Credit card insights for detailed analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreditCardInsights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<CreditCardType>,
    pub is_business: bool,
    pub is_prepaid: bool,
    pub is_virtual: bool,
    pub is_issued_in_billing_address_country: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreditCardIssuerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreditCardIssuerInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub matches_provided_name: bool,
    pub matches_provided_phone_number: bool,
}

impl CreditCard {
    /// Check if card has risk indicators
    pub fn has_risk_indicators(&self) -> bool {
        self.chargeback_count > 0 ||
        self.is_prepaid.unwrap_or(false) ||
        self.risk_score.map_or(false, |score| {
            score.to_string().parse::<f64>().unwrap_or(0.0) > 50.0
        })
    }

    /// Calculate chargeback rate as percentage
    pub fn chargeback_rate(&self) -> f64 {
        if self.transaction_count == 0 {
            return 0.0;
        }
        (self.chargeback_count as f64 / self.transaction_count as f64) * 100.0
    }

    /// Get BIN (Bank Identification Number)
    pub fn bin(&self) -> Option<&str> {
        self.issuer_id_number.as_deref()
    }

    /// Check if card is from high-risk category
    pub fn is_high_risk_category(&self) -> bool {
        self.is_prepaid.unwrap_or(false) ||
        self.is_virtual.unwrap_or(false) ||
        self.chargeback_rate() > 5.0
    }
} 