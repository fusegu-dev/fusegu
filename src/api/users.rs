//! User API endpoints

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
        requests::{UserRequest, UserUpdate},
        responses::{UserResponse, UserList},
        common::{RiskLevel, Pagination, Links},
    },
    services::UserService,
};

use super::{ApiError, ApiResult};

/// Query parameters for listing users
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct UserListQuery {
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default)]
    pub offset: i32,
    pub risk_level: Option<RiskLevel>,
    pub is_flagged: Option<bool>,
    pub is_verified: Option<bool>,
    #[serde(default = "default_sort")]
    pub sort: String,
}

fn default_limit() -> i32 {
    20
}

fn default_sort() -> String {
    "-created_at".to_string()
}

/// Create a new user profile
#[utoipa::path(
    post,
    path = "/v1/users",
    tags = ["Users"],
    summary = "Create a new user profile",
    description = "Create a new user profile for cross-transaction risk tracking and analysis.",
    request_body = UserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Bad Request - Invalid request format"),
        (status = 401, description = "Unauthorized - Invalid API key"),
        (status = 409, description = "Conflict - User already exists"),
        (status = 422, description = "Validation Error - Request validation failed"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn create_user(
    State(_config): State<Config>,
    Json(request): Json<UserRequest>,
) -> ApiResult<(StatusCode, Json<UserResponse>)> {
    // Validate request
    if let Err(validation_errors) = validator::Validate::validate(&request) {
        return Err(ApiError::Validation(
            format!("Request validation failed: {:?}", validation_errors)
        ));
    }

    // TODO: Get user service from app state
    // For now, create a mock response
    let user_id = Uuid::new_v4();
    
    let response = UserResponse {
        id: user_id,
        external_user_id: request.external_user_id,
        user_hash: request.user_hash,
        risk_score: Some(rust_decimal::Decimal::new(1542, 2)), // 15.42
        risk_level: Some(RiskLevel::Low),
        total_transactions: 0,
        successful_transactions: 0,
        failed_transactions: 0,
        chargeback_count: 0,
        first_transaction_at: None,
        last_transaction_at: None,
        is_verified: false,
        is_flagged: false,
        flags: vec![],
        metadata: request.metadata,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        links: Links {
            self_link: Some(format!("/v1/users/{}", user_id)),
            transactions: Some(format!("/v1/users/{}/transactions", user_id)),
            risk_analysis: Some(format!("/v1/users/{}/risk-analysis", user_id)),
            next: None,
            prev: None,
            first: None,
            last: None,
            insights: None,
            factors: None,
            report: None,
        },
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/v1/users/{user_id}",
    tags = ["Users"],
    summary = "Get user by ID",
    description = "Retrieve detailed information about a specific user.",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details retrieved successfully", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized - Invalid API key"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_user(
    State(_config): State<Config>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<UserResponse>> {
    // TODO: Get user service from app state and fetch from database
    // For now, return a mock response
    let response = UserResponse {
        id: user_id,
        external_user_id: Some("user_12345".to_string()),
        user_hash: Some("5d41402abc4b2a76b9719d911017c592".to_string()),
        risk_score: Some(rust_decimal::Decimal::new(1542, 2)), // 15.42
        risk_level: Some(RiskLevel::Low),
        total_transactions: 45,
        successful_transactions: 42,
        failed_transactions: 3,
        chargeback_count: 0,
        first_transaction_at: Some(chrono::Utc::now() - chrono::Duration::days(30)),
        last_transaction_at: Some(chrono::Utc::now() - chrono::Duration::hours(2)),
        is_verified: true,
        is_flagged: false,
        flags: vec![],
        metadata: None,
        created_at: chrono::Utc::now() - chrono::Duration::days(30),
        updated_at: chrono::Utc::now(),
        links: Links {
            self_link: Some(format!("/v1/users/{}", user_id)),
            transactions: Some(format!("/v1/users/{}/transactions", user_id)),
            risk_analysis: Some(format!("/v1/users/{}/risk-analysis", user_id)),
            next: None,
            prev: None,
            first: None,
            last: None,
            insights: None,
            factors: None,
            report: None,
        },
    };

    Ok(Json(response))
}

/// List users with filters
#[utoipa::path(
    get,
    path = "/v1/users",
    tags = ["Users"],
    summary = "List users",
    description = "Retrieve a paginated list of users with optional filters.",
    params(UserListQuery),
    responses(
        (status = 200, description = "List of users retrieved successfully", body = UserList),
        (status = 400, description = "Bad Request - Invalid query parameters"),
        (status = 401, description = "Unauthorized - Invalid API key"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn list_users(
    State(_config): State<Config>,
    Query(query): Query<UserListQuery>,
) -> ApiResult<Json<UserList>> {
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

    // TODO: Get user service from app state and fetch from database
    // For now, return a mock response
    let users = vec![
        UserResponse {
            id: Uuid::new_v4(),
            external_user_id: Some("user_12345".to_string()),
            user_hash: Some("5d41402abc4b2a76b9719d911017c592".to_string()),
            risk_score: Some(rust_decimal::Decimal::new(1542, 2)), // 15.42
            risk_level: Some(RiskLevel::Low),
            total_transactions: 45,
            successful_transactions: 42,
            failed_transactions: 3,
            chargeback_count: 0,
            first_transaction_at: Some(chrono::Utc::now() - chrono::Duration::days(30)),
            last_transaction_at: Some(chrono::Utc::now() - chrono::Duration::hours(2)),
            is_verified: true,
            is_flagged: false,
            flags: vec![],
            metadata: None,
            created_at: chrono::Utc::now() - chrono::Duration::days(30),
            updated_at: chrono::Utc::now(),
            links: Links {
                self_link: Some(format!("/v1/users/{}", Uuid::new_v4())),
                transactions: Some(format!("/v1/users/{}/transactions", Uuid::new_v4())),
                risk_analysis: Some(format!("/v1/users/{}/risk-analysis", Uuid::new_v4())),
                next: None,
                prev: None,
                first: None,
                last: None,
                insights: None,
                factors: None,
                report: None,
            },
        },
    ];

    let pagination = Pagination {
        limit: query.limit,
        offset: query.offset,
        total: 1,
        has_more: false,
    };

    let response = UserList {
        users,
        pagination,
        links: Links {
            self_link: Some("/v1/users".to_string()),
            next: None,
            prev: None,
            first: Some("/v1/users?offset=0&limit=20".to_string()),
            last: Some("/v1/users?offset=0&limit=20".to_string()),
            insights: None,
            factors: None,
            report: None,
            transactions: None,
            risk_analysis: None,
        },
    };

    Ok(Json(response))
}

/// Update user profile
#[utoipa::path(
    patch,
    path = "/v1/users/{user_id}",
    tags = ["Users"],
    summary = "Update user profile",
    description = "Update user profile information, risk settings, or verification status.",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    request_body = UserUpdate,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 400, description = "Bad Request - Invalid request format"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized - Invalid API key"),
        (status = 422, description = "Validation Error - Request validation failed"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn update_user(
    State(_config): State<Config>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UserUpdate>,
) -> ApiResult<Json<UserResponse>> {
    // Validate request
    if let Err(validation_errors) = validator::Validate::validate(&request) {
        return Err(ApiError::Validation(
            format!("Request validation failed: {:?}", validation_errors)
        ));
    }

    // TODO: Get user service from app state and update in database
    // For now, return a mock response
    let response = UserResponse {
        id: user_id,
        external_user_id: Some("user_12345".to_string()),
        user_hash: Some("5d41402abc4b2a76b9719d911017c592".to_string()),
        risk_score: Some(rust_decimal::Decimal::new(1542, 2)), // 15.42
        risk_level: Some(RiskLevel::Low),
        total_transactions: 45,
        successful_transactions: 42,
        failed_transactions: 3,
        chargeback_count: 0,
        first_transaction_at: Some(chrono::Utc::now() - chrono::Duration::days(30)),
        last_transaction_at: Some(chrono::Utc::now() - chrono::Duration::hours(2)),
        is_verified: request.is_verified.unwrap_or(true),
        is_flagged: request.is_flagged.unwrap_or(false),
        flags: request.flags.unwrap_or_default(),
        metadata: request.metadata,
        created_at: chrono::Utc::now() - chrono::Duration::days(30),
        updated_at: chrono::Utc::now(),
        links: Links {
            self_link: Some(format!("/v1/users/{}", user_id)),
            transactions: Some(format!("/v1/users/{}/transactions", user_id)),
            risk_analysis: Some(format!("/v1/users/{}/risk-analysis", user_id)),
            next: None,
            prev: None,
            first: None,
            last: None,
            insights: None,
            factors: None,
            report: None,
        },
    };

    Ok(Json(response))
} 