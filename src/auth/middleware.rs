// src/auth/middleware.rs
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

#[allow(dead_code)]
pub async fn auth_middleware(
    headers: HeaderMap,
    State(_state): State<crate::api::routes::AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                // Here you'd verify the token against your AuthManager
                // For now, we'll just check if it's not empty
                if !token.is_empty() {
                    // Add user info to request extensions if needed
                    return Ok(next.run(request).await);
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
