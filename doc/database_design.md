# Fusegu Database Design

This document describes the database schema design for the Fusegu fraud detection API, based on the MinFraud-style transaction risk assessment API specification.

## Overview

The database is designed to support a comprehensive fraud detection and risk assessment platform with the following key features:

- **Transaction Risk Scoring**: Real-time analysis of transaction risk
- **Device Fingerprinting**: Track and analyze device patterns
- **Email & Address Analysis**: Verify and assess contact information
- **Credit Card Intelligence**: Analyze payment method risk factors
- **Batch Processing**: Handle large volumes of transactions
- **Webhook Notifications**: Real-time event notifications
- **Account Management**: Multi-tenant architecture with usage tracking
- **Analytics & Reporting**: Historical data analysis and insights

## Core Architecture

The schema follows a normalized design with the following principles:

1. **UUID Primary Keys**: All tables use UUID primary keys for better distributed system support
2. **Junction Tables**: Many-to-many relationships are handled via junction tables
3. **JSONB Storage**: Flexible data storage for dynamic fields and metadata
4. **Audit Trails**: All tables include created_at and updated_at timestamps
5. **Data Integrity**: Foreign key constraints and check constraints ensure data quality

## Main Entity Tables

### Accounts (`accounts`)
- **Purpose**: Multi-tenant account management
- **Key Features**: 
  - Subscription tier management (free, pro, enterprise)
  - Usage tracking and billing cycle management
  - Rate limiting and quota enforcement

### Transactions (`transactions`)
- **Purpose**: Core transaction risk assessment records
- **Key Features**:
  - Risk scoring (0.01-99.99 scale)
  - Risk level classification (low, medium, high, very_high)
  - Disposition recommendations (accept, reject, review, test)
  - Event type tracking
  - Custom input storage via JSONB

### Devices (`devices`)
- **Purpose**: Device fingerprinting and tracking
- **Key Features**:
  - IP address geolocation and risk analysis
  - Browser fingerprinting via User-Agent
  - Session tracking and analysis
  - Device risk scoring
  - First/last seen tracking for velocity analysis

### Email Addresses (`email_addresses`)
- **Purpose**: Email address analysis and risk assessment
- **Key Features**:
  - Email hashing for privacy (MD5)
  - Free/disposable email detection
  - High-risk email flagging
  - Domain-level analysis
  - Historical tracking (first seen dates)

### Addresses (`addresses`)
- **Purpose**: Billing and shipping address verification
- **Key Features**:
  - Full address normalization
  - Geolocation coordinates
  - Risk flagging
  - Phone number verification
  - International address support (ISO codes)

### Credit Cards (`credit_cards`)
- **Purpose**: Payment method intelligence
- **Key Features**:
  - BIN (Bank Identification Number) analysis
  - Card type detection (credit, debit, charge)
  - Issuer information and verification
  - AVS/CVV result tracking
  - 3D Secure status
  - Business/prepaid/virtual card detection

## Junction Tables

The schema uses junction tables to handle complex many-to-many relationships:

- `transaction_devices`: Links transactions to devices
- `transaction_emails`: Links transactions to email addresses  
- `transaction_addresses`: Links transactions to billing/shipping addresses
- `transaction_credit_cards`: Links transactions to payment methods

This design allows:
- A single transaction to have multiple addresses (billing vs shipping)
- Device/email/card reuse across multiple transactions
- Efficient querying and analysis of patterns

## Advanced Features

### Order Management (`orders`, `cart_items`)
- Detailed order information including amount, currency, discounts
- Shopping cart analysis with item-level details
- Gift transaction tracking
- Affiliate and referrer tracking

### Batch Processing (`batches`, `batch_transactions`)
- Asynchronous processing of large transaction volumes
- Progress tracking and error handling
- External ID mapping for customer correlation
- Webhook notifications on completion

### Reporting System (`transaction_reports`)
- Feedback loop for machine learning improvement
- Chargeback and fraud outcome tracking
- False positive reporting
- Performance analytics support

### Webhook System (`webhooks`)
- Event-driven notifications
- Configurable event subscriptions
- Success/failure tracking
- Secret-based authentication

### Security & Performance

#### Rate Limiting (`rate_limits`)
- Per-account, per-endpoint rate limiting
- Sliding window implementation
- Automatic cleanup of old windows

#### API Key Management (`api_keys`)
- Secure key storage (hashed)
- Permission-based access control
- Expiration and activity tracking
- Key rotation support

#### Risk Factor Analysis (`risk_factors`)
- Detailed explanation of risk score components
- Factor-specific multipliers and reasoning
- Machine learning feature storage
- Audit trail for risk decisions

#### IP Risk Caching (`ip_risk_cache`)
- Performance optimization for repeated IP lookups
- Geolocation and trait caching
- TTL-based cache expiration
- Risk score caching

## Data Types and Standards

### Standards Compliance
- **Country Codes**: ISO 3166-1 alpha-2 (e.g., "US", "CA")
- **Region Codes**: ISO 3166-2 subdivision codes
- **Currency Codes**: ISO 4217 (e.g., "USD", "EUR")
- **Timestamps**: All timestamps are stored with timezone information
- **IP Addresses**: PostgreSQL INET type for both IPv4 and IPv6

### Security Considerations
- **Email Hashing**: Emails are stored as MD5 hashes for privacy
- **API Key Security**: Keys are hashed before storage
- **Webhook Secrets**: Webhook secrets are hashed
- **Credit Card Data**: Only non-sensitive identifiers stored (BIN, last 4 digits)

## Indexing Strategy

The schema includes strategic indexes for:
- **Primary Lookups**: Account ID, transaction ID, external transaction ID
- **Time-based Queries**: Created dates, billing cycles, rate limit windows
- **Risk Analysis**: Risk scores, IP addresses, country codes
- **Performance**: High-frequency query patterns

## Triggers and Automation

- **Timestamp Updates**: Automatic `updated_at` timestamp maintenance
- **Data Validation**: Check constraints for enums and business rules
- **Referential Integrity**: Foreign key constraints with cascade options

## Scalability Considerations

The design supports horizontal scaling through:
- **UUID Keys**: Globally unique identifiers
- **Partitioning-Ready**: Time-based partitioning for large tables
- **JSONB Flexibility**: Schema evolution without migrations
- **Caching Layers**: Built-in caching tables for performance
- **Batch Processing**: Asynchronous processing support

## Future Extensions

The schema is designed to accommodate:
- **Machine Learning Features**: Risk factor and metadata storage
- **Advanced Analytics**: Time-series data and aggregations
- **Additional Data Sources**: External intelligence integration
- **Compliance Features**: GDPR, PCI DSS data handling
- **Multi-region Deployment**: Geographic data distribution

This database design provides a solid foundation for a production-grade fraud detection platform while maintaining flexibility for future enhancements and scaling requirements. 