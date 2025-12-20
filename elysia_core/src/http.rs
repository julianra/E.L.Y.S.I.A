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
    Json,
    Router,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{Response, IntoResponse},
    body::Body,
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::KernelState;
async fn status(state: Arc<KernelState>) -> Json<StatusResponse> {
    let count = state.modules.read().await.len(); // 🔒 single source of truth

    Json(StatusResponse {
        status: "online".into(),
        version: "2.1".into(),
        db: "ok".into(),
        modules: count,
    })
}


use crate::security::{
    hash_password,
    verify_password,
    get_user_password_hash,
    user_exists,
    insert_user,
    create_user_token,
    validate_user_token,
};


async fn http_access_guard(
    state: Arc<KernelState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let path = req.uri().path();
    let method = req.method().as_str();

    let admin_exists = user_exists(&state, "admin");

    // ==================================================
    // BOOTSTRAP MODE — NO ADMIN
    // ==================================================
    if !admin_exists {
        let allowed = matches!(
            (method, path),
            ("GET",  "/auth/has_admin")
                | ("POST", "/auth/create_admin")
                | ("POST", "/auth/login")
                | ("GET",  "/status")
        );

        if !allowed {
            return StatusCode::FORBIDDEN.into_response();
        }

        return next.run(req).await;
    }

    // ==================================================
    // ADMIN MODE — ADMIN EXISTS
    // ==================================================

    let public = matches!(
        (method, path),
        ("GET",  "/auth/has_admin")
            | ("POST", "/auth/login")
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

// ======================================================================
// MODULES (Global Module View — fase 1: native only)
// ======================================================================

#[derive(Serialize)]
pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub kind: String,      // "module"
    pub installed: bool,
    pub loaded: bool,
    pub paired: bool,     // false (fase 1)
}

async fn list_modules(state: Arc<KernelState>) -> Json<Vec<ModuleInfo>> {
    use crate::plugins::loader::PluginLoader;

    // 1️⃣ Scan plugins folder → installed truth
    let loader = PluginLoader::new("plugins");
    let found = loader.scan();

    // 2️⃣ Lees actieve registry → loaded truth
    let registry = state.modules.read().await;

    let modules = found
        .into_iter()
        .map(|pl| {
            let name = pl.manifest.name.clone();

            let loaded = registry
                .iter()
                .any(|m| m.name() == name);

            ModuleInfo {
                id: name.clone(),        // fase 1: name == id
                name,
                kind: "module".into(),
                installed: true,
                loaded,
                paired: false,           // fase 1
            }
        })
        .collect();

    Json(modules)
}

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

        // ---------- AI (blocked by guard) ----------
        .route("/ai/execute", post({
            let s = state.clone();
            move |payload| ai_execute(s.clone(), payload)
        }))

        // ---------- MODULES ----------
        .route("/modules", get({
            let s = state.clone();
            move || list_modules(s.clone())
        }))

        // ---------- GLOBAL GUARD ----------
        .layer(middleware::from_fn({
            let s = state.clone();
            move |req, next| http_access_guard(s.clone(), req, next)
        }))
}
