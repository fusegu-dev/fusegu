//! Transaction API endpoints

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{ToSchema, IntoParams};

use crate::{
    config::Config,
    models::{
        requests::TransactionRequest,
        responses::{TransactionResponse, TransactionList},
        common::{RiskLevel, Disposition, Pagination, Links},
    },
    services::TransactionService,
};

use super::{ApiError, ApiResult};

/// Query parameters for listing transactions
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct TransactionListQuery {
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default)]
    pub offset: i32,
    pub risk_level: Option<RiskLevel>,
    pub disposition: Option<Disposition>,
    pub from_date: Option<chrono::DateTime<chrono::Utc>>,
    pub to_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "default_sort")]
    pub sort: String,
}

fn default_limit() -> i32 {
    20
}

fn default_sort() -> String {
    "-created_at".to_string()
}

/// Create and score a transaction
#[utoipa::path(
    post,
    path = "/v1/transactions",
    tags = ["Transactions"],
    summary = "Create and score a transaction",
    description = "Submit a new transaction for fraud analysis and receive a risk assessment.",
    request_body = TransactionRequest,
    responses(
        (status = 201, description = "Transaction created and scored successfully", body = TransactionResponse),
        (status = 400, description = "Bad Request - Invalid request format"),
        (status = 401, description = "Unauthorized - Invalid API key"),
        (status = 422, description = "Validation Error - Request validation failed"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn create_transaction(
    State(_config): State<Config>,
    Json(request): Json<TransactionRequest>,
) -> ApiResult<(StatusCode, Json<TransactionResponse>)> {
    // Validate request
    if let Err(validation_errors) = validator::Validate::validate(&request) {
        return Err(ApiError::Validation(
            format!("Request validation failed: {:?}", validation_errors)
        ));
    }

    // TODO: Get transaction service from app state
    // For now, create a mock response
    let transaction_id = Uuid::new_v4();
    let user_id = request.user_id;
    
    let response = TransactionResponse {
        id: transaction_id,
        user_id,
        risk_score: rust_decimal::Decimal::new(245, 2), // 2.45
        risk_level: RiskLevel::Low,
        disposition: Disposition::Accept,
        created_at: chrono::Utc::now(),
        user: None,
        ip_address: None,
        funds_remaining: rust_decimal::Decimal::new(985000, 2), // 9850.00
        queries_remaining: 485,
        warnings: None,
        links: Links {
            self_link: Some(format!("/v1/transactions/{}", transaction_id)),
            insights: Some(format!("/v1/transactions/{}/insights", transaction_id)),
            factors: Some(format!("/v1/transactions/{}/factors", transaction_id)),
            next: None,
            prev: None,
            first: None,
            last: None,
            report: None,
            transactions: None,
            risk_analysis: None,
        },
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Get transaction by ID
#[utoipa::path(
    get,
    path = "/v1/transactions/{transaction_id}",
    tags = ["Transactions"],
    summary = "Get transaction by ID",
    description = "Retrieve detailed information about a specific transaction.",
    params(
        ("transaction_id" = Uuid, Path, description = "Transaction ID")
    ),
    responses(
        (status = 200, description = "Transaction details retrieved successfully", body = TransactionResponse),
        (status = 404, description = "Transaction not found"),
        (status = 401, description = "Unauthorized - Invalid API key"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_transaction(
    State(_config): State<Config>,
    Path(transaction_id): Path<Uuid>,
) -> ApiResult<Json<TransactionResponse>> {
    // TODO: Get transaction service from app state and fetch from database
    // For now, return a mock response
    let response = TransactionResponse {
        id: transaction_id,
        user_id: Some(Uuid::new_v4()),
        risk_score: rust_decimal::Decimal::new(245, 2), // 2.45
        risk_level: RiskLevel::Low,
        disposition: Disposition::Accept,
        created_at: chrono::Utc::now(),
        user: None,
        ip_address: None,
        funds_remaining: rust_decimal::Decimal::new(985000, 2), // 9850.00
        queries_remaining: 485,
        warnings: None,
        links: Links {
            self_link: Some(format!("/v1/transactions/{}", transaction_id)),
            insights: Some(format!("/v1/transactions/{}/insights", transaction_id)),
            factors: Some(format!("/v1/transactions/{}/factors", transaction_id)),
            next: None,
            prev: None,
            first: None,
            last: None,
            report: None,
            transactions: None,
            risk_analysis: None,
        },
    };

    Ok(Json(response))
}

/// List transactions with filters
#[utoipa::path(
    get,
    path = "/v1/transactions",
    tags = ["Transactions"],
    summary = "List transactions",
    description = "Retrieve a paginated list of transactions with optional filters.",
    params(TransactionListQuery),
    responses(
        (status = 200, description = "List of transactions retrieved successfully", body = TransactionList),
        (status = 400, description = "Bad Request - Invalid query parameters"),
        (status = 401, description = "Unauthorized - Invalid API key"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn list_transactions(
    State(_config): State<Config>,
    Query(query): Query<TransactionListQuery>,
) -> ApiResult<Json<TransactionList>> {
    // Validate query parameters
    if query.limit < 1 || query.limit > 100 {
        return Err(ApiError::BadRequest(
            "Limit must be between 1 and 100".to_string()
        ));
    }

    if query.offset < 0 {
        return Err(ApiError::BadRequest(
            "Offset must be non-negative".to_string()
        ));
    }

    // TODO: Get transaction service from app state and fetch from database
    // For now, return a mock response
    let transactions = vec![
        TransactionResponse {
            id: Uuid::new_v4(),
            user_id: Some(Uuid::new_v4()),
            risk_score: rust_decimal::Decimal::new(245, 2), // 2.45
            risk_level: RiskLevel::Low,
            disposition: Disposition::Accept,
            created_at: chrono::Utc::now(),
            user: None,
            ip_address: None,
            funds_remaining: rust_decimal::Decimal::new(985000, 2), // 9850.00
            queries_remaining: 485,
            warnings: None,
            links: Links {
                self_link: Some(format!("/v1/transactions/{}", Uuid::new_v4())),
                insights: Some(format!("/v1/transactions/{}/insights", Uuid::new_v4())),
                factors: Some(format!("/v1/transactions/{}/factors", Uuid::new_v4())),
                ..Default::default()
            },
        },
    ];

    let pagination = Pagination {
        limit: query.limit,
        offset: query.offset,
        total: 1,
        has_more: false,
    };

    let response = TransactionList {
        transactions,
        pagination,
        links: Links {
            self_link: Some("/v1/transactions".to_string()),
            next: None,
            prev: None,
            first: Some("/v1/transactions?offset=0&limit=20".to_string()),
            last: Some("/v1/transactions?offset=0&limit=20".to_string()),
            insights: None,
            factors: None,
            report: None,
            transactions: None,
            risk_analysis: None,
        },
    };

    Ok(Json(response))
} 