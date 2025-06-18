-- Fusegu Fraud Detection API Database Schema
-- Based on MinFraud-style transaction risk assessment API
-- PostgreSQL compatible schema with UUID primary keys

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Account management
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id VARCHAR(255) UNIQUE NOT NULL,
    subscription_tier VARCHAR(50) NOT NULL DEFAULT 'free' CHECK (subscription_tier IN ('free', 'pro', 'enterprise')),
    funds_remaining DECIMAL(12,2) NOT NULL DEFAULT 0.00,
    monthly_quota INTEGER NOT NULL DEFAULT 1000,
    queries_used_this_month INTEGER NOT NULL DEFAULT 0,
    billing_cycle_start DATE NOT NULL,
    billing_cycle_end DATE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- API Keys for authentication
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    key_hash VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    permissions JSONB DEFAULT '{}',
    last_used_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Device fingerprinting and tracking
CREATE TABLE devices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ip_address INET NOT NULL,
    user_agent TEXT,
    accept_language VARCHAR(255),
    session_id VARCHAR(255),
    session_age INTEGER,
    risk_score DECIMAL(5,2),
    location_data JSONB DEFAULT '{}',
    traits_data JSONB DEFAULT '{}',
    first_seen TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Email address analysis
CREATE TABLE email_addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email_hash VARCHAR(64) UNIQUE NOT NULL, -- MD5 hash of email
    domain VARCHAR(255),
    is_free BOOLEAN DEFAULT false,
    is_disposable BOOLEAN DEFAULT false,
    is_high_risk BOOLEAN DEFAULT false,
    first_seen DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Address information for billing/shipping
CREATE TABLE addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    company VARCHAR(255),
    address_line_1 VARCHAR(255),
    address_line_2 VARCHAR(255),
    city VARCHAR(255),
    region VARCHAR(4), -- ISO 3166-2 subdivision code
    postal_code VARCHAR(255),
    country CHAR(2), -- ISO 3166-1 alpha-2 country code
    phone_number VARCHAR(255),
    phone_country_code VARCHAR(4),
    latitude DECIMAL(10,8),
    longitude DECIMAL(11,8),
    is_high_risk BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Credit card information
CREATE TABLE credit_cards (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    issuer_id_number VARCHAR(8), -- BIN (first 6-8 digits)
    last_digits VARCHAR(4), -- Last 2-4 digits
    token_hash VARCHAR(255), -- Hashed token
    bank_name VARCHAR(255),
    bank_phone_number VARCHAR(255),
    bank_phone_country_code VARCHAR(4),
    country CHAR(2), -- Country where card was issued
    avs_result CHAR(1), -- Address Verification System result
    cvv_result CHAR(1), -- CVV verification result
    was_3d_secure_successful BOOLEAN,
    brand VARCHAR(50), -- Visa, Mastercard, etc.
    card_type VARCHAR(20) CHECK (card_type IN ('credit', 'debit', 'charge')),
    is_business BOOLEAN DEFAULT false,
    is_prepaid BOOLEAN DEFAULT false,
    is_virtual BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Core transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    external_transaction_id VARCHAR(255), -- Customer's transaction ID
    risk_score DECIMAL(5,2) NOT NULL,
    risk_level VARCHAR(20) NOT NULL CHECK (risk_level IN ('low', 'medium', 'high', 'very_high')),
    disposition VARCHAR(20) NOT NULL CHECK (disposition IN ('accept', 'reject', 'review', 'test')),
    event_type VARCHAR(50) NOT NULL CHECK (event_type IN ('account_creation', 'account_login', 'email_change', 'password_reset', 'payout_change', 'purchase', 'recurring_purchase', 'referral', 'survey')),
    shop_id VARCHAR(255),
    event_time TIMESTAMP WITH TIME ZONE NOT NULL,
    device_data JSONB DEFAULT '{}',
    custom_inputs JSONB DEFAULT '{}',
    warnings JSONB DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Junction table for transaction-device relationships
CREATE TABLE transaction_devices (
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (transaction_id, device_id)
);

-- Junction table for transaction-email relationships
CREATE TABLE transaction_emails (
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    email_id UUID NOT NULL REFERENCES email_addresses(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (transaction_id, email_id)
);

-- Junction table for transaction-address relationships
CREATE TABLE transaction_addresses (
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    address_id UUID NOT NULL REFERENCES addresses(id) ON DELETE CASCADE,
    address_type VARCHAR(20) NOT NULL CHECK (address_type IN ('billing', 'shipping')),
    delivery_speed VARCHAR(20) CHECK (delivery_speed IN ('same_day', 'overnight', 'expedited', 'standard')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (transaction_id, address_id, address_type)
);

-- Junction table for transaction-credit card relationships
CREATE TABLE transaction_credit_cards (
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    credit_card_id UUID NOT NULL REFERENCES credit_cards(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (transaction_id, credit_card_id)
);

-- Order information
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    amount DECIMAL(15,2) NOT NULL,
    currency CHAR(3) NOT NULL, -- ISO 4217 currency code
    discount_code VARCHAR(255),
    affiliate_id VARCHAR(255),
    subaffiliate_id VARCHAR(255),
    referrer_uri TEXT,
    is_gift BOOLEAN DEFAULT false,
    has_gift_message BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Shopping cart items
CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    item_id VARCHAR(255) NOT NULL,
    category VARCHAR(255),
    price DECIMAL(15,2) NOT NULL,
    quantity INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Transaction outcome reporting
CREATE TABLE transaction_reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL CHECK (tag IN ('chargeback', 'not_fraud', 'suspected_fraud', 'spam_or_abuse')),
    chargeback_code VARCHAR(32),
    notes TEXT,
    occurred_at TIMESTAMP WITH TIME ZONE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'received' CHECK (status IN ('received', 'processing', 'processed')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(transaction_id) -- Only one report per transaction
);

-- Batch processing
CREATE TABLE batches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'processing', 'completed', 'failed')),
    transaction_count INTEGER NOT NULL DEFAULT 0,
    processed_count INTEGER NOT NULL DEFAULT 0,
    success_count INTEGER NOT NULL DEFAULT 0,
    error_count INTEGER NOT NULL DEFAULT 0,
    webhook_url TEXT,
    submitted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP WITH TIME ZONE,
    estimated_completion_time TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Batch transaction processing details
CREATE TABLE batch_transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    batch_id UUID NOT NULL REFERENCES batches(id) ON DELETE CASCADE,
    transaction_id UUID REFERENCES transactions(id) ON DELETE CASCADE,
    external_id VARCHAR(255), -- Customer's external ID
    error_data JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Webhook subscriptions
CREATE TABLE webhooks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    events JSONB NOT NULL DEFAULT '[]', -- Array of event types
    secret_hash VARCHAR(255), -- Hashed webhook secret
    description VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_triggered TIMESTAMP WITH TIME ZONE,
    success_count INTEGER NOT NULL DEFAULT 0,
    failure_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Rate limiting tracking
CREATE TABLE rate_limits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    endpoint VARCHAR(255) NOT NULL,
    requests_count INTEGER NOT NULL DEFAULT 0,
    window_start TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(account_id, endpoint, window_start)
);

-- Risk factor analysis details
CREATE TABLE risk_factors (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    factor_code VARCHAR(100) NOT NULL,
    factor_type VARCHAR(50) NOT NULL,
    multiplier DECIMAL(8,2) NOT NULL,
    reason TEXT NOT NULL,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- IP address risk caching
CREATE TABLE ip_risk_cache (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ip_address INET UNIQUE NOT NULL,
    risk_score DECIMAL(5,2),
    risk_reasons JSONB DEFAULT '[]',
    location_data JSONB DEFAULT '{}',
    traits_data JSONB DEFAULT '{}',
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_transactions_account_id ON transactions(account_id);
CREATE INDEX idx_transactions_created_at ON transactions(created_at);
CREATE INDEX idx_transactions_risk_score ON transactions(risk_score);
CREATE INDEX idx_transactions_external_id ON transactions(external_transaction_id);

CREATE INDEX idx_devices_ip_address ON devices(ip_address);
CREATE INDEX idx_devices_last_seen ON devices(last_seen);

CREATE INDEX idx_email_addresses_domain ON email_addresses(domain);
CREATE INDEX idx_email_addresses_is_high_risk ON email_addresses(is_high_risk);

CREATE INDEX idx_addresses_country ON addresses(country);
CREATE INDEX idx_addresses_postal_code ON addresses(postal_code);

CREATE INDEX idx_credit_cards_issuer_id ON credit_cards(issuer_id_number);
CREATE INDEX idx_credit_cards_country ON credit_cards(country);

CREATE INDEX idx_api_keys_account_id ON api_keys(account_id);
CREATE INDEX idx_api_keys_is_active ON api_keys(is_active);

CREATE INDEX idx_batches_account_id ON batches(account_id);
CREATE INDEX idx_batches_status ON batches(status);
CREATE INDEX idx_batches_submitted_at ON batches(submitted_at);

CREATE INDEX idx_webhooks_account_id ON webhooks(account_id);
CREATE INDEX idx_webhooks_is_active ON webhooks(is_active);

CREATE INDEX idx_rate_limits_window_start ON rate_limits(window_start);
CREATE INDEX idx_rate_limits_account_endpoint ON rate_limits(account_id, endpoint);

CREATE INDEX idx_ip_risk_cache_expires_at ON ip_risk_cache(expires_at);

-- Triggers for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_accounts_updated_at BEFORE UPDATE ON accounts FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_api_keys_updated_at BEFORE UPDATE ON api_keys FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_devices_updated_at BEFORE UPDATE ON devices FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_email_addresses_updated_at BEFORE UPDATE ON email_addresses FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_addresses_updated_at BEFORE UPDATE ON addresses FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_credit_cards_updated_at BEFORE UPDATE ON credit_cards FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_transactions_updated_at BEFORE UPDATE ON transactions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_orders_updated_at BEFORE UPDATE ON orders FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_cart_items_updated_at BEFORE UPDATE ON cart_items FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_transaction_reports_updated_at BEFORE UPDATE ON transaction_reports FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_batches_updated_at BEFORE UPDATE ON batches FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_batch_transactions_updated_at BEFORE UPDATE ON batch_transactions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_webhooks_updated_at BEFORE UPDATE ON webhooks FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_rate_limits_updated_at BEFORE UPDATE ON rate_limits FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_ip_risk_cache_updated_at BEFORE UPDATE ON ip_risk_cache FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert default account for testing
INSERT INTO accounts (account_id, subscription_tier, funds_remaining, monthly_quota, billing_cycle_start, billing_cycle_end)
VALUES ('demo_account', 'pro', 10000.00, 100000, CURRENT_DATE, CURRENT_DATE + INTERVAL '1 month'); 