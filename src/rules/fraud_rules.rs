use async_trait::async_trait;
use super::{RuleContext, RuleHit, RulePriority};
use anyhow::Result;

/// Trait that all fraud rules must implement
#[async_trait]
pub trait FraudRule: Send + Sync + std::fmt::Debug {
    /// Get the name of this rule
    fn name(&self) -> &str;
    
    /// Get the priority of this rule
    fn priority(&self) -> RulePriority;
    
    /// Evaluate this rule against the given context
    /// Returns Some(RuleHit) if the rule triggers, None if it passes
    async fn evaluate(&self, context: &RuleContext) -> Result<Option<RuleHit>>;
}

/// A simple high-risk IP rule
#[derive(Debug)]
pub struct HighRiskIpRule;

#[async_trait]
impl FraudRule for HighRiskIpRule {
    fn name(&self) -> &str {
        "high_risk_ip"
    }
    
    fn priority(&self) -> RulePriority {
        RulePriority::High
    }
    
    async fn evaluate(&self, context: &RuleContext) -> Result<Option<RuleHit>> {
        let is_anonymous = context.feature_store.is_anonymous_ip(context.ip_address()).await?;
        
        if is_anonymous {
            Ok(Some(RuleHit {
                rule_name: self.name().to_string(),
                score: 50.0,
                reason: "Transaction from anonymous IP (VPN/Proxy/Tor)".to_string(),
                metadata: Some(serde_json::json!({
                    "ip_address": context.ip_address().to_string()
                })),
            }))
        } else {
            Ok(None)
        }
    }
}

/// A velocity rule for transaction frequency
#[derive(Debug)]
pub struct VelocityRule;

#[async_trait]
impl FraudRule for VelocityRule {
    fn name(&self) -> &str {
        "velocity_check"
    }
    
    fn priority(&self) -> RulePriority {
        RulePriority::Critical
    }
    
    async fn evaluate(&self, context: &RuleContext) -> Result<Option<RuleHit>> {
        if let Some(user) = &context.user {
            let count = context.feature_store.get_user_transaction_count(user.id, 1).await?;
            
            if count > 10 {
                Ok(Some(RuleHit {
                    rule_name: self.name().to_string(),
                    score: 80.0,
                    reason: format!("User has {} transactions in the last hour", count),
                    metadata: Some(serde_json::json!({
                        "transaction_count": count,
                        "time_window": "1_hour"
                    })),
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

/// A rule for suspicious amounts
#[derive(Debug)]
pub struct SuspiciousAmountRule;

#[async_trait]
impl FraudRule for SuspiciousAmountRule {
    fn name(&self) -> &str {
        "suspicious_amount"
    }
    
    fn priority(&self) -> RulePriority {
        RulePriority::Medium
    }
    
    async fn evaluate(&self, context: &RuleContext) -> Result<Option<RuleHit>> {
        if let Some(amount) = context.order_amount() {
            if amount > 10000.0 {
                Ok(Some(RuleHit {
                    rule_name: self.name().to_string(),
                    score: 30.0,
                    reason: "High transaction amount".to_string(),
                    metadata: Some(serde_json::json!({
                        "amount": amount
                    })),
                }))
            } else if amount == 0.0 {
                Ok(Some(RuleHit {
                    rule_name: self.name().to_string(),
                    score: 20.0,
                    reason: "Zero amount transaction".to_string(),
                    metadata: Some(serde_json::json!({
                        "amount": amount
                    })),
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

/// Get the default set of fraud rules
pub fn get_default_fraud_rules() -> Vec<Box<dyn FraudRule>> {
    vec![
        Box::new(HighRiskIpRule),
        Box::new(VelocityRule),
        Box::new(SuspiciousAmountRule),
    ]
} 