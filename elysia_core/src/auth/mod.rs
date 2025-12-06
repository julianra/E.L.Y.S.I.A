// ======================================================================
// 📍 FILE: elysia_core/src/auth/mod.rs
// 📝 Beschrijving:
//   User-authenticatie voor ELYSIA Core
//   - Hashen + valideren van wachtwoorden (Argon2)
//   - User-tokens met HMAC (usr.<username>.<signature>)
//   - Laden van user-secret vanuit databestand
// ======================================================================

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::kernel::KernelState;
use axum::{extract::State, Json};
use rusqlite::params;

type HmacSha256 = Hmac<Sha256>;

// -----------------------------------------------------
// SECRET LOADING
// -----------------------------------------------------

fn load_user_secret() -> Vec<u8> {
    use std::fs;

    let base = dirs::data_local_dir().unwrap().join("elysia");
    let file = base.join("user_secret");

    if file.exists() {
        return fs::read(file).expect("Failed reading user_secret");
    }

    let s = rand::random::<[u8; 32]>().to_vec();
    fs::create_dir_all(&base).ok();
    fs::write(&file, &s).expect("Failed writing user_secret");
    s
}

fn user_hmac() -> HmacSha256 {
    HmacSha256::new_from_slice(&load_user_secret()).expect("HMAC init failed")
}

// -----------------------------------------------------
// PASSWORD SECURITY
// -----------------------------------------------------

fn hash_password(pw: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(pw.as_bytes(), &salt)
        .expect("pw hash fail")
        .to_string()
}

fn verify_password(pw: &str, hash: &str) -> bool {
    let parsed = PasswordHash::new(hash).ok();
    let Some(parsed) = parsed else { return false };
    Argon2::default().verify_password(pw.as_bytes(), &parsed).is_ok()
}

// -----------------------------------------------------
// USER TOKENS → usr.<username>.<signature>
// -----------------------------------------------------

pub fn create_user_token(username: &str) -> String {
    let mut mac = user_hmac();
    mac.update(username.as_bytes());
    let sig = mac.finalize().into_bytes();
    format!("usr.{username}.{}", hex::encode(sig))
}

pub fn validate_user_token(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 || parts[0] != "usr" {
        return None;
    }

    let username = parts[1];
    let sig_hex = parts[2];

    let mut mac = user_hmac();
    mac.update(username.as_bytes());

    if hex::encode(mac.finalize().into_bytes()) == sig_hex {
        Some(username.to_string())
    } else {
        None
    }
}

// -----------------------------------------------------
// API STRUCTS
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
        .expect("Failed admin check");

    Json(InitialState { admin_exists: exists })
}

/// POST /auth/create_admin
pub async fn create_admin(
    State(state): State<KernelState>,
    Json(body): Json<CreateAdminRequest>,
) -> Json<serde_json::Value> {
    let conn = state.ctx.db();

    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE role = 'admin')",
            [],
            |r| r.get(0),
        )
        .unwrap();

    if exists {
        return Json(serde_json::json!({
            "success": false, "error": "Admin already exists"
        }));
    }

    let hash = hash_password(&body.password);

    conn.execute(
        "INSERT INTO users (username, password_hash, role) VALUES (?1, ?2, 'admin')",
        params![body.username, hash],
    )
    .expect("Failed inserting admin");

    Json(serde_json::json!({ "success": true }))
}

/// POST /auth/login
pub async fn login(
    State(state): State<KernelState>,
    Json(body): Json<LoginRequest>,
) -> Json<serde_json::Value> {
    let conn = state.ctx.db();

    let row = conn.query_row(
        "SELECT password_hash FROM users WHERE username = ?1",
        params![body.username],
        |r| r.get::<_, String>(0),
    );

    let Ok(hash) = row else {
        return Json(serde_json::json!({
            "success": false, "error": "Invalid credentials"
        }));
    };

    if !verify_password(&body.password, &hash) {
        return Json(serde_json::json!({
            "success": false, "error": "Invalid credentials"
        }));
    }

    let token = create_user_token(&body.username);

    Json(serde_json::json!({ "success": true, "token": token }))
}
