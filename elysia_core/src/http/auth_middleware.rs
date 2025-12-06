// ======================================================================
// 📍 FILE: elysia_core/src/http/auth_middleware.rs
//
// 📝 BESCHRIJVING:
//   Enterprise LAN beveiliging voor alle protected routes.
//   - User tokens:  usr.<username>.<hmac>
//   - Device tokens: dev.<device_id>.<hmac>
//   - Axum 0.7 compatibel middleware
// ======================================================================

use axum::{
    http::{Request, StatusCode},
    response::Response,
    Extension,
};
use crate::{
    auth::validate_user_token,
    pairing::validate_device_token,
    kernel::KernelState,
};

#[derive(Clone)]
pub enum AuthSubject {
    User(String),
    Device(String),
}

fn is_public(path: &str) -> bool {
    path.starts_with("/health")
        || path.starts_with("/status")
        || path.starts_with("/auth/")
        || path.starts_with("/pair/")
}

/// Middleware entrypoint
pub async fn auth_layer(
    Extension(state): Extension<KernelState>,
    mut req: Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode>

{
    let path = req.uri().path().to_string();

    // PUBLIC ROUTES
    if is_public(&path) {
        return Ok(next.run(req).await);
    }

    // AUTH HEADER REQUIRED
    let Some(auth) = req.headers().get("Authorization") else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let auth_str = auth.to_str().unwrap_or("");
    if !auth_str.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_str[7..];

    // USER TOKEN?
    if let Some(username) = validate_user_token(token) {
        req.extensions_mut().insert(AuthSubject::User(username));
        return Ok(next.run(req).await);
    }

    // DEVICE TOKEN?
    if let Some(id) = validate_device_token(token, &state) {
        req.extensions_mut().insert(AuthSubject::Device(id));
        return Ok(next.run(req).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}
