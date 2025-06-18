//! # Fusegu - Anti-Fraud Detection API
//!
//! Environment-based configuration with security-first design.

pub mod api;
pub mod config;
pub mod models;
pub mod server;

// Re-export commonly used types
pub use config::Config;
pub use server::create_app;
