//! API endpoints and handlers

pub mod errors;
pub mod health;

// Re-export common types
pub use errors::{ApiError, ApiResult};
pub use health::health_check;
