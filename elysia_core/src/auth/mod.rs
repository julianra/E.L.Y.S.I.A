// ======================================================================
// 📍 FILE: elysia_core/src/auth/mod.rs
//
// 📝 DEEL 1 – SECURITY BASELINE
//     - User token helpers (create_user_token / validate_user_token)
//     - Password hashing (argon2id)
//     - Login & initial admin creation
//     - Localhost-only rule voor /auth/create_admin
//
//   🔐 Functioneel blijft het token EXACT hetzelfde als vroeger,
//      zodat niets breekt. We structureren enkel de boel.
//
// ======================================================================

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

use crate::kernel::KernelState;

use axum::{
    extract::{State, ConnectInfo},
    Json,
};
use rusqlite::params;
use std::net::SocketAddr;

// -----------------------------------------------------
// Data structs
// -----------------------------------------------------

#[derive(Serialize)]
pub struct InitialState {
    pub admin_exists: bool,
}

#[derive(Deserialize)]
pub struct CreateAdminRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: String,
}

// -----------------------------------------------------
// Password hashing helpers (argon2id)
// -----------------------------------------------------

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

fn verify_password(password: &str, hash: &str) -> bool {
    let parsed = PasswordHash::new(hash).ok();
    let Some(parsed) = parsed else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

// -----------------------------------------------------
// USER TOKEN – SAME BEHAVIOUR AS BEFORE
// -----------------------------------------------------

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Generates the user token; same content as before.
pub fn create_user_token(username: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(b"ELYISA_SUPER_SECRET_CHANGE_THIS")
        .expect("HMAC init failed");

    mac.update(username.as_bytes());
    let signature = mac.finalize().into_bytes();

    format!("{}.{}", username, hex::encode(signature))
}

/// Validates the user token.
pub fn validate_user_token(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 2 {
        return None;
    }

    let username = parts[0];
    let sig_hex = parts[1];

    let mut mac = HmacSha256::new_from_slice(b"ELYISA_SUPER_SECRET_CHANGE_THIS")
        .expect("HMAC init failed");

    mac.update(username.as_bytes());
    let expected = mac.finalize().into_bytes();

    if hex::encode(expected) == sig_hex {
        Some(username.to_string())
    } else {
        None
    }
}

// -----------------------------------------------------
// HTTP HANDLERS
// -----------------------------------------------------

/// GET /auth/initial_state
pub async fn get_initial_state(State(state): State<KernelState>) -> Json<InitialState> {
    let conn = state.ctx.db();

    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE role = 'admin')",
            [],
            |r| r.get(0),
        )
        .expect("Failed to query admin existence");

    Json(InitialState { admin_exists: exists })
}

/// POST /auth/create_admin  (LOCALHOST ONLY!)
///
/// ONLY allowed from 127.0.0.1 or ::1.
/// Remote LAN devices MUST NOT be able to create the first admin.
pub async fn create_admin(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<KernelState>,
    Json(body): Json<CreateAdminRequest>,
) -> Json<serde_json::Value> {
    // ------------------------------
    // 🔒 LOCALHOST-ONLY CHECK
    // ------------------------------
    let ip = addr.ip();

    let is_localhost = ip.is_loopback() || ip.to_string() == "127.0.0.1";
    if !is_localhost {
        return Json(serde_json::json!({
            "success": false,
            "error": "Admin creation allowed only from localhost"
        }));
    }

    let conn = state.ctx.db();

    // Prevent double-admin
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE role = 'admin')",
            [],
            |r| r.get(0),
        )
        .expect("Failed to query admin existence");

    if exists {
        return Json(serde_json::json!({
            "success": false,
            "error": "Admin already exists"
        }));
    }

    let hash = hash_password(&body.password);

    conn.execute(
        "INSERT INTO users (username, password_hash, role) VALUES (?, ?, 'admin')",
        params![body.username, hash],
    )
    .expect("Failed to insert admin user");

    Json(serde_json::json!({ "success": true }))
}

/// POST /auth/login
pub async fn login(
    State(state): State<KernelState>,
    Json(body): Json<LoginRequest>,
) -> Json<serde_json::Value> {
    let conn = state.ctx.db();

    let row = conn.query_row(
        "SELECT password_hash FROM users WHERE username = ?",
        params![body.username],
        |r| r.get::<_, String>(0),
    );

    let Ok(hash) = row else {
        return Json(serde_json::json!({
            "success": false,
            "error": "Invalid credentials"
        }));
    };

    if !verify_password(&body.password, &hash) {
        return Json(serde_json::json!({
            "success": false,
            "error": "Invalid credentials"
        }));
    }

    let token = create_user_token(&body.username);

    Json(serde_json::json!({
        "success": true,
        "token": token
    }))
}
