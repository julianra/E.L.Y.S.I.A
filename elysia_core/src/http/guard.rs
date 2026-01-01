// ======================================================================
// 📍 FILE: elysia_core/src/http/guard.rs
// ======================================================================

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::KernelState;
use crate::security::{user_exists, validate_user_token};

pub async fn http_access_guard(
    state: Arc<KernelState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let path = req.uri().path();
    let method = req.method().as_str();

    let admin_exists = user_exists(&state, "admin");

    // ---------------- BOOTSTRAP ----------------
    if !admin_exists {
        let allowed = matches!(
            (method, path),
            ("GET", "/auth/has_admin")
                | ("POST", "/auth/create_admin")
                | ("POST", "/auth/login")
                | ("GET", "/status")
        );

        if !allowed {
            return StatusCode::FORBIDDEN.into_response();
        }

        return next.run(req).await;
    }

    // ---------------- ADMIN MODE ----------------
    let public = matches!(
        (method, path),
        ("GET", "/auth/has_admin") | ("POST", "/auth/login") | ("POST", "/modules/upload")
    );

    if public {
        return next.run(req).await;
    }

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let Some(username) = validate_user_token(token) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    if username != "admin" {
        return StatusCode::FORBIDDEN.into_response();
    }

    next.run(req).await
}
