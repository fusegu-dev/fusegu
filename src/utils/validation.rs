use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashMap;
use validator::{validate_email, validate_url};

/// Validate email address
pub fn validate_email_address(email: &str) -> Result<()> {
    if validate_email(email) {
        Ok(())
    } else {
        Err(anyhow!("Invalid email address: {}", email))
    }
}

/// Validate phone number (basic validation)
pub fn validate_phone_number(phone: &str) -> Result<()> {
    let phone_regex = Regex::new(r"^\+?[\d\s\-\(\)]+$")?;
    if phone_regex.is_match(phone) && phone.len() >= 10 && phone.len() <= 20 {
        Ok(())
    } else {
        Err(anyhow!("Invalid phone number: {}", phone))
    }
}

/// Validate URL
pub fn validate_url_string(url: &str) -> Result<()> {
    if validate_url(url) {
        Ok(())
    } else {
        Err(anyhow!("Invalid URL: {}", url))
    }
}

/// Validate IP address
pub fn validate_ip_address(ip: &str) -> Result<()> {
    ip.parse::<std::net::IpAddr>()
        .map(|_| ())
        .map_err(|_| anyhow!("Invalid IP address: {}", ip))
}

/// Validate currency code (ISO 4217)
pub fn validate_currency_code(currency: &str) -> Result<()> {
    if currency.len() == 3 && currency.chars().all(|c| c.is_ascii_uppercase()) {
        Ok(())
    } else {
        Err(anyhow!("Invalid currency code: {}", currency))
    }
}

/// Validate country code (ISO 3166-1 alpha-2)
pub fn validate_country_code(country: &str) -> Result<()> {
    if country.len() == 2 && country.chars().all(|c| c.is_ascii_uppercase()) {
        Ok(())
    } else {
        Err(anyhow!("Invalid country code: {}", country))
    }
}

/// Sanitize string input (remove potentially dangerous characters)
pub fn sanitize_string(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || " .-_@".contains(*c))
        .collect::<String>()
        .trim()
        .to_string()
}

/// Validate transaction amount
pub fn validate_transaction_amount(amount: f64, currency: &str) -> Result<()> {
    if amount < 0.0 {
        return Err(anyhow!("Transaction amount cannot be negative"));
    }
    
    if amount > 1_000_000.0 {
        return Err(anyhow!("Transaction amount exceeds maximum limit"));
    }
    
    // Currency-specific validation
    match currency {
        "USD" | "EUR" | "GBP" => {
            if amount > 100_000.0 {
                return Err(anyhow!("Amount exceeds daily limit for {}", currency));
            }
        }
        _ => {} // Other currencies
    }
    
    Ok(())
}

/// Validate user ID format
pub fn validate_user_id(user_id: &str) -> Result<()> {
    if user_id.is_empty() {
        return Err(anyhow!("User ID cannot be empty"));
    }
    
    if user_id.len() > 255 {
        return Err(anyhow!("User ID too long"));
    }
    
    // Allow alphanumeric, hyphens, underscores
    let user_id_regex = Regex::new(r"^[a-zA-Z0-9_-]+$")?;
    if !user_id_regex.is_match(user_id) {
        return Err(anyhow!("Invalid user ID format: {}", user_id));
    }
    
    Ok(())
}

/// Validate merchant ID format
pub fn validate_merchant_id(merchant_id: &str) -> Result<()> {
    if merchant_id.is_empty() {
        return Err(anyhow!("Merchant ID cannot be empty"));
    }
    
    if merchant_id.len() > 100 {
        return Err(anyhow!("Merchant ID too long"));
    }
    
    let merchant_id_regex = Regex::new(r"^[a-zA-Z0-9_-]+$")?;
    if !merchant_id_regex.is_match(merchant_id) {
        return Err(anyhow!("Invalid merchant ID format: {}", merchant_id));
    }
    
    Ok(())
}

/// Validate device fingerprint
pub fn validate_device_fingerprint(fingerprint: &str) -> Result<()> {
    if fingerprint.len() < 10 || fingerprint.len() > 500 {
        return Err(anyhow!("Invalid device fingerprint length"));
    }
    
    // Should be base64 or hex encoded
    let valid_chars = fingerprint.chars().all(|c| {
        c.is_ascii_alphanumeric() || "=+/".contains(c) // base64
    });
    
    if !valid_chars {
        return Err(anyhow!("Invalid device fingerprint format"));
    }
    
    Ok(())
}

/// Comprehensive input validation for transaction requests
pub fn validate_transaction_request(data: &HashMap<String, String>) -> Result<()> {
    // Validate required fields
    let required_fields = ["user_id", "amount", "currency", "merchant_id"];
    for field in &required_fields {
        if !data.contains_key(*field) {
            return Err(anyhow!("Missing required field: {}", field));
        }
    }
    
    // Validate user ID
    if let Some(user_id) = data.get("user_id") {
        validate_user_id(user_id)?;
    }
    
    // Validate amount
    if let Some(amount_str) = data.get("amount") {
        let amount: f64 = amount_str.parse()
            .map_err(|_| anyhow!("Invalid amount format"))?;
        if let Some(currency) = data.get("currency") {
            validate_transaction_amount(amount, currency)?;
        }
    }
    
    // Validate currency
    if let Some(currency) = data.get("currency") {
        validate_currency_code(currency)?;
    }
    
    // Validate merchant ID
    if let Some(merchant_id) = data.get("merchant_id") {
        validate_merchant_id(merchant_id)?;
    }
    
    // Validate optional fields
    if let Some(email) = data.get("email") {
        validate_email_address(email)?;
    }
    
    if let Some(phone) = data.get("phone") {
        validate_phone_number(phone)?;
    }
    
    if let Some(ip) = data.get("ip_address") {
        validate_ip_address(ip)?;
    }
    
    if let Some(fingerprint) = data.get("device_fingerprint") {
        validate_device_fingerprint(fingerprint)?;
    }
    
    Ok(())
} 