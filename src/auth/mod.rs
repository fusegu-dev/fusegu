pub mod api_key;
pub mod middleware;

pub use api_key::*;
pub use middleware::*;

use crate::models::{Account, ApiKey};
use anyhow::Result;
use uuid::Uuid;

/// Authentication context containing account and API key information
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub account: Account,
    pub api_key: ApiKey,
}

impl AuthContext {
    pub fn new(account: Account, api_key: ApiKey) -> Self {
        Self { account, api_key }
    }

    /// Check if the authenticated account can access a specific feature
    pub fn can_access_feature(&self, feature: &str) -> bool {
        self.account.can_access_feature(feature) && 
        self.api_key.has_permission("read")
    }

    /// Check if the authenticated account can perform write operations
    pub fn can_write(&self) -> bool {
        self.api_key.has_permission("write")
    }

    /// Get account ID
    pub fn account_id(&self) -> Uuid {
        self.account.id
    }
} 