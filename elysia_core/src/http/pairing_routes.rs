// ======================================================================
// 📍 FILE: elysia_core/src/http/pairing_routes.rs
//
// 📝 BESCHRIJVING:
//   HTTP endpoints voor ELYSIA Pairing 2.0.
//   - GET  /pair/status
//   - GET  /pair/init
//   - POST /pair/complete
//
//   Enterprise LAN Security compatibel (Axum 0.7)
// ======================================================================

use axum::{
    Json,
    Router,
    routing::{get, post},
    extract::State,
};
use crate::{
    kernel::KernelState,
    pairing::{
        PairInitResponse,
        PairCompleteRequest,
        PairCompleteResponse,
        generate_nonce,
        create_device_token,
    },
};
use serde_json::json;
use uuid::Uuid;
use rusqlite::params;

// ------------------------------------------------------------
// ROUTER
// ------------------------------------------------------------

pub fn pairing_routes(state: KernelState) -> Router {
    Router::new()
        .route("/pair/status", get(get_status))
        .route("/pair/init",   get(start_pairing))
        .route("/pair/complete", post(complete_pairing))
        .with_state(state)
}

// ------------------------------------------------------------
// GET /pair/status
// ------------------------------------------------------------

async fn get_status() -> Json<serde_json::Value> {
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

async fn start_pairing() -> Json<PairInitResponse> {
    Json(PairInitResponse {
        node_id: "core-node".into(),
        nonce: generate_nonce(),
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

    // Device-ID genereren
    let device_id = Uuid::new_v4().to_string();

    // Enterprise token genereren: dev.<id>.<hmac>
    let device_token = create_device_token(&device_id);

    // Device opslaan in DB
    {
        let conn = state.ctx.db();

        conn.execute(
            "INSERT INTO devices (id, name, secret_hash, os, model, created_at)
             VALUES (?1, ?2, '', ?3, ?4, CURRENT_TIMESTAMP)",
            params![device_id, body.device_name, body.os, body.model],
        )
        .expect("Failed to insert device");
    }

    Json(PairCompleteResponse {
        success: true,
        device_id,
        device_token,
    })
}
