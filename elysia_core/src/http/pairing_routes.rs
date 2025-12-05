// ======================================================================
// 📍 FILE: elysia_core/src/http/pairing_routes.rs
//
// 📝 BESCHRIJVING:
//   HTTP endpoints voor ELYSIA Pairing 2.0.
//   Compatibel met de SvelteKit UI die je al hebt.
//
//   Endpoints:
//     - GET  /pair/status
//     - GET  /pair/init
//     - POST /pair/complete
//
//   → Hiermee koppelt Orbit correct met ELYSIA Core.
// ======================================================================

use axum::{Json, Router, routing::{get, post}, extract::State};
use crate::{kernel::KernelState, pairing::*};
use serde_json::json;
use uuid::Uuid;

pub fn pairing_routes(state: KernelState) -> Router {
    Router::new()
        .route("/pair/status", get(get_status))
        .route("/pair/init", get(start_pairing))
        .route("/pair/complete", post(complete_pairing))
        .with_state(state)
}

// ------------------------------------------------------------
// GET /pair/status
// ------------------------------------------------------------

async fn get_status(State(_state): State<KernelState>) -> Json<serde_json::Value> {
    Json(json!({
        "paired": false,
        "kernel": {
            "version": "2.0",
            "node_id": "core-node",
            "capabilities": ["kernel", "planner", "router"]
        }
    }))
}

// ------------------------------------------------------------
// GET /pair/init
// ------------------------------------------------------------

async fn start_pairing(State(_state): State<KernelState>) -> Json<PairInitResponse> {
    let nonce = generate_nonce();

    Json(PairInitResponse {
        node_id: "core-node".into(),
        nonce,
        version: "2.0".into(),
        capabilities: vec![
            "kernel".into(),
            "planner".into(),
            "router".into(),
        ],
    })
}

// ------------------------------------------------------------
// POST /pair/complete
// ------------------------------------------------------------

async fn complete_pairing(
    State(state): State<KernelState>,
    Json(body): Json<PairCompleteRequest>,
) -> Json<PairCompleteResponse> {

    let device_id = Uuid::new_v4().to_string();
    let device_secret = generate_device_secret();
    let device_hash = hash_secret(&device_secret);

    insert_device(
        &state,
        &device_id,
        &body.device_name,
        &device_hash,
        body.os,
        body.model,
    );

    let token = create_device_token(&device_id, &device_secret);

    Json(PairCompleteResponse {
        success: true,
        device_id,
        device_token: token
    })
}
