use super::{FraudRule, RuleContext, RuleHit, RuleConfig, RulePriority, get_default_fraud_rules};
use anyhow::Result;
use std::collections::HashMap;
use tracing::{debug, warn};
use std::sync::Arc;

/// Rule engine for evaluating fraud rules against transactions
#[derive(Debug)]
pub struct RuleEngine {
    /// Active fraud rules
    rules: Vec<(Box<dyn FraudRule>, RuleConfig)>,
    
    /// Rule evaluation metrics
    metrics: RuleMetrics,
}

#[derive(Debug, Default)]
pub struct RuleMetrics {
    pub total_evaluations: u64,
    pub rule_hits: HashMap<String, u64>,
    pub evaluation_times: HashMap<String, Vec<u64>>, // in microseconds
}

#[derive(Debug)]
pub struct RuleEvaluationResult {
    pub hits: Vec<RuleHit>,
    pub total_score: f64,
    pub rule_count: usize,
    pub evaluation_time_ms: u64,
}

impl RuleEngine {
    /// Create a new rule engine with default rules
    pub fn new() -> Self {
        let rules = get_default_fraud_rules()
            .into_iter()
            .map(|rule| {
                let config = RuleConfig {
                    name: rule.name().to_string(),
                    enabled: true,
                    priority: rule.priority(),
                    weight: Self::get_default_weight(rule.priority()),
                };
                (rule, config)
            })
            .collect();

        Self {
            rules,
            metrics: RuleMetrics::default(),
        }
    }

    /// Create rule engine with custom rules
    pub fn with_rules(rules: Vec<(Box<dyn FraudRule>, RuleConfig)>) -> Self {
        Self {
            rules,
            metrics: RuleMetrics::default(),
        }
    }

    /// Evaluate all active rules against the given context
    pub async fn evaluate(&mut self, context: &RuleContext) -> Result<RuleEvaluationResult> {
        let start_time = std::time::Instant::now();
        let mut hits = Vec::new();
        let mut total_score = 0.0;
        let mut evaluated_count = 0;

        // Sort rules by priority (critical first)
        let mut sorted_rules: Vec<_> = self.rules.iter().collect();
        sorted_rules.sort_by(|a, b| b.1.priority.cmp(&a.1.priority));

        for (rule, config) in sorted_rules {
            if !config.enabled {
                continue;
            }

            let rule_start = std::time::Instant::now();
            
            match rule.evaluate(context).await {
                Ok(Some(mut hit)) => {
                    // Apply rule weight to the score
                    hit.score *= config.weight;
                    total_score += hit.score;
                    
                    debug!(
                        rule_name = rule.name(),
                        score = hit.score,
                        reason = %hit.reason,
                        "Fraud rule triggered"
                    );
                    
                    hits.push(hit);
                    
                    // Update metrics
                    *self.metrics.rule_hits.entry(rule.name().to_string()).or_insert(0) += 1;
                }
                Ok(None) => {
                    // Rule didn't trigger, but evaluation was successful
                    debug!(rule_name = rule.name(), "Fraud rule passed");
                }
                Err(e) => {
                    warn!(
                        rule_name = rule.name(),
                        error = %e,
                        "Fraud rule evaluation failed"
                    );
                    // Continue with other rules even if one fails
                }
            }
            
            // Record evaluation time
            let rule_time = rule_start.elapsed().as_micros() as u64;
            self.metrics.evaluation_times
                .entry(rule.name().to_string())
                .or_insert_with(Vec::new)
                .push(rule_time);
            
            evaluated_count += 1;
        }

        self.metrics.total_evaluations += 1;
        let total_time = start_time.elapsed().as_millis() as u64;

        // Sort hits by score (highest first)
        hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        debug!(
            total_score = total_score,
            rule_hits = hits.len(),
            evaluation_time_ms = total_time,
            "Rule engine evaluation completed"
        );

        Ok(RuleEvaluationResult {
            hits,
            total_score,
            rule_count: evaluated_count,
            evaluation_time_ms: total_time,
        })
    }

    /// Add a new rule to the engine
    pub fn add_rule(&mut self, rule: Box<dyn FraudRule>, config: RuleConfig) {
        self.rules.push((rule, config));
    }

    /// Remove a rule by name
    pub fn remove_rule(&mut self, rule_name: &str) {
        self.rules.retain(|(rule, _)| rule.name() != rule_name);
    }

    /// Enable or disable a rule
    pub fn set_rule_enabled(&mut self, rule_name: &str, enabled: bool) {
        for (rule, config) in &mut self.rules {
            if rule.name() == rule_name {
                config.enabled = enabled;
                break;
            }
        }
    }

    /// Update rule weight
    pub fn set_rule_weight(&mut self, rule_name: &str, weight: f64) {
        for (rule, config) in &mut self.rules {
            if rule.name() == rule_name {
                config.weight = weight;
                break;
            }
        }
    }

    /// Get rule evaluation metrics
    pub fn get_metrics(&self) -> &RuleMetrics {
        &self.metrics
    }

    /// Get list of active rules
    pub fn get_active_rules(&self) -> Vec<(&str, &RuleConfig)> {
        self.rules
            .iter()
            .filter(|(_, config)| config.enabled)
            .map(|(rule, config)| (rule.name(), config))
            .collect()
    }

    /// Evaluate rules against a transaction with user and device context
    pub async fn evaluate_transaction(
        &mut self,
        transaction: &crate::models::Transaction,
        user: &crate::models::User,
        device: &crate::models::Device,
        feature_store: Arc<dyn crate::rules::context::FeatureStore>,
    ) -> Result<RuleEvaluationResult> {
        // Convert to the expected types for RuleContext
        let transaction_request = Arc::new(crate::models::requests::TransactionRequest {
            device: crate::models::requests::DeviceRequest {
                ip_address: device.ip_address,
                user_agent: device.user_agent.clone(),
                accept_language: device.accept_language.clone(),
                session_id: device.session_id.clone(),
                session_age: None,
            },
            event: crate::models::requests::EventRequest {
                event_type: transaction.event_type.clone(),
                transaction_id: transaction.external_transaction_id.clone(),
                shop_id: transaction.shop_id.clone(),
                time: transaction.event_time,
            },
            user_id: Some(user.id),
            account: None,
            email: None,
            billing: None,
            shipping: None,
            payment: None,
            credit_card: None,
            order: Some(crate::models::requests::OrderRequest {
                amount: transaction.order_amount,
                currency: transaction.order_currency.clone(),
                discount_code: transaction.discount_code.clone(),
                affiliate_id: transaction.affiliate_id.clone(),
                subaffiliate_id: transaction.subaffiliate_id.clone(),
                referrer_uri: transaction.referrer_uri.clone(),
                is_gift: transaction.is_gift,
                has_gift_message: transaction.has_gift_message,
            }),
            shopping_cart: None,
            custom_inputs: transaction.custom_inputs.clone(),
        });
        
        let context = RuleContext::new(
            transaction_request,
            Some(Arc::new(user.clone())),
            Some(Arc::new(device.clone())),
            feature_store,
        );
        
        self.evaluate(&context).await
    }

    /// Get default weight based on rule priority
    fn get_default_weight(priority: RulePriority) -> f64 {
        match priority {
            RulePriority::Critical => 3.0,
            RulePriority::High => 2.0,
            RulePriority::Medium => 1.0,
            RulePriority::Low => 0.5,
        }
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleMetrics {
    /// Get average evaluation time for a rule (in microseconds)
    pub fn average_evaluation_time(&self, rule_name: &str) -> Option<f64> {
        self.evaluation_times
            .get(rule_name)
            .map(|times| {
                let sum: u64 = times.iter().sum();
                sum as f64 / times.len() as f64
            })
    }

    /// Get total rule hits across all rules
    pub fn total_hits(&self) -> u64 {
        self.rule_hits.values().sum()
    }

    /// Get hit rate for a specific rule
    pub fn hit_rate(&self, rule_name: &str) -> f64 {
        if self.total_evaluations == 0 {
            return 0.0;
        }
        
        let hits = self.rule_hits.get(rule_name).unwrap_or(&0);
        *hits as f64 / self.total_evaluations as f64
    }
} 