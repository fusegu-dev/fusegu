//! Error handling for API endpoints

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;

/// API result type alias
pub type ApiResult<T> = Result<T, ApiError>;

/// Error codes for machine-readable responses
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(
    title = "ErrorCode",
    description = "Machine-readable error codes for API responses",
    example = "bad_request"
)]
pub enum ErrorCode {
    /// Bad request from client - Invalid request parameters or format
    BadRequest,
    /// Authentication required - Authentication credentials required
    Unauthorized,
    /// Resource not found - Requested resource does not exist
    NotFound,
    /// Validation failed - Request validation failed
    ValidationError,
    /// Internal server error - Unexpected server error occurred
    InternalError,
}

/// API error types
#[derive(Error, Debug)]
pub enum ApiError {
    /// Internal server error
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    /// Invalid JSON in request
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),

    /// Bad request with details
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Resource not found
    #[error("Not found")]
    NotFound,

    /// Authentication required
    #[error("Unauthorized")]
    Unauthorized,

    /// Validation error with details
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Error response structure
#[derive(Debug, Serialize, ToSchema)]
#[schema(
    title = "ErrorResponse",
    description = "Standard error response format",
    example = json!({
        "error": "bad_request",
        "message": "Invalid request parameters"
    })
)]
pub struct ErrorResponse {
    /// Machine-readable error code
    #[schema(example = "bad_request")]
    pub error: ErrorCode,
    /// Human-readable error message
    #[schema(example = "Invalid request parameters")]
    pub message: String,
}

impl ApiError {
    fn to_response(&self) -> (StatusCode, ErrorResponse) {
        match self {
            ApiError::Internal(e) => {
                tracing::error!(error = %e, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        error: ErrorCode::InternalError,
                        message: "Internal server error".to_string(),
                    },
                )
            },
            ApiError::InvalidJson(e) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: ErrorCode::BadRequest,
                    message: format!("Invalid JSON: {}", e),
                },
            ),
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: ErrorCode::BadRequest,
                    message: msg.clone(),
                },
            ),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    error: ErrorCode::NotFound,
                    message: "Resource not found".to_string(),
                },
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    error: ErrorCode::Unauthorized,
                    message: "Authentication required".to_string(),
                },
            ),
            ApiError::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse {
                    error: ErrorCode::ValidationError,
                    message: msg.clone(),
                },
            ),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_response) = self.to_response();
        (status, Json(error_response)).into_response()
    }
}
