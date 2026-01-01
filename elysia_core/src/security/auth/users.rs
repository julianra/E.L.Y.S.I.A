// ======================================================================
// 📍 FILE: elysia_core/src/security/auth/users.rs
// 📝 Database helpers for user records (no HTTP, no flows).
// ======================================================================

use crate::kernel::KernelState;
use anyhow::Result;
use rusqlite::params;

/// Check if a user exists.
pub fn user_exists(state: &KernelState, username: &str) -> bool {
    let conn = state.ctx.db();

    conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?)",
        params![username],
        |r| r.get(0),
    )
    .unwrap_or(false)
}

/// Get a user's password hash.
/// Returns None if user does not exist.
pub fn get_user_password_hash(state: &KernelState, username: &str) -> Option<String> {
    let conn = state.ctx.db();

    conn.query_row(
        "SELECT password_hash FROM users WHERE username = ?",
        params![username],
        |r| r.get::<_, String>(0),
    )
    .ok()
}

/// Insert a new user into the database.
/// Kernel does NOT enforce flows (admin-only, first-run etc.).
pub fn insert_user(
    state: &KernelState,
    username: &str,
    password_hash: &str,
    role: &str,
) -> Result<()> {
    let conn = state.ctx.db();

    conn.execute(
        "INSERT INTO users (username, password_hash, role)
         VALUES (?, ?, ?)",
        params![username, password_hash, role],
    )?;

    Ok(())
}
