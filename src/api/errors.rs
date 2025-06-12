//! Error handling for API endpoints

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// API result type alias
pub type ApiResult<T> = Result<T, ApiError>;

/// Error codes for machine-readable responses
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    BadRequest,
    Unauthorized,
    NotFound,
    ValidationError,
    InternalError,
}

/// API error types
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Validation error: {0}")]
    Validation(String),
}

/// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorCode,
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
