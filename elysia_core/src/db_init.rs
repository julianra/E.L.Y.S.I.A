// ======================================================================
// 📍 FILE: elysia_core/src/db_init.rs
//
// 📝 BESCHRIJVING:
//   Initialiseert de database. Probeert eerst AppData, anders fallback.
//   - Maakt directory aan als die niet bestaat
//   - Opent SQLite database
//
// 🔧 RETURN:
//   (Connection, pad_string)
// ======================================================================

use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

pub fn init_database() -> Result<(Connection, String), Box<dyn std::error::Error>> {
    // 1. Pad bepalen (AppData → fallback)
    let db_path = get_default_db_path();

    // 2. Parent-directory aanmaken
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 3. SQLite openen (maakt file automatisch aan)
    let conn = Connection::open(&db_path)?;

    Ok((conn, db_path.to_string_lossy().to_string()))
}

// ======================================================================
// Bepaalt database-pad
// ======================================================================
fn get_default_db_path() -> PathBuf {
    // Windows: %APPDATA%\Elysia
    if let Some(appdata) = dirs::config_dir() {
        let mut p = appdata;
        p.push("Elysia");
        p.push("elysia.db");
        return p;
    }

    // fallback → lokale map
    PathBuf::from("elysia_local.db")
}
