use anyhow::{Context, Result};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use sha2::{Digest, Sha256};

/// Hash a password using Argon2
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;
    
    Ok(password_hash.to_string())
}

/// Verify a password against its hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| anyhow::anyhow!("Invalid hash format: {}", e))?;
    
    let argon2 = Argon2::default();
    
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Generate SHA-256 hash of input string
pub fn sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Generate a secure random string for API keys
pub fn generate_api_key() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const KEY_LENGTH: usize = 32;
    
    let mut rng = rand::thread_rng();
    let key: String = (0..KEY_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    format!("fusegu_{}", key)
}

/// Generate device fingerprint hash from multiple components
pub fn generate_device_fingerprint(
    ip: &str,
    user_agent: Option<&str>,
    accept_language: Option<&str>,
) -> String {
    let mut components = vec![ip];
    
    if let Some(ua) = user_agent {
        components.push(ua);
    }
    
    if let Some(lang) = accept_language {
        components.push(lang);
    }
    
    let combined = components.join("|");
    sha256_hash(&combined)
}

/// Generate address hash for privacy-preserving storage
pub fn generate_address_hash(
    address_line: Option<&str>,
    city: Option<&str>,
    postal: Option<&str>,
    country: Option<&str>,
) -> String {
    let components: Vec<&str> = [address_line, city, postal, country]
        .iter()
        .filter_map(|&x| x)
        .collect();
    
    let normalized = components
        .iter()
        .map(|s| s.trim().to_lowercase())
        .collect::<Vec<_>>()
        .join("|");
    
    sha256_hash(&normalized)
}

/// Generate card hash from BIN and last digits
pub fn generate_card_hash(bin: Option<&str>, last_digits: Option<&str>) -> String {
    let components: Vec<&str> = [bin, last_digits]
        .iter()
        .filter_map(|&x| x)
        .collect();
    
    if components.is_empty() {
        return sha256_hash("unknown_card");
    }
    
    let combined = components.join("|");
    sha256_hash(&combined)
} 