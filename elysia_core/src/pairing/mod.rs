// ======================================================================
// 📍 FILE: elysia_core/src/pairing/mod.rs
//
// 📝 BESCHRIJVING:
//   Interne pairing-backend van ELYSIA Core.
//   Wordt gebruikt door pairing_routes.rs.
//
//   Functies:
//     - generate_nonce()
//     - generate_device_secret()
//     - hash_secret()
//     - insert_device()
//     - create_device_token()
//
//   Tabellen (aangemaakt via migraties):
//     - devices(id, name, secret_hash, os, model, created_at)
//
//   Dit is klaar voor:
//     • Orbit pairing
//     • Portal pairing
//     • SvelteKit UI onboarding
// ======================================================================

use rand::RngCore;
use rand::rngs::OsRng;
use rusqlite::params;
use crate::kernel::KernelState;
use serde::{Serialize, Deserialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sha2::Digest;


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
    let digest = sha2::Sha256::digest(secret.as_bytes());
    hex::encode(digest)
}

// ------------------------------------------------------------
// DEVICE TOKEN (HMAC)
// ------------------------------------------------------------

pub fn create_device_token(device_id: &str, secret: &str) -> String {
    type HmacSha = Hmac<Sha256>;

    let mut mac = HmacSha::new_from_slice(b"ELYISA_DEVICE_SECRET")
        .expect("HMAC init failed");

    mac.update(device_id.as_bytes());
    mac.update(secret.as_bytes());

    let sig = mac.finalize().into_bytes();

    format!("{}.{}", device_id, hex::encode(sig))
}

// ------------------------------------------------------------
// DATABASE INSERT
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
