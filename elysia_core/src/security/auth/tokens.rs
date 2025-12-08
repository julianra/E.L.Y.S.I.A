// ======================================================================
// 📍 FILE: elysia_core/src/security/auth/tokens.rs
// 📝 User token generation & validation for LAN auth (no HTTP).
// ======================================================================

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

const TOKEN_SECRET: &[u8] = b"ELYISA_SUPER_SECRET_CHANGE_THIS";

/// Create a signed user token.
/// Format: username.signature_hex
pub fn create_user_token(username: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(TOKEN_SECRET)
        .expect("HMAC init failed");

    mac.update(username.as_bytes());
    let signature = mac.finalize().into_bytes();

    format!("{}.{}", username, hex::encode(signature))
}

/// Validate a signed user token.
/// Returns Some(username) if valid, else None.
pub fn validate_user_token(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 2 {
        return None;
    }

    let username = parts[0];
    let sig_hex = parts[1];

    let mut mac = HmacSha256::new_from_slice(TOKEN_SECRET)
        .expect("HMAC init failed");

    mac.update(username.as_bytes());
    let expected = mac.finalize().into_bytes();

    if hex::encode(expected) == sig_hex {
        Some(username.to_string())
    } else {
        None
    }
}
