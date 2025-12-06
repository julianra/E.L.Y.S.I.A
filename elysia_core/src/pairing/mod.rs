// ======================================================================
// 📍 FILE: elysia_core/src/pairing/mod.rs
//
// 📝 DEEL 1 – SECURITY BASELINE
//     - Device token helpers (same behaviour as before)
//     - Nonce generators
//     - Device secret hashing
//     - insert_device()
//
//   🚫 GEEN pairing-mode
//   🚫 GEEN pending devices
//   🚫 GEEN pairing code
//
//   We maken het enkel proper & veilig als fundament voor DEEL 2.
//
// ======================================================================

use rand::RngCore;
use rand::rngs::OsRng;
use rusqlite::params;

use crate::kernel::KernelState;

use serde::{Serialize, Deserialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sha2::Digest;

type HmacSha = Hmac<Sha256>;

// ------------------------------------------------------------
// DATA STRUCTS
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
}

#[derive(Serialize)]
pub struct PairCompleteResponse {
    pub success: bool,
    pub device_id: String,
    pub device_token: String,
}

// ------------------------------------------------------------
// GENERATORS
// ------------------------------------------------------------

pub fn generate_nonce() -> String {
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

pub fn generate_device_secret() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

pub fn hash_secret(secret: &str) -> String {
    let digest = Sha256::digest(secret.as_bytes());
    hex::encode(digest)
}

// ------------------------------------------------------------
// DEVICE TOKEN (same behaviour as before)
// ------------------------------------------------------------

pub fn create_device_token(device_id: &str) -> String {
    let mut mac = HmacSha::new_from_slice(b"ELYISA_DEVICE_SECRET")
        .expect("HMAC init failed");

    mac.update(device_id.as_bytes());
    let sig = mac.finalize().into_bytes();

    format!("dev.{}.{}", device_id, hex::encode(sig))
}

/// Validates the device token. Returns Some(device_id) if valid.
/// SAME behaviour as before, but now clean & central.
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

    // HMAC check
    let mut mac = HmacSha::new_from_slice(b"ELYISA_DEVICE_SECRET")
        .expect("HMAC init failed");

    mac.update(device_id.as_bytes());
    let expected = mac.finalize().into_bytes();

    if hex::encode(expected) != sig_hex {
        return None;
    }

    // DB check: does device exist?
    let conn = state.ctx.db();
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM devices WHERE id = ?)",
            params![device_id],
            |r| r.get(0),
        )
        .unwrap_or(false);

    if !exists {
        return None;
    }

    Some(device_id.to_string())
}

// ------------------------------------------------------------
// DB insert (unchanged behaviour)
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
    .expect("Failed to insert paired device");
}
