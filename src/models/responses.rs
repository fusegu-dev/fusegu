use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use utoipa::ToSchema;
use uuid::Uuid;

use super::common::{Disposition, Links, Pagination, RiskLevel, SubscriptionTier, Warning};
use super::transaction::{IPAddressInfo, RiskScoreReason};
use super::{
    AddressInsights, CreditCardInsights, DeviceInsights, EmailInsights, PhoneInsights, UserInfo,
};

/// Transaction response after fraud analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponse {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[schema(value_type = f64)]
    pub risk_score: rust_decimal::Decimal,
    pub risk_level: RiskLevel,
    pub disposition: Disposition,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<IPAddressInfo>,
    #[schema(value_type = f64)]
    pub funds_remaining: rust_decimal::Decimal,
    pub queries_remaining: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
    #[serde(rename = "_links")]
    pub links: Links,
}

/// Detailed transaction insights (Pro+ plans)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransactionInsights {
    #[serde(flatten)]
    pub transaction: TransactionResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CreditCardInsights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceInsights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailInsights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<AddressInsights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<AddressInsights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_phone: Option<PhoneInsights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_phone: Option<PhoneInsights>,
}

/// Detailed risk factors analysis (Enterprise plans)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransactionFactors {
    #[serde(flatten)]
    pub insights: TransactionInsights,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score_reasons: Option<Vec<RiskScoreReason>>,
}

/// Transaction list response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransactionList {
    pub transactions: Vec<TransactionResponse>,
    pub pagination: Pagination,
    #[serde(rename = "_links")]
    pub links: Links,
}

/// User response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
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
    #[serde(rename = "_links")]
    pub links: Links,
}

/// User list response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserList {
    pub users: Vec<UserResponse>,
    pub pagination: Pagination,
    #[serde(rename = "_links")]
    pub links: Links,
}

/// User risk analysis response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserRiskAnalysis {
    #[serde(flatten)]
    pub user: UserResponse,
    pub behavioral_patterns: BehavioralPatterns,
    pub velocity_analysis: VelocityAnalysis,
    pub cross_transaction_factors: Vec<CrossTransactionFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BehavioralPatterns {
    pub transaction_frequency: TransactionFrequency,
    pub transaction_amounts: TransactionAmounts,
    pub device_consistency: DeviceConsistency,
    pub location_patterns: LocationPatterns,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransactionFrequency {
    #[schema(value_type = f64)]
    pub daily_average: rust_decimal::Decimal,
    #[schema(value_type = f64)]
    pub weekly_average: rust_decimal::Decimal,
    pub peak_day: String,
    pub peak_hour: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransactionAmounts {
    #[schema(value_type = f64)]
    pub average_amount: rust_decimal::Decimal,
    #[schema(value_type = f64)]
    pub median_amount: rust_decimal::Decimal,
    #[schema(value_type = f64)]
    pub largest_amount: rust_decimal::Decimal,
    #[schema(value_type = f64)]
    pub amount_variance: rust_decimal::Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeviceConsistency {
    pub unique_devices: i32,
    #[schema(value_type = f64)]
    pub primary_device_usage: rust_decimal::Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LocationPatterns {
    pub unique_countries: i32,
    pub primary_country: String,
    #[schema(value_type = f64)]
    pub location_consistency: rust_decimal::Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VelocityAnalysis {
    pub transactions_last_24h: i32,
    pub transactions_last_7d: i32,
    pub transactions_last_30d: i32,
    #[schema(value_type = f64)]
    pub amount_last_24h: rust_decimal::Decimal,
    #[schema(value_type = f64)]
    pub amount_last_7d: rust_decimal::Decimal,
    #[schema(value_type = f64)]
    pub amount_last_30d: rust_decimal::Decimal,
    pub velocity_flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CrossTransactionFactor {
    pub factor_type: String,
    #[schema(value_type = f64)]
    pub factor_value: rust_decimal::Decimal,
    pub description: String,
    pub risk_impact: RiskImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RiskImpact {
    Positive,
    Negative,
    Neutral,
}

/// Account information response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AccountInfo {
    pub account_id: String,
    pub subscription_tier: SubscriptionTier,
    #[schema(value_type = f64)]
    pub funds_remaining: rust_decimal::Decimal,
    pub monthly_quota: i32,
    pub queries_used_this_month: i32,
    pub queries_remaining_this_month: i32,
    pub billing_cycle_start: String,
    pub billing_cycle_end: String,
}

/// Analytics response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Analytics {
    pub period: AnalyticsPeriod,
    pub summary: AnalyticsSummary,
    pub time_series: Vec<AnalyticsTimePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnalyticsPeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnalyticsSummary {
    pub total_transactions: i64,
    pub total_users: i64,
    #[schema(value_type = f64)]
    pub average_risk_score: rust_decimal::Decimal,
    pub risk_distribution: RiskDistribution,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_risk_distribution: Option<RiskDistribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RiskDistribution {
    pub low: i64,
    pub medium: i64,
    pub high: i64,
    pub very_high: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnalyticsTimePoint {
    pub timestamp: DateTime<Utc>,
    pub transaction_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i64>,
    #[schema(value_type = f64)]
    pub average_risk_score: rust_decimal::Decimal,
    pub disposition_counts: DispositionCounts,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DispositionCounts {
    pub accept: i64,
    pub reject: i64,
    pub review: i64,
}

/// Error response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<JsonValue>,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ValidationError {
    pub field: String,
    pub code: String,
    pub message: String,
}

/// Transaction analysis response for fraud detection
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransactionAnalysisResponse {
    pub transaction_id: Uuid,
    #[schema(value_type = f64)]
    pub risk_score: rust_decimal::Decimal,
    pub risk_level: RiskLevel,
    pub disposition: Disposition,
    pub rule_hits: Vec<String>,
    pub risk_factors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
    pub created_at: DateTime<Utc>,
    #[serde(rename = "_links")]
    pub links: Links,
}

impl TransactionAnalysisResponse {
    pub fn from_analysis(
        transaction: crate::models::Transaction,
        risk_score: f64,
        rule_results: Vec<String>,
    ) -> Self {
        Self {
            transaction_id: transaction.id,
            risk_score: rust_decimal::Decimal::from_f64_retain(risk_score).unwrap_or_default(),
            risk_level: RiskLevel::from(risk_score),
            disposition: Disposition::from(RiskLevel::from(risk_score)),
            rule_hits: rule_results,
            risk_factors: vec![], // Would be populated with actual risk factors
            warnings: None,
            created_at: transaction.created_at,
            links: Links {
                self_link: None,
                self_ref: None,
                next: None,
                prev: None,
                first: None,
                last: None,
                insights: None,
                factors: None,
                report: None,
            },
        }
    }
}

/// User analysis response for fraud assessment  
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserAnalysisResponse {
    pub user_id: String,
    #[schema(value_type = f64)]
    pub risk_score: rust_decimal::Decimal,
    pub risk_level: RiskLevel,
    pub risk_indicators: Vec<String>,
    pub behavior_analysis: BehavioralPatterns,
    pub velocity_analysis: VelocityAnalysis,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
    pub analyzed_at: DateTime<Utc>,
    #[serde(rename = "_links")]
    pub links: Links,
}

impl UserAnalysisResponse {
    pub fn from_analysis(user: crate::models::User, risk_indicators: Vec<String>) -> Self {
        Self {
            user_id: user.external_user_id.unwrap_or_else(|| user.id.to_string()),
            risk_score: user.risk_score.unwrap_or_default(),
            risk_level: user.risk_level.unwrap_or(RiskLevel::Low),
            risk_indicators,
            behavior_analysis: BehavioralPatterns {
                transaction_frequency: TransactionFrequency {
                    daily_average: rust_decimal::Decimal::ZERO,
                    weekly_average: rust_decimal::Decimal::ZERO,
                    peak_day: "Monday".to_string(),
                    peak_hour: 12,
                },
                transaction_amounts: TransactionAmounts {
                    average_amount: rust_decimal::Decimal::ZERO,
                    median_amount: rust_decimal::Decimal::ZERO,
                    largest_amount: rust_decimal::Decimal::ZERO,
                    amount_variance: rust_decimal::Decimal::ZERO,
                },
                device_consistency: DeviceConsistency {
                    unique_devices: 1,
                    primary_device_usage: rust_decimal::Decimal::from(100),
                },
                location_patterns: LocationPatterns {
                    unique_countries: 1,
                    primary_country: "US".to_string(),
                    location_consistency: rust_decimal::Decimal::from(100),
                },
            },
            velocity_analysis: VelocityAnalysis {
                transactions_last_24h: 0,
                transactions_last_7d: 0,
                transactions_last_30d: 0,
                amount_last_24h: rust_decimal::Decimal::ZERO,
                amount_last_7d: rust_decimal::Decimal::ZERO,
                amount_last_30d: rust_decimal::Decimal::ZERO,
                velocity_flags: vec![],
            },
            warnings: None,
            analyzed_at: Utc::now(),
            links: Links {
                self_link: None,
                self_ref: None,
                next: None,
                prev: None,
                first: None,
                last: None,
                insights: None,
                factors: None,
                report: None,
            },
        }
    }
} 