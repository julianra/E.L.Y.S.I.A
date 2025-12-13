// ======================================================================
// 📍 FILE: elysia_core/src/http.rs
// 📝 Minimal HTTP API for ELYSIA Core
//
//     This is the ONLY external HTTP boundary of the Kernel.
//     - No domain logic
//     - No AI logic
//     - No module logic
//
//     Fase 1:
//       - Admin bootstrap
//       - Status
//       - AI execute (simple forward stub)
//
// ======================================================================

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
// ======================================================================
//  ADMIN EXISTS
// ======================================================================
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
// ======================================================================
//  CREATE ADMIN
// ======================================================================
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
// ======================================================================
//  LOGIN
// ======================================================================
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
// ======================================================================
//  STATUS
// ======================================================================
//

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub version: String,
    pub db: String,
    pub modules: usize,
}

async fn status(_state: Arc<KernelState>) -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "online".into(),
        version: "2.1".into(),
        db: "ok".into(),
        modules: 0,
    })
}

//
// ======================================================================
//  AI EXECUTE — FASE 1 FORWARD STUB
// ======================================================================
//

#[derive(Deserialize)]
pub struct AiExecuteRequest {
    pub question: String,
    pub mode: String,
}

#[derive(Serialize)]
pub struct AiExecuteResponse {
    pub reply: Option<String>,
    pub error: Option<String>,
}

// ======================================================================
//  AI EXECUTE — STRICT CONTRACT VALIDATION
// ======================================================================

async fn ai_execute(
    _state: Arc<KernelState>,
    Json(req): Json<AiExecuteRequest>,
) -> Json<AiExecuteResponse> {
    log::info!(
        "[CORE][AI] Incoming request | question=\"{}\" mode=\"{}\"",
        req.question,
        req.mode
    );

    let client = reqwest::Client::new();

    let payload = serde_json::json!({
        "input": req.question
    });

    log::info!("[CORE][AI] Sending payload to AI: {}", payload);

    let response = client
        .post("http://127.0.0.1:8123/generate")
        .json(&payload)
        .send()
        .await;

    let resp = match response {
        Ok(r) => r,
        Err(e) => {
            log::error!("[CORE][AI] AI unreachable: {}", e);
            return Json(AiExecuteResponse {
                reply: None,
                error: Some("AI unreachable".into()),
            });
        }
    };

    let json: serde_json::Value = match resp.json().await {
        Ok(j) => j,
        Err(_) => {
            log::error!("[CORE][AI] Invalid JSON from AI");
            return Json(AiExecuteResponse {
                reply: None,
                error: Some("Invalid AI JSON".into()),
            });
        }
    };

    log::info!("[CORE][AI] Raw AI response: {}", json);

    // -----------------------------
    // STRICT CONTRACT VALIDATION
    // -----------------------------

    let success = json.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
    let reply = json.get("reply").and_then(|v| v.as_str());
    let error = json.get("error").and_then(|v| v.as_str());

    if !success {
        log::error!(
            "[CORE][AI] AI reported failure: {:?}",
            error
        );

        return Json(AiExecuteResponse {
            reply: None,
            error: error.map(|e| e.to_string()),
        });
    }

    let reply = match reply {
        Some(r) => r.to_string(),
        None => {
            log::error!("[CORE][AI] Contract violation: success=true but reply missing");
            return Json(AiExecuteResponse {
                reply: None,
                error: Some("Invalid AI response contract".into()),
            });
        }
    };

    log::info!(
        "[CORE][AI] Forwarding reply to module | reply=\"{}\"",
        reply
    );

    Json(AiExecuteResponse {
        reply: Some(reply),
        error: None,
    })
}

//
// ======================================================================
//  ROUTER BUILDER
// ======================================================================
//

pub fn build_router(state: Arc<KernelState>) -> Router {
    Router::new()

        // ---------- ADMIN ----------
        .route("/auth/has_admin", get({
            let s = state.clone();
            move || has_admin(s.clone())
        }))
        .route("/auth/create_admin", post({
            let s = state.clone();
            move |payload| create_admin(s.clone(), payload)
        }))
        .route("/auth/login", post({
            let s = state.clone();
            move |payload| login(s.clone(), payload)
        }))

        // ---------- STATUS ----------
        .route("/status", get({
            let s = state.clone();
            move || status(s.clone())
        }))

        // ---------- AI ----------
        .route("/ai/execute", post({
            let s = state.clone();
            move |payload| ai_execute(s.clone(), payload)
        }))
}
