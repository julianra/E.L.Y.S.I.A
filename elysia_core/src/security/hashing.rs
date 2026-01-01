// ======================================================================
// 📍 FILE: elysia_core/src/security/auth/hashing.rs
// 📝 Secure password hashing + verification for Kernel LAN auth.
// ======================================================================

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand_core::OsRng;

/// Hash a password using Argon2id.
/// Kernel uses this for user + device secrets.
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

/// Verify a password against its stored hash.
/// Returns true if valid.
pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed = PasswordHash::new(hash).ok();
    let Some(parsed) = parsed else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}
