use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Shopping cart item model for transaction analysis
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CartItem {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[schema(value_type = f64)]
    pub price: rust_decimal::Decimal,
    pub quantity: i32,
}

impl CartItem {
    /// Calculate total value for this item
    pub fn total_value(&self) -> rust_decimal::Decimal {
        self.price * rust_decimal::Decimal::from(self.quantity)
    }

    /// Calculate total value as f64 for calculations
    pub fn total_value_f64(&self) -> f64 {
        self.total_value().to_string().parse().unwrap_or(0.0)
    }

    /// Check if item has unusual quantity
    pub fn has_unusual_quantity(&self) -> bool {
        self.quantity > 100 || self.quantity <= 0
    }

    /// Check if item price seems unusual
    pub fn has_unusual_price(&self) -> bool {
        let price_f64 = self.price.to_string().parse::<f64>().unwrap_or(0.0);
        price_f64 > 10000.0 || price_f64 <= 0.0
    }
} 