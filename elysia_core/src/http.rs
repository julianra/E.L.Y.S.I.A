// elysia_core/src/http.rs
// ======================================================================
// 📍 FILE: elysia_core/src/http.rs
// 📝 HTTP API module for ELYSIA Core.
// ======================================================================
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::{
    kernel::KernelState,
    security::{
        hash_password,
        verify_password,
        get_user_password_hash,
        user_exists,
        insert_user,
        create_user_token,
    },
};

//
// =============================
//  ADMIN EXISTS
// =============================
//

#[derive(Serialize)]
pub struct AdminExistsResponse {
    pub exists: bool,
}

async fn has_admin(state: Arc<KernelState>) -> Json<AdminExistsResponse> {
    let exists = user_exists(&state, "admin");
    Json(AdminExistsResponse { exists })
}

//
// =============================
//  CREATE ADMIN
// =============================
//

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

async fn create_admin(
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

//
// =============================
//  LOGIN
// =============================
//

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

async fn login(
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

//
// =============================
//  ROUTER BUILDER
// =============================
//

pub fn build_router(state: Arc<KernelState>) -> Router {
    Router::new()
        .route(
            "/auth/has_admin",
            get({
                let s = state.clone();
                move || has_admin(s.clone())
            }),
        )
        .route(
            "/auth/create_admin",
            post({
                let s = state.clone();
                move |payload| create_admin(s.clone(), payload)
            }),
        )
        .route(
            "/auth/login",
            post({
                let s = state.clone();
                move |payload| login(s.clone(), payload)
            }),
        )
}
