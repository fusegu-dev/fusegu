use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Risk level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl From<f64> for RiskLevel {
    fn from(score: f64) -> Self {
        match score {
            s if s < 10.0 => RiskLevel::Low,
            s if s < 30.0 => RiskLevel::Medium,
            s if s < 70.0 => RiskLevel::High,
            _ => RiskLevel::VeryHigh,
        }
    }
}

/// Transaction disposition recommendation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Disposition {
    Accept,
    Reject,
    Review,
    Test,
}

impl From<RiskLevel> for Disposition {
    fn from(risk_level: RiskLevel) -> Self {
        match risk_level {
            RiskLevel::Low => Disposition::Accept,
            RiskLevel::Medium => Disposition::Review,
            RiskLevel::High | RiskLevel::VeryHigh => Disposition::Reject,
        }
    }
}

/// Event types for transaction analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    AccountCreation,
    AccountLogin,
    EmailChange,
    PasswordReset,
    PayoutChange,
    Purchase,
    RecurringPurchase,
    Referral,
    Survey,
}

/// Subscription tiers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionTier {
    Free,
    Pro,
    Enterprise,
}

/// Credit card types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CreditCardType {
    Credit,
    Debit,
    Charge,
}

/// Address types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AddressType {
    Billing,
    Shipping,
}

/// Delivery speeds
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeliverySpeed {
    SameDay,
    Overnight,
    Expedited,
    Standard,
}

/// Payment processors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PaymentProcessor {
    Stripe,
    Square,
    Paypal,
    Braintree,
    Adyen,
    Worldpay,
    Other,
}

/// Links for HATEOAS support
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct Links {
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factors: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_analysis: Option<String>,
}



/// Pagination information
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Pagination {
    pub limit: i32,
    pub offset: i32,
    pub total: i64,
    pub has_more: bool,
}

/// Warning about data quality issues
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Warning {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_path: Option<String>,
} 