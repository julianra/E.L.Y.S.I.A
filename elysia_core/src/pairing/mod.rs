// ======================================================================
// 📍 FILE: elysia_core/src/pairing/mod.rs
//
// 📝 Kernlogica voor ELYSIA Pairing 2.0
//     - Nonce generator
//     - Device-secret generator + hashing
//     - Device-token generator + validator
//     - DB insert
//     - Datatypes voor HTTP-laag
// ======================================================================

use rand::RngCore;
use rand::rngs::OsRng;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::kernel::KernelState;

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

type HmacSha = Hmac<Sha256>;

// ------------------------------------------------------------
// HTTP STRUCTS
// ------------------------------------------------------------

#[derive(Serialize)]
pub struct PairInitResponse {
    pub node_id: String,
    pub nonce: String,
    pub version: String,
    pub capabilities: Vec<String>,
}

#[derive(Deserialize)]
pub struct PairCompleteRequest {
    pub device_name: String,
    pub os: String,
    pub model: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct PairCompleteResponse {
    pub success: bool,
    pub device_id: Option<String>,
    pub device_token: Option<String>,
    pub error: Option<String>,
}

impl PairCompleteResponse {
    pub fn success(id: String, token: String) -> Self {
        Self {
            success: true,
            device_id: Some(id),
            device_token: Some(token),
            error: None,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            device_id: None,
            device_token: None,
            error: Some(msg.into()),
        }
    }
}

// ------------------------------------------------------------
// RANDOM GENERATORS
// ------------------------------------------------------------

pub fn generate_nonce() -> String {
    let mut b = [0u8; 16];
    OsRng.fill_bytes(&mut b);
    hex::encode(b)
}

pub fn generate_device_secret() -> String {
    let mut b = [0u8; 32];
    OsRng.fill_bytes(&mut b);
    hex::encode(b)
}

pub fn hash_secret(secret: &str) -> String {
    let d = Sha256::digest(secret.as_bytes());
    hex::encode(d)
}

// ------------------------------------------------------------
// DEVICE TOKEN
// ------------------------------------------------------------

pub fn create_device_token(device_id: &str) -> String {
    let mut mac = HmacSha::new_from_slice(b"ELYSIA_DEVICE_SECRET").expect("HMAC init failed");

    mac.update(device_id.as_bytes());
    let sig = mac.finalize().into_bytes();

    format!("dev.{}.{}", device_id, hex::encode(sig))
}

pub fn validate_device_token(token: &str, state: &KernelState) -> Option<String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    if parts[0] != "dev" {
        return None;
    }

    let device_id = parts[1];
    let sig_hex = parts[2];

    let mut mac = HmacSha::new_from_slice(b"ELYSIA_DEVICE_SECRET").expect("HMAC init failed");

    mac.update(device_id.as_bytes());
    let expected = mac.finalize().into_bytes();

    if hex::encode(expected) != sig_hex {
        return None;
    }

    let conn = state.ctx.db();
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM devices WHERE id = ?)",
            params![device_id],
            |r| r.get(0),
        )
        .unwrap_or(false);

    exists.then(|| device_id.to_string())
}

// ------------------------------------------------------------
// DB INSERT
// ------------------------------------------------------------

pub fn insert_device(
    state: &KernelState,
    id: &str,
    name: &str,
    hash: &str,
    os: String,
    model: String,
) {
    let conn = state.ctx.db();
    conn.execute(
        "INSERT INTO devices (id, name, secret_hash, os, model, created_at)
         VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)",
        params![id, name, hash, os, model],
    )
    .expect("Failed to insert device");
}
