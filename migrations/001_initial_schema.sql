-- Phase 1: Core Schema for Multi-tenant Fraud Detection Platform
-- This schema supports user-centric fraud detection with cross-transaction analysis

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Accounts table for multi-tenancy
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    subscription_tier VARCHAR(50) NOT NULL DEFAULT 'free' CHECK (subscription_tier IN ('free', 'pro', 'enterprise')),
    funds_remaining DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    monthly_quota INTEGER NOT NULL DEFAULT 1000,
    queries_used_this_month INTEGER NOT NULL DEFAULT 0,
    billing_cycle_start DATE,
    billing_cycle_end DATE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- API Keys for authentication
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    key_hash VARCHAR(255) NOT NULL UNIQUE, -- Hashed API key
    name VARCHAR(255) NOT NULL,
    permissions TEXT[] NOT NULL DEFAULT ARRAY['read', 'write'],
    last_used_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Users table for cross-transaction tracking
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    external_user_id VARCHAR(255), -- Customer's internal user ID
    user_hash VARCHAR(64), -- Hash of user identifier for privacy
    risk_score DECIMAL(5,2) CHECK (risk_score >= 0.00 AND risk_score <= 99.99),
    risk_level VARCHAR(20) CHECK (risk_level IN ('low', 'medium', 'high', 'very_high')),
    total_transactions INTEGER NOT NULL DEFAULT 0,
    successful_transactions INTEGER NOT NULL DEFAULT 0,
    failed_transactions INTEGER NOT NULL DEFAULT 0,
    chargeback_count INTEGER NOT NULL DEFAULT 0,
    first_transaction_at TIMESTAMPTZ,
    last_transaction_at TIMESTAMPTZ,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    is_flagged BOOLEAN NOT NULL DEFAULT false,
    flags TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Devices table for device fingerprinting
CREATE TABLE devices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    fingerprint_hash VARCHAR(64) NOT NULL, -- Hash of device fingerprint
    ip_address INET NOT NULL,
    user_agent TEXT,
    accept_language VARCHAR(255),
    session_id VARCHAR(255),
    first_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_count INTEGER NOT NULL DEFAULT 0,
    risk_score DECIMAL(5,2) CHECK (risk_score >= 0.00 AND risk_score <= 99.99),
    is_suspicious BOOLEAN NOT NULL DEFAULT false,
    metadata JSONB
);

-- Email addresses table
CREATE TABLE email_addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    email_hash VARCHAR(64) NOT NULL, -- Hash of email for privacy
    domain VARCHAR(255),
    is_free_provider BOOLEAN,
    is_disposable BOOLEAN,
    is_high_risk BOOLEAN NOT NULL DEFAULT false,
    first_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_count INTEGER NOT NULL DEFAULT 0,
    risk_score DECIMAL(5,2) CHECK (risk_score >= 0.00 AND risk_score <= 99.99)
);

-- Addresses table
CREATE TABLE addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    address_hash VARCHAR(64) NOT NULL, -- Hash of normalized address
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    company VARCHAR(255),
    address_line_1 VARCHAR(255),
    address_line_2 VARCHAR(255),
    city VARCHAR(255),
    region VARCHAR(10), -- ISO 3166-2 subdivision code
    postal_code VARCHAR(20),
    country VARCHAR(2), -- ISO 3166-1 alpha-2 country code
    phone_number VARCHAR(50),
    phone_country_code VARCHAR(4),
    latitude DECIMAL(10,7),
    longitude DECIMAL(10,7),
    is_high_risk BOOLEAN NOT NULL DEFAULT false,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    first_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Credit cards table
CREATE TABLE credit_cards (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    card_hash VARCHAR(64) NOT NULL, -- Hash of BIN + last 4 digits
    issuer_id_number VARCHAR(8), -- BIN (first 6-8 digits)
    last_digits VARCHAR(4),
    brand VARCHAR(50),
    card_type VARCHAR(20) CHECK (card_type IN ('credit', 'debit', 'charge')),
    country VARCHAR(2), -- Country where card was issued
    bank_name VARCHAR(255),
    bank_phone VARCHAR(50),
    is_prepaid BOOLEAN,
    is_business BOOLEAN,
    is_virtual BOOLEAN,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    chargeback_count INTEGER NOT NULL DEFAULT 0,
    risk_score DECIMAL(5,2) CHECK (risk_score >= 0.00 AND risk_score <= 99.99),
    first_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Transactions table - core data
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    external_transaction_id VARCHAR(255), -- Customer's transaction ID
    event_type VARCHAR(50) NOT NULL CHECK (event_type IN ('account_creation', 'account_login', 'email_change', 'password_reset', 'payout_change', 'purchase', 'recurring_purchase', 'referral', 'survey')),
    event_time TIMESTAMPTZ,
    shop_id VARCHAR(255),
    
    -- Risk assessment results
    risk_score DECIMAL(5,2) NOT NULL CHECK (risk_score >= 0.01 AND risk_score <= 99.99),
    risk_level VARCHAR(20) NOT NULL CHECK (risk_level IN ('low', 'medium', 'high', 'very_high')),
    disposition VARCHAR(20) NOT NULL CHECK (disposition IN ('accept', 'reject', 'review', 'test')),
    
    -- Order information
    order_amount DECIMAL(15,2),
    order_currency VARCHAR(3),
    discount_code VARCHAR(255),
    affiliate_id VARCHAR(255),
    subaffiliate_id VARCHAR(255),
    referrer_uri TEXT,
    is_gift BOOLEAN,
    has_gift_message BOOLEAN,
    
    -- Payment information
    payment_processor VARCHAR(50),
    was_authorized BOOLEAN,
    decline_code VARCHAR(255),
    avs_result VARCHAR(1),
    cvv_result VARCHAR(1),
    was_3d_secure_successful BOOLEAN,
    
    -- Analysis metadata
    rule_hits JSONB, -- Details of which rules triggered
    custom_inputs JSONB, -- Custom fields from API request
    warnings JSONB, -- Warnings about data quality
    raw_request JSONB, -- Original request for debugging
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Junction tables for many-to-many relationships

-- Transaction-Device associations
CREATE TABLE transaction_devices (
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    session_age INTEGER, -- Session age in seconds
    PRIMARY KEY (transaction_id, device_id)
);

-- Transaction-Email associations
CREATE TABLE transaction_emails (
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    email_id UUID NOT NULL REFERENCES email_addresses(id) ON DELETE CASCADE,
    PRIMARY KEY (transaction_id, email_id)
);

-- Transaction-Address associations (billing and shipping)
CREATE TABLE transaction_addresses (
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    address_id UUID NOT NULL REFERENCES addresses(id) ON DELETE CASCADE,
    address_type VARCHAR(20) NOT NULL CHECK (address_type IN ('billing', 'shipping')),
    delivery_speed VARCHAR(20) CHECK (delivery_speed IN ('same_day', 'overnight', 'expedited', 'standard')),
    PRIMARY KEY (transaction_id, address_id, address_type)
);

-- Transaction-CreditCard associations
CREATE TABLE transaction_credit_cards (
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    credit_card_id UUID NOT NULL REFERENCES credit_cards(id) ON DELETE CASCADE,
    PRIMARY KEY (transaction_id, credit_card_id)
);

-- Shopping cart items
CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    item_id VARCHAR(255) NOT NULL,
    category VARCHAR(255),
    price DECIMAL(15,2) NOT NULL CHECK (price >= 0),
    quantity INTEGER NOT NULL CHECK (quantity >= 0)
);

-- Essential indexes for Phase 1 performance
CREATE INDEX CONCURRENTLY idx_accounts_email ON accounts(email);
CREATE INDEX CONCURRENTLY idx_api_keys_account_active ON api_keys(account_id, is_active);
CREATE INDEX CONCURRENTLY idx_api_keys_hash ON api_keys(key_hash);

CREATE INDEX CONCURRENTLY idx_users_account_external ON users(account_id, external_user_id);
CREATE INDEX CONCURRENTLY idx_users_account_hash ON users(account_id, user_hash);
CREATE INDEX CONCURRENTLY idx_users_risk_flagged ON users(account_id, risk_level, is_flagged);

CREATE INDEX CONCURRENTLY idx_transactions_account_created ON transactions(account_id, created_at DESC);
CREATE INDEX CONCURRENTLY idx_transactions_user_created ON transactions(user_id, created_at DESC);
CREATE INDEX CONCURRENTLY idx_transactions_risk_score ON transactions(account_id, risk_score DESC);
CREATE INDEX CONCURRENTLY idx_transactions_external_id ON transactions(account_id, external_transaction_id);

CREATE INDEX CONCURRENTLY idx_devices_account_ip ON devices(account_id, ip_address);
CREATE INDEX CONCURRENTLY idx_devices_fingerprint ON devices(account_id, fingerprint_hash);
CREATE INDEX CONCURRENTLY idx_devices_ip_seen ON devices(ip_address, last_seen);

CREATE INDEX CONCURRENTLY idx_emails_account_hash ON email_addresses(account_id, email_hash);
CREATE INDEX CONCURRENTLY idx_emails_domain ON email_addresses(domain);

CREATE INDEX CONCURRENTLY idx_addresses_account_hash ON addresses(account_id, address_hash);
CREATE INDEX CONCURRENTLY idx_addresses_country ON addresses(country);

CREATE INDEX CONCURRENTLY idx_credit_cards_account_hash ON credit_cards(account_id, card_hash);
CREATE INDEX CONCURRENTLY idx_credit_cards_bin ON credit_cards(issuer_id_number);

CREATE INDEX CONCURRENTLY idx_cart_items_transaction ON cart_items(transaction_id);

-- Update timestamp triggers
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_accounts_updated_at BEFORE UPDATE ON accounts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_api_keys_updated_at BEFORE UPDATE ON api_keys
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_transactions_updated_at BEFORE UPDATE ON transactions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column(); 