use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::database::DatabasePool;
use crate::models::{
    requests::TransactionRequest,
    responses::TransactionAnalysisResponse,
    transaction::Transaction,
    Device, User,
};
use crate::rules::RuleEngine;
use crate::scoring::RiskScorer;

pub struct TransactionService {
    db_pool: Arc<DatabasePool>,
    rule_engine: Arc<std::sync::Mutex<RuleEngine>>,
    risk_scorer: Arc<RiskScorer>,
}

impl TransactionService {
    pub fn new(
        db_pool: Arc<DatabasePool>,
        rule_engine: Arc<std::sync::Mutex<RuleEngine>>,
        risk_scorer: Arc<RiskScorer>,
    ) -> Self {
        Self {
            db_pool,
            rule_engine,
            risk_scorer,
        }
    }

    pub async fn analyze_transaction(
        &self,
        request: TransactionRequest,
    ) -> Result<TransactionAnalysisResponse> {
        // Create transaction record
        let transaction = Transaction::from_request(&request);
        
        // Get user and device context
        let user = self.get_or_create_user(&request.user_id.map(|u| u.to_string()).unwrap_or_default()).await?;
        let device = self.get_or_create_device(&request.device.user_agent).await?;
        
        // Evaluate rules and calculate risk score
        let feature_store = Arc::new(crate::services::feature_store::RedisFeatureStore::new(
            Arc::new(crate::database::CacheService::new(self.db_pool.redis().clone())),
            self.db_pool.clone(),
        ));
        
        let mut rule_engine = self.rule_engine.lock().unwrap();
        let rule_results = rule_engine.evaluate_transaction(&transaction, &user, &device, feature_store).await?;
        drop(rule_engine);
        
        let risk_score = self.risk_scorer.calculate_risk_score(&transaction, &user, &device, &rule_results).await?;
        
        // Store transaction
        self.store_transaction(&transaction).await?;
        
        // Create analysis response
        let rule_names: Vec<String> = rule_results.hits.iter().map(|hit| hit.rule_name.clone()).collect();
        
        Ok(TransactionAnalysisResponse::from_analysis(
            transaction,
            risk_score,
            rule_names,
        ))
    }

    async fn get_or_create_user(&self, user_id: &str) -> Result<User> {
        // Implementation would query/create user from database
        todo!("Implement user lookup/creation")
    }

    async fn get_or_create_device(&self, fingerprint: &Option<String>) -> Result<Device> {
        // Implementation would query/create device from database
        todo!("Implement device lookup/creation")
    }

    async fn store_transaction(&self, transaction: &Transaction) -> Result<()> {
        // Implementation would store transaction in database
        todo!("Implement transaction storage")
    }
} 