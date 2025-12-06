// ======================================================================
// 📍 FILE: elysia_core/src/pairing/mod.rs
//
// 📝 BESCHRIJVING:
//   Backend voor ELYSIA Pairing 2.0
//
//   Device onboarding:
//      • generate_nonce() → UI handshake
//      • complete_pairing() → DB insert + token
//      • create_device_token() → dev.<id>.<hmac>
//
//   Enterprise LAN Security:
//      Device secret staat in /elysia/device_secret
// ======================================================================

use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha = Hmac<Sha256>;

// ------------------------------------------------------------
// SECRET LOADING
// ------------------------------------------------------------

fn load_device_secret() -> Vec<u8> {
    use std::fs;

    let base = dirs::data_local_dir().unwrap().join("elysia");
    let file = base.join("device_secret");

    if file.exists() {
        return fs::read(file).expect("Failed reading device_secret");
    }

    let s = rand::random::<[u8; 32]>().to_vec();
    fs::create_dir_all(&base).ok();
    fs::write(&file, &s).expect("Failed writing device_secret");
    s
}

fn device_hmac() -> HmacSha {
    HmacSha::new_from_slice(&load_device_secret()).unwrap()
}

// ------------------------------------------------------------
// PUBLIC TOKEN FUNCTIONS
// ------------------------------------------------------------

pub fn create_device_token(device_id: &str) -> String {
    let mut mac = device_hmac();
    mac.update(device_id.as_bytes());
    let sig = mac.finalize().into_bytes();
    format!("dev.{device_id}.{}", hex::encode(sig))
}

pub fn validate_device_token(token: &str, state: &crate::kernel::KernelState)
    -> Option<String>
{
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 || parts[0] != "dev" {
        return None;
    }

    let device_id = parts[1];
    let sig_hex   = parts[2];

    // Check DB
    {
        let conn = state.ctx.db();
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM devices WHERE id = ?1)",
            [device_id],
            |r| r.get(0),
        ).ok()?;

        if !exists { return None; }
    }

    // Verify signature
    let mut mac = device_hmac();
    mac.update(device_id.as_bytes());
    let expected = mac.finalize().into_bytes();

    if hex::encode(expected) == sig_hex {
        Some(device_id.to_string())
    } else {
        None
    }
}

// ------------------------------------------------------------
// API TYPES
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
// UTILS
// ------------------------------------------------------------

pub fn generate_nonce() -> String {
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}
