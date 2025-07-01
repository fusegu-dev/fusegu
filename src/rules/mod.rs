pub mod engine;
pub mod fraud_rules;
pub mod context;

pub use engine::*;
pub use fraud_rules::*;
pub use context::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Result of a fraud rule evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleHit {
    pub rule_name: String,
    pub score: f64,
    pub reason: String,
    pub metadata: Option<serde_json::Value>,
}

/// Priority level for fraud rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RulePriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Configuration for a fraud rule
#[derive(Debug, Clone)]
pub struct RuleConfig {
    pub name: String,
    pub enabled: bool,
    pub priority: RulePriority,
    pub weight: f64,
}

impl Default for RuleConfig {
    fn default() -> Self {
        Self {
            name: "unnamed_rule".to_string(),
            enabled: true,
            priority: RulePriority::Medium,
            weight: 1.0,
        }
    }
} 