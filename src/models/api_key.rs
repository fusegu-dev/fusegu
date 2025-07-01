use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// API Key model for authentication
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ApiKey {
    pub id: Uuid,
    pub account_id: Uuid,
    pub key_hash: String,
    pub name: String,
    pub permissions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ApiKey {
    /// Check if API key is valid for use
    pub fn is_valid(&self) -> bool {
        self.is_active && 
        self.expires_at.map_or(true, |exp| exp > Utc::now())
    }

    /// Check if API key has specific permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string()) || 
        self.permissions.contains(&"admin".to_string())
    }
} 