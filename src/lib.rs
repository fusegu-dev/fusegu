//! # Fusegu - Anti-Fraud Detection API
//!
//! A comprehensive fraud detection platform with user-centric analysis and cross-transaction risk assessment.
//! 
//! Phase 1 MVP features:
//! - Real-time transaction risk scoring
//! - User-centric fraud detection with cross-transaction analysis
//! - Multi-tenant architecture with API key authentication
//! - Device fingerprinting and IP analysis
//! - Basic rule engine with 20+ fraud patterns
//! - PostgreSQL + Redis architecture

pub mod api;
pub mod auth;
pub mod config;
pub mod database;
pub mod models;
pub mod rules;
pub mod scoring;
pub mod server;
pub mod services;
pub mod utils;

// Re-export commonly used types
pub use config::Config;
pub use server::create_app;
