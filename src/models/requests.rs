use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::net::IpAddr;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use super::common::{DeliverySpeed, EventType, PaymentProcessor};

/// Transaction request for fraud analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct TransactionRequest {
    pub device: DeviceRequest,
    pub event: EventRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<AddressRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingAddressRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<PaymentRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<CreditCardRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopping_cart: Option<Vec<CartItemRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_inputs: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct DeviceRequest {
    #[validate(ip)]
    #[schema(value_type = String)]
    pub ip_address: IpAddr,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 512))]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub accept_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 2147483647))]
    pub session_age: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct EventRequest {
    pub event_type: EventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub shop_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct AccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 32, max = 32))]
    pub username_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub user_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct EmailRequest {
    #[validate(length(max = 255))]
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub domain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct AddressRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub address_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 4))]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub postal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 2, max = 2))]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 4))]
    pub phone_country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct ShippingAddressRequest {
    #[serde(flatten)]
    pub address: AddressRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_speed: Option<DeliverySpeed>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct PaymentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<PaymentProcessor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_authorized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub decline_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreditCardRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 6, max = 8))]
    pub issuer_id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 2, max = 4))]
    pub last_digits: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub bank_phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 4))]
    pub bank_phone_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 2, max = 2))]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 1))]
    pub avs_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 1))]
    pub cvv_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_3d_secure_successful: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct OrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<f64>)]
    pub amount: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 3, max = 3))]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub discount_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub affiliate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub subaffiliate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url, length(max = 1024))]
    pub referrer_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_gift: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_gift_message: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct CartItemRequest {
    #[validate(length(max = 255))]
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub category: Option<String>,
    #[schema(value_type = f64)]
    pub price: rust_decimal::Decimal,
    #[validate(range(min = 0, max = 2147483647))]
    pub quantity: i32,
}

/// User creation request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct UserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub external_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub user_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonValue>,
}

/// User update request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct UserUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flagged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonValue>,
}

/// User analysis request for fraud assessment
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct UserAnalysisRequest {
    #[validate(length(min = 1))]
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_historical_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window_days: Option<u32>,
} 