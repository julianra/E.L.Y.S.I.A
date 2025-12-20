// ======================================================================
// 📍 FILE: elysia_core/src/http/auth.rs
// ======================================================================

use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::KernelState;
use crate::security::{
    hash_password,
    verify_password,
    get_user_password_hash,
    user_exists,
    insert_user,
    create_user_token,
};

#[derive(Serialize)]
pub struct AdminExistsResponse {
    pub exists: bool,
}

pub async fn has_admin(state: Arc<KernelState>) -> Json<AdminExistsResponse> {
    let exists = user_exists(&state, "admin");
    Json(AdminExistsResponse { exists })
}

#[derive(Deserialize)]
pub struct CreateAdminRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateAdminResponse {
    pub success: bool,
    pub error: Option<String>,
}

pub async fn create_admin(
    state: Arc<KernelState>,
    Json(req): Json<CreateAdminRequest>,
) -> Json<CreateAdminResponse> {
    if user_exists(&state, &req.username) {
        return Json(CreateAdminResponse {
            success: false,
            error: Some("Admin already exists".into()),
        });
    }

    let hash = hash_password(&req.password);

    if let Err(e) = insert_user(&state, &req.username, &hash, "admin") {
        return Json(CreateAdminResponse {
            success: false,
            error: Some(format!("DB error: {}", e)),
        });
    }

    Json(CreateAdminResponse {
        success: true,
        error: None,
    })
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: Option<String>,
    pub error: Option<String>,
}

pub async fn login(
    state: Arc<KernelState>,
    Json(req): Json<LoginRequest>,
) -> Json<LoginResponse> {
    let Some(stored) = get_user_password_hash(&state, &req.username) else {
        return Json(LoginResponse {
            success: false,
            token: None,
            error: Some("User not found".into()),
        });
    };

    if !verify_password(&req.password, &stored) {
        return Json(LoginResponse {
            success: false,
            token: None,
            error: Some("Invalid credentials".into()),
        });
    }

    let token = create_user_token(&req.username);

    Json(LoginResponse {
        success: true,
        token: Some(token),
        error: None,
    })
}
