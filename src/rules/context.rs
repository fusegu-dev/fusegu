use crate::models::{Device, Transaction, User, requests::TransactionRequest};
use std::sync::Arc;

/// Context provided to fraud rules during evaluation
/// Contains all relevant data for fraud detection analysis
#[derive(Debug)]
pub struct RuleContext {
    /// Original transaction request
    pub transaction: Arc<TransactionRequest>,
    
    /// User associated with the transaction (if any)
    pub user: Option<Arc<User>>,
    
    /// Device information for the transaction
    pub device: Option<Arc<Device>>,
    
    /// Feature store for accessing historical data and computed features
    pub feature_store: Arc<dyn FeatureStore>,
    
    /// Additional context data
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl RuleContext {
    pub fn new(
        transaction: Arc<TransactionRequest>,
        user: Option<Arc<User>>,
        device: Option<Arc<Device>>,
        feature_store: Arc<dyn FeatureStore>,
    ) -> Self {
        Self {
            transaction,
            user,
            device,
            feature_store,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Add metadata to the context
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get order amount if available
    pub fn order_amount(&self) -> Option<f64> {
        self.transaction.order.as_ref()
            .and_then(|order| order.amount)
            .map(|amount| amount.to_string().parse().unwrap_or(0.0))
    }

    /// Get user ID if available
    pub fn user_id(&self) -> Option<uuid::Uuid> {
        self.user.as_ref().map(|user| user.id)
    }

    /// Get device IP address
    pub fn ip_address(&self) -> std::net::IpAddr {
        self.transaction.device.ip_address
    }

    /// Get email address if available
    pub fn email_address(&self) -> Option<&str> {
        self.transaction.email.as_ref().map(|email| email.address.as_str())
    }
}

/// Feature store trait for accessing historical data and computed features
/// This is the interface between rules and the data layer
#[async_trait::async_trait]
pub trait FeatureStore: Send + Sync {
    /// Get user transaction count in the specified time window (hours)
    async fn get_user_transaction_count(&self, user_id: uuid::Uuid, window_hours: u32) -> anyhow::Result<u32>;
    
    /// Get total transaction amount for user in the specified time window (hours)
    async fn get_user_transaction_amount(&self, user_id: uuid::Uuid, window_hours: u32) -> anyhow::Result<f64>;
    
    /// Get IP address transaction count in the specified time window (minutes)
    async fn get_ip_transaction_count(&self, ip: std::net::IpAddr, window_minutes: u32) -> anyhow::Result<u32>;
    
    /// Get email domain risk score
    async fn get_email_domain_risk(&self, domain: &str) -> anyhow::Result<f64>;
    
    /// Check if IP is from anonymous proxy/VPN/Tor
    async fn is_anonymous_ip(&self, ip: std::net::IpAddr) -> anyhow::Result<bool>;
    
    /// Get IP geolocation country code
    async fn get_ip_country(&self, ip: std::net::IpAddr) -> anyhow::Result<Option<String>>;
    
    /// Get device transaction count in the specified time window (hours)
    async fn get_device_transaction_count(&self, device_id: uuid::Uuid, window_hours: u32) -> anyhow::Result<u32>;
    
    /// Get address transaction count in the specified time window (hours)
    async fn get_address_transaction_count(&self, address_hash: &str, window_hours: u32) -> anyhow::Result<u32>;
    
    /// Get user's average transaction amount
    async fn get_user_average_amount(&self, user_id: uuid::Uuid) -> anyhow::Result<f64>;
    
    /// Get card BIN transaction count in the specified time window (hours)
    async fn get_card_bin_transaction_count(&self, bin: &str, window_hours: u32) -> anyhow::Result<u32>;
    
    /// Check if card is prepaid
    async fn is_prepaid_card(&self, bin: &str) -> anyhow::Result<bool>;
    
    /// Get distance between two locations (in kilometers)
    async fn calculate_distance(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> anyhow::Result<f64>;
} 