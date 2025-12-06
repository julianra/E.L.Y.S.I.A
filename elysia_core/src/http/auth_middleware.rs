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

impl AuthSubject {
    pub fn is_admin(&self) -> bool {
        matches!(self, AuthSubject::User(u) if u == "admin")
    }
}

fn is_public(path: &str) -> bool {
    path.starts_with("/health")
        || path.starts_with("/status")
        || path.starts_with("/auth/")
        || path.starts_with("/pair/")
}

pub async fn auth_layer(
    Extension(state): Extension<KernelState>,
    mut req: Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode> {

    let path = req.uri().path().to_string();

    if is_public(&path) {
        return Ok(next.run(req).await);
    }

    let Some(auth) = req.headers().get("Authorization") else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let token = auth.to_str().unwrap_or("").trim_start_matches("Bearer ");

    if let Some(username) = validate_user_token(token) {
        req.extensions_mut().insert(AuthSubject::User(username));
        return Ok(next.run(req).await);
    }

    if let Some(device_id) = validate_device_token(token, &state) {
        req.extensions_mut().insert(AuthSubject::Device(device_id));
        return Ok(next.run(req).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}
