//! Account API endpoints

use axum::{
    extract::State,
    response::Json,
};

use crate::{
    config::Config,
    models::{
        responses::AccountInfo,
        common::SubscriptionTier,
    },
};

use super::{ApiError, ApiResult};

/// Get account information
#[utoipa::path(
    get,
    path = "/v1/account",
    tags = ["Account"],
    summary = "Get account information",
    description = "Retrieve current account status including subscription details, usage statistics, and remaining credits/queries.",
    responses(
        (status = 200, description = "Account information retrieved successfully", body = AccountInfo),
        (status = 401, description = "Unauthorized - Invalid API key"),
        (status = 500, description = "Internal Server Error"),
    )
)]
pub async fn get_account(
    State(_config): State<Config>,
) -> ApiResult<Json<AccountInfo>> {
    // TODO: Get account service from app state and fetch from database
    // For now, return a mock response
    let response = AccountInfo {
        account_id: "acc_12345".to_string(),
        subscription_tier: SubscriptionTier::Pro,
        funds_remaining: rust_decimal::Decimal::new(985075, 2), // 9850.75
        monthly_quota: 100000,
        queries_used_this_month: 15642,
        queries_remaining_this_month: 84358,
        billing_cycle_start: "2025-06-01".to_string(),
        billing_cycle_end: "2025-06-30".to_string(),
    };

    Ok(Json(response))
} 