// ======================================================================
// 📍 FILE: elysia_core/src/http/pairing_routes.rs
//
// 📝 Pairing HTTP API:
//   Public (device):
//     - GET  /pair/status
//     - GET  /pair/init
//     - POST /pair/complete
//
//   Admin (alleen via admin-token):
//     - GET  /pairing/status
//     - POST /pairing/enable
//     - POST /pairing/disable
//
//   Pairing-mode state zit in KernelContext.
// ======================================================================

use axum::{
    Json,
    Router,
    extract::State,
    routing::{get, post},
};
use serde_json::json;
use rand::Rng;
use uuid::Uuid;

use crate::{
    kernel::KernelState,
    pairing::{
        PairInitResponse,
        PairCompleteRequest,
        PairCompleteResponse,
        generate_nonce,
        generate_device_secret,
        hash_secret,
        insert_device,
        create_device_token,
    },
};

// ------------------------------------------------------------
// ROUTER
// ------------------------------------------------------------

pub fn pairing_routes(state: KernelState) -> Router {
    Router::new()
        .route("/pair/status", get(public_status))
        .route("/pair/init", get(public_init))
        .route("/pair/complete", post(public_complete))

        .route("/pairing/status", get(admin_status))
        .route("/pairing/enable", post(admin_enable))
        .route("/pairing/disable", post(admin_disable))

        .with_state(state)
}

// ------------------------------------------------------------
// PUBLIC: GET /pair/status
// ------------------------------------------------------------

async fn public_status(State(state): State<KernelState>) -> Json<serde_json::Value> {
    Json(json!({
        "pairing_active": state.ctx.pairing_is_active(),
        "kernel": {
            "version": "2.0",
            "node_id": "core-node",
            "capabilities": ["kernel", "planner", "router"]
        }
    }))
}

// ------------------------------------------------------------
// PUBLIC: GET /pair/init
// ------------------------------------------------------------

async fn public_init() -> Json<PairInitResponse> {
    Json(PairInitResponse {
        node_id: "core-node".into(),
        nonce: generate_nonce(),
        version: "2.0".into(),
        capabilities: vec!["kernel".into(), "planner".into(), "router".into()],
    })
}

// ------------------------------------------------------------
// PUBLIC: POST /pair/complete
// ------------------------------------------------------------

async fn public_complete(
    State(state): State<KernelState>,
    Json(body): Json<PairCompleteRequest>,
) -> Json<PairCompleteResponse> {

    if !state.ctx.pairing_is_active() {
        return Json(PairCompleteResponse::error("Pairing mode disabled"));
    }

    let code = state.ctx.pairing_code.read().unwrap().clone();
    if code.as_deref() != Some(body.code.as_str()) {
        return Json(PairCompleteResponse::error("Invalid pairing code"));
    }

    let device_id = Uuid::new_v4().to_string();
    let secret = generate_device_secret();
    let hash = hash_secret(&secret);

    insert_device(
        &state,
        &device_id,
        &body.device_name,
        &hash,
        body.os,
        body.model,
    );

    let token = create_device_token(&device_id);

    Json(PairCompleteResponse::success(device_id, token))
}

// ------------------------------------------------------------
// ADMIN: GET /pairing/status
// ------------------------------------------------------------

async fn admin_status(State(state): State<KernelState>) -> Json<serde_json::Value> {
    let active = state.ctx.pairing_is_active();
    let code = state.ctx.pairing_code.read().unwrap().clone();

    Json(json!({
        "active": active,
        "code": code
    }))
}

// ------------------------------------------------------------
// ADMIN: POST /pairing/enable
// ------------------------------------------------------------

async fn admin_enable(
    State(state): State<KernelState>,
    Json(body): Json<serde_json::Value>,
) -> Json<serde_json::Value> {

    let minutes = body.get("minutes").and_then(|v| v.as_u64()).unwrap_or(5);
    let minutes = minutes.clamp(1, 60);

    let code = format!("{:06}", rand::thread_rng().gen_range(0..=999999));

    {
    *state.ctx.pairing_enabled.write().unwrap() = true;
    *state.ctx.pairing_code.write().unwrap() = Some(code.clone());
    *state.ctx.pairing_expires_at.write().unwrap() =
        Some(std::time::Instant::now() + std::time::Duration::from_secs(minutes * 60));
}


    Json(json!({
        "success": true,
        "code": code,
        "minutes": minutes
    }))
}

// ------------------------------------------------------------
// ADMIN: POST /pairing/disable
// ------------------------------------------------------------

async fn admin_disable(State(state): State<KernelState>) -> Json<serde_json::Value> {
    *state.ctx.pairing_enabled.write().unwrap() = false;
    *state.ctx.pairing_code.write().unwrap() = None;

    Json(json!({ "success": true }))
}
