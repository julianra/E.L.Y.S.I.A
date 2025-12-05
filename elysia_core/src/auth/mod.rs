// ======================================================================
// 📍 FILE: elysia_core/src/auth/mod.rs
//
// 📝 BESCHRIJVING:
//   Auth-module van ELYSIA Core. Verantwoordelijk voor:
//     - Password hashing (argon2)
//     - Credentials valideren
//     - Genereren van een eenvoudige HMAC-gebaseerde access token
//     - Basis HTTP-handlers voor:
//         * GET  /auth/initial_state  → Bestaat er al een admin?
//         * POST /auth/create_admin   → Eerste admin aanmaken
//         * POST /auth/login          → Inloggen, token ontvangen
//
//   Deze module vormt de fundering voor:
//     - Onboarding-flow in de Vite/Svelte UI
//     - Later: echte role-checking, sessions, device tokens, enz.
//
//   Let op: dit is al production-minded (argon2 + HMAC), maar
//   nog bewust klein gehouden. Geen JWT-lib of complexe ACL's,
//   zodat de kern leesbaar en beheersbaar blijft.
// ======================================================================

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use crate::kernel::KernelState;
use axum::{extract::State, Json};
use rusqlite::params;

// -----------------------------------------------------
// Data structs voor (de)serialisatie
// -----------------------------------------------------

#[derive(Serialize)]
pub struct InitialState {
    /// True als er minstens één admin-user bestaat.
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
// Password hashing helpers (argon2id, veilig & standard)
// -----------------------------------------------------

fn hash_password(password: &str) -> String {
    // Salt genereren met OS RNG
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hashen met argon2 (argon2id)
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
// Token generation – simpele HMAC token
//
// Formaat: "<username>.<hex_signature>"
//
//  - Secret key is voorlopig hardcoded, maar wordt later
//    vervangen door een configuratie/geheime sleutel
//    opgeslagen in de OS-keystore of config.
// -----------------------------------------------------

fn create_token(username: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac =
        HmacSha256::new_from_slice(b"ELYISA_SUPER_SECRET_CHANGE_THIS").expect("HMAC init failed");
    mac.update(username.as_bytes());
    let signature = mac.finalize().into_bytes();

    format!("{}.{}", username, hex::encode(signature))
}

/// Valideert het token en geeft de username terug als het geldig is.
/// Wordt NU nog niet gebruikt, maar is klaar voor:
///   - /auth/me
///   - middleware voor protected routes
pub fn validate_token(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 2 {
        return None;
    }

    let username = parts[0];
    let sig_hex = parts[1];

    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac =
        HmacSha256::new_from_slice(b"ELYISA_SUPER_SECRET_CHANGE_THIS").expect("HMAC init failed");
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
///
/// Doel:
///   UI moet weten of er al een admin bestaat.
///   - admin_exists = false → Onboarding-flow tonen (admin aanmaken)
///   - admin_exists = true  → Login-screen tonen
pub async fn get_initial_state(State(state): State<KernelState>) -> Json<InitialState> {
    let conn = state.ctx.db();

    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE role = 'admin')",
            [],
            |r| r.get(0),
        )
        .expect("Failed to query admin existence");

    Json(InitialState {
        admin_exists: exists,
    })
}

/// POST /auth/create_admin
///
/// Body: { "username": "...", "password": "..." }
///
/// Gedrag:
///   - Als er al een admin bestaat → error (je mag maar 1 keer onboarden)
///   - Anders → nieuwe admin user aanmaken (role = 'admin')
pub async fn create_admin(
    State(state): State<KernelState>,
    Json(body): Json<CreateAdminRequest>,
) -> Json<serde_json::Value> {
    let conn = state.ctx.db();

    // Voorkom tweede admin-onboarding
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
///
/// Body: { "username": "...", "password": "..." }
///
/// Gedrag:
///   - Zoekt user op basis van username
///   - Valideert password met argon2
///   - Genereert token via HMAC
///   - Stuurt JSON terug: { success: true, token: "<...>" }
///
/// In de Vite/Svelte UI:
///   - Token wordt in memory / localStorage bewaard
///   - Bij elke volgende request wordt `Authorization: Bearer <token>`
///     meegestuurd (dat bouwen we later in).
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

    let token = create_token(&body.username);

    Json(serde_json::json!({
        "success": true,
        "token": token
    }))
}
