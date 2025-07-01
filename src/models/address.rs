use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use super::common::{AddressType, DeliverySpeed};

/// Address model for verification and analysis
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Address {
    pub id: Uuid,
    pub account_id: Uuid,
    pub address_hash: String, // Hash of normalized address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>, // ISO 3166-2 subdivision code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>, // ISO 3166-1 alpha-2 country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub latitude: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub longitude: Option<rust_decimal::Decimal>,
    pub is_high_risk: bool,
    pub transaction_count: i32,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

/// Address insights for detailed analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddressInsights {
    pub is_postal_in_city: bool,
    pub is_in_ip_country: bool,
    pub is_high_risk: bool,
    #[schema(value_type = f64)]
    pub distance_to_ip_location: rust_decimal::Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub distance_to_billing_address: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub latitude: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub longitude: Option<rust_decimal::Decimal>,
}

/// Phone insights for detailed analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PhoneInsights {
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_operator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_type: Option<String>,
    pub is_voip: bool,
    pub matches_postal: bool,
}

impl Address {
    /// Check if address has complete information
    pub fn is_complete(&self) -> bool {
        self.address_line_1.is_some() &&
        self.city.is_some() &&
        self.postal_code.is_some() &&
        self.country.is_some()
    }

    /// Get full name if available
    pub fn full_name(&self) -> Option<String> {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(first), None) => Some(first.clone()),
            (None, Some(last)) => Some(last.clone()),
            (None, None) => None,
        }
    }

    /// Calculate distance from another address (simplified calculation)
    pub fn distance_to(&self, other: &Address) -> Option<f64> {
        match (
            (&self.latitude, &self.longitude),
            (&other.latitude, &other.longitude)
        ) {
            ((Some(lat1), Some(lon1)), (Some(lat2), Some(lon2))) => {
                // Simple haversine distance calculation would go here
                // For now, return a placeholder
                Some(0.0) // TODO: Implement proper distance calculation
            }
            _ => None,
        }
    }
} 