use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use super::common::{Disposition, EventType, Links, RiskLevel, Warning};
use super::{DeviceInsights, UserInfo};

/// Core transaction model for fraud analysis
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_transaction_id: Option<String>,
    pub event_type: EventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_id: Option<String>,
    
    // Risk assessment results
    #[schema(value_type = f64)]
    pub risk_score: rust_decimal::Decimal,
    pub risk_level: RiskLevel,
    pub disposition: Disposition,
    
    // Order information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub order_amount: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaffiliate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_gift: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_gift_message: Option<bool>,
    
    // Payment information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_processor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_authorized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_3d_secure_successful: Option<bool>,
    
    // Analysis metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_hits: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_inputs: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_request: Option<JsonValue>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// IP Address information for transaction analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IPAddressInfo {
    #[schema(value_type = f64)]
    pub risk: rust_decimal::Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<IPCountryInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<IPLocationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_reasons: Option<Vec<IPRiskReason>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<IPTraits>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IPCountryInfo {
    pub iso_code: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IPLocationInfo {
    #[schema(value_type = f64)]
    pub latitude: rust_decimal::Decimal,
    #[schema(value_type = f64)]
    pub longitude: rust_decimal::Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_radius: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IPRiskReason {
    pub code: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IPTraits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous_proxy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous_vpn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hosting_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public_proxy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_residential_proxy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tor_exit_node: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}

/// Risk score reason for detailed analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RiskScoreReason {
    #[schema(value_type = f64)]
    pub multiplier: rust_decimal::Decimal,
    pub reasons: Vec<RiskReasonDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RiskReasonDetail {
    pub code: String,
    pub reason: String,
}

impl Transaction {
    /// Create transaction from request
    pub fn from_request(request: &crate::models::requests::TransactionRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            account_id: Uuid::new_v4(), // Should be passed from context
            user_id: request.user_id,
            external_transaction_id: None,
            event_type: request.event.event_type.clone(),
            event_time: request.event.time,
            shop_id: request.event.shop_id.clone(),
            risk_score: rust_decimal::Decimal::ZERO,
            risk_level: RiskLevel::Low,
            disposition: Disposition::Accept,
            order_amount: request.order.as_ref().and_then(|o| o.amount),
            order_currency: request.order.as_ref().and_then(|o| o.currency.clone()),
            discount_code: request.order.as_ref().and_then(|o| o.discount_code.clone()),
            affiliate_id: request.order.as_ref().and_then(|o| o.affiliate_id.clone()),
            subaffiliate_id: request.order.as_ref().and_then(|o| o.subaffiliate_id.clone()),
            referrer_uri: request.order.as_ref().and_then(|o| o.referrer_uri.clone()),
            is_gift: request.order.as_ref().and_then(|o| o.is_gift),
            has_gift_message: request.order.as_ref().and_then(|o| o.has_gift_message),
            payment_processor: request.payment.as_ref().and_then(|p| p.processor.as_ref().map(|pr| format!("{:?}", pr))),
            was_authorized: request.payment.as_ref().and_then(|p| p.was_authorized),
            decline_code: request.payment.as_ref().and_then(|p| p.decline_code.clone()),
            avs_result: request.credit_card.as_ref().and_then(|cc| cc.avs_result.clone()),
            cvv_result: request.credit_card.as_ref().and_then(|cc| cc.cvv_result.clone()),
            was_3d_secure_successful: request.credit_card.as_ref().and_then(|cc| cc.was_3d_secure_successful),
            rule_hits: None,
            custom_inputs: request.custom_inputs.clone(),
            warnings: None,
            raw_request: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Get risk score as f64 for calculations
    pub fn risk_score_f64(&self) -> f64 {
        self.risk_score.to_string().parse().unwrap_or(0.0)
    }

    /// Check if transaction is high risk
    pub fn is_high_risk(&self) -> bool {
        matches!(self.risk_level, RiskLevel::High | RiskLevel::VeryHigh)
    }

    /// Check if transaction should be automatically rejected
    pub fn should_reject(&self) -> bool {
        matches!(self.disposition, Disposition::Reject)
    }
} 