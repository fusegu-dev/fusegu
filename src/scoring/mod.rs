use crate::models::common::{Disposition, RiskLevel};
use crate::rules::{RuleEvaluationResult, RuleHit};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Scoring engine for calculating final risk scores from rule evaluations
#[derive(Debug, Clone)]
pub struct ScoringEngine {
    config: ScoringConfig,
}

#[derive(Debug, Clone)]
pub struct ScoringConfig {
    /// Base score when no rules are triggered
    pub base_score: f64,
    
    /// Maximum possible risk score
    pub max_score: f64,
    
    /// Score thresholds for risk levels
    pub risk_thresholds: RiskThresholds,
    
    /// Whether to apply score normalization
    pub normalize_scores: bool,
}

#[derive(Debug, Clone)]
pub struct RiskThresholds {
    pub low: f64,      // 0 to low = Low risk
    pub medium: f64,   // low to medium = Medium risk  
    pub high: f64,     // medium to high = High risk
    // high to max = Very High risk
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringResult {
    pub risk_score: Decimal,
    pub risk_level: RiskLevel,
    pub disposition: Disposition,
    pub rule_contributions: Vec<RuleContribution>,
    pub score_breakdown: ScoreBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleContribution {
    pub rule_name: String,
    pub score: f64,
    pub weight: f64,
    pub weighted_score: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub base_score: f64,
    pub rule_score: f64,
    pub total_score: f64,
    pub normalized_score: f64,
    pub rule_count: usize,
}

impl ScoringEngine {
    /// Create a new scoring engine with default configuration
    pub fn new() -> Self {
        Self {
            config: ScoringConfig::default(),
        }
    }

    /// Create scoring engine with custom configuration
    pub fn with_config(config: ScoringConfig) -> Self {
        Self { config }
    }

    /// Calculate final risk score from rule evaluation results
    pub fn calculate_score(&self, rule_result: &RuleEvaluationResult) -> ScoringResult {
        let rule_contributions: Vec<RuleContribution> = rule_result
            .hits
            .iter()
            .map(|hit| RuleContribution {
                rule_name: hit.rule_name.clone(),
                score: hit.score,
                weight: 1.0, // TODO: Get actual weight from rule config
                weighted_score: hit.score,
                reason: hit.reason.clone(),
            })
            .collect();

        let rule_score = rule_result.total_score;
        let total_score = self.config.base_score + rule_score;
        
        // Apply normalization if enabled
        let normalized_score = if self.config.normalize_scores {
            self.normalize_score(total_score)
        } else {
            total_score.min(self.config.max_score)
        };

        let risk_level = self.calculate_risk_level(normalized_score);
        let disposition = self.calculate_disposition(risk_level);

        let score_breakdown = ScoreBreakdown {
            base_score: self.config.base_score,
            rule_score,
            total_score,
            normalized_score,
            rule_count: rule_result.hits.len(),
        };

        ScoringResult {
            risk_score: Decimal::from_f64_retain(normalized_score)
                .unwrap_or_else(|| Decimal::from(0)),
            risk_level,
            disposition,
            rule_contributions,
            score_breakdown,
        }
    }

    /// Normalize score using sigmoid function to prevent extreme values
    fn normalize_score(&self, score: f64) -> f64 {
        // Sigmoid normalization: maps [0, infinity] to [0, max_score]
        let normalized = self.config.max_score * (1.0 / (1.0 + (-score / 20.0).exp()));
        normalized.max(0.01).min(self.config.max_score) // Ensure within bounds
    }

    /// Calculate risk level based on score thresholds
    fn calculate_risk_level(&self, score: f64) -> RiskLevel {
        if score < self.config.risk_thresholds.low {
            RiskLevel::Low
        } else if score < self.config.risk_thresholds.medium {
            RiskLevel::Medium
        } else if score < self.config.risk_thresholds.high {
            RiskLevel::High
        } else {
            RiskLevel::VeryHigh
        }
    }

    /// Calculate disposition based on risk level
    fn calculate_disposition(&self, risk_level: RiskLevel) -> Disposition {
        match risk_level {
            RiskLevel::Low => Disposition::Accept,
            RiskLevel::Medium => Disposition::Review,
            RiskLevel::High | RiskLevel::VeryHigh => Disposition::Reject,
        }
    }

    /// Update scoring configuration
    pub fn update_config(&mut self, config: ScoringConfig) {
        self.config = config;
    }

    /// Get current scoring configuration
    pub fn get_config(&self) -> &ScoringConfig {
        &self.config
    }
}

impl Default for ScoringEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ScoringConfig {
    fn default() -> Self {
        Self {
            base_score: 1.0,
            max_score: 99.99,
            risk_thresholds: RiskThresholds::default(),
            normalize_scores: true,
        }
    }
}

impl Default for RiskThresholds {
    fn default() -> Self {
        Self {
            low: 10.0,
            medium: 30.0,
            high: 70.0,
        }
    }
}

/// Risk scorer type alias for compatibility
pub type RiskScorer = ScoringEngine;

impl ScoringEngine {
    /// Calculate final risk score (simplified interface for services)
    pub async fn calculate_risk_score(
        &self,
        _transaction: &crate::models::Transaction,
        _user: &crate::models::User,
        _device: &crate::models::Device,
        rule_results: &RuleEvaluationResult,
    ) -> anyhow::Result<f64> {
        let scoring_result = self.calculate_score(rule_results);
        Ok(scoring_result.risk_score.to_string().parse().unwrap_or(0.0))
    }
} 