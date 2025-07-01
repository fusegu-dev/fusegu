use crate::auth::{ApiKeyService, AuthContext};
use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

/// Authentication middleware that validates API keys
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Parse Bearer token
    let api_key = if let Some(token) = auth_header.strip_prefix("Bearer ") {
        token
    } else if let Some(token) = auth_header.strip_prefix("bearer ") {
        token
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // TODO: Get API key service from app state
    let api_key_service = ApiKeyService::new();

    // Authenticate the API key
    let (account, api_key_record) = api_key_service
        .authenticate(api_key)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Create auth context and add to request extensions
    let auth_context = AuthContext::new(account, api_key_record);
    request.extensions_mut().insert(auth_context);

    Ok(next.run(request).await)
}

/// Extract auth context from request
pub fn extract_auth_context(request: &Request) -> Option<&AuthContext> {
    request.extensions().get::<AuthContext>()
} 