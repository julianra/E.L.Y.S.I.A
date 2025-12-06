// ======================================================================
// 📍 FILE: elysia_core/src/http/auth_middleware.rs
//
// 📝 DEEL 1 – SECURITY BASELINE
//     - Validate user token
//     - Validate device token
//     - Public vs protected routes
//
//   🔐 Pairing-mode / pending devices komen pas in DEEL 2.
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

/// Global security middleware
pub async fn auth_layer(
    Extension(state): Extension<KernelState>,
    mut req: Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_string();

    // Public routes
    if is_public(&path) {
        return Ok(next.run(req).await);
    }

    // Authorization header required
    let Some(auth) = req.headers().get("Authorization") else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let auth_str = auth.to_str().unwrap_or("");
    if !auth_str.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_str[7..];

    // Try user-token
    if let Some(username) = validate_user_token(token) {
        req.extensions_mut().insert(AuthSubject::User(username));
        return Ok(next.run(req).await);
    }

    // Try device-token
    if let Some(device_id) = validate_device_token(token, &state) {
        req.extensions_mut().insert(AuthSubject::Device(device_id));
        return Ok(next.run(req).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}
