pub mod crypto;
pub mod geo;
pub mod validation;

pub use crypto::*;
pub use geo::*;
pub use validation::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate a simple hash of a string (not cryptographically secure)
pub fn simple_hash(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Generate a deterministic ID from multiple components
pub fn generate_composite_hash(components: &[&str]) -> String {
    let combined = components.join("|");
    simple_hash(&combined)
}

/// Normalize email address for consistent hashing
pub fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

/// Extract domain from email address
pub fn extract_email_domain(email: &str) -> Option<String> {
    email.split('@').nth(1).map(|domain| domain.trim().to_lowercase())
}

/// Check if string is likely a MD5 hash (32 hex characters)
pub fn is_md5_hash(input: &str) -> bool {
    input.len() == 32 && input.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validate and normalize country code (ISO 3166-1 alpha-2)
pub fn normalize_country_code(country: &str) -> Option<String> {
    let normalized = country.trim().to_uppercase();
    if normalized.len() == 2 && normalized.chars().all(|c| c.is_ascii_alphabetic()) {
        Some(normalized)
    } else {
        None
    }
}

/// Validate and normalize currency code (ISO 4217)
pub fn normalize_currency_code(currency: &str) -> Option<String> {
    let normalized = currency.trim().to_uppercase();
    if normalized.len() == 3 && normalized.chars().all(|c| c.is_ascii_alphabetic()) {
        Some(normalized)
    } else {
        None
    }
} 