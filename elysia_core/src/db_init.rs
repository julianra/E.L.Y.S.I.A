// ======================================================================
// 📍 FILE: elysia_core/src/db_init.rs
//
// Foolproof SQLite init voor ELYSIA
// ======================================================================

use sqlx::SqlitePool;
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================
// 1. Preferred AppData DB Location
// ============================================================
fn preferred_appdata_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|base| base.join("Elysia").join("elysia.db"))
}

// ============================================================
// 2. Fallback: project-root/data/elysia.db 
// (GEEN current_dir, 100% stabiel)
// ============================================================
fn fallback_project_path() -> PathBuf {
    let core_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let project_root = core_dir
        .parent()
        .expect("Could not determine project root");
    project_root.join("data").join("elysia.db")
}

// ============================================================
// Helper: directory maken
// ============================================================
fn ensure_parent_exists(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

// ============================================================
// Debug logging
// ============================================================
fn debug_paths() {
    log::warn!("[DB-DEBUG] Preferred AppData: {:?}", preferred_appdata_path());
    log::warn!("[DB-DEBUG] Fallback path: {:?}", fallback_project_path());
    log::warn!("[DB-DEBUG] Working directory: {:?}", std::env::current_dir());
}

// ============================================================
// Public functie om SQLite te initialiseren
// ============================================================
pub async fn init_sqlite() -> (SqlitePool, PathBuf) {
    debug_paths();

    // -------------------------
    // 1. PROBEER APPDATA
    // -------------------------
    if let Some(p) = preferred_appdata_path() {
        log::info!("[DB] Trying AppData DB at: {}", p.display());

        if ensure_parent_exists(&p).is_ok() {
            let url = format!("sqlite:///{}", p.to_string_lossy());

            if let Ok(pool) = SqlitePool::connect(&url).await {
                log::info!("[DB] Using AppData DB");
                return (pool, p);
            } else {
                log::warn!("[DB] AppData DB failed, falling back...");
            }
        }
    }

    // -------------------------
    // 2. FALLBACK → PROJECT ROOT
    // -------------------------
    let fallback = fallback_project_path();
    log::info!("[DB] Trying fallback DB at: {}", fallback.display());

    ensure_parent_exists(&fallback)
        .expect("Cannot create fallback DB directory");

    let url = format!("sqlite:///{}", fallback.to_string_lossy());

    match SqlitePool::connect(&url).await {
        Ok(pool) => {
            log::info!("[DB] Using fallback SQLite DB");
            (pool, fallback)
        }
        Err(e) => panic!(
            "Could not open fallback SQLite DB: {:?}\nPath: {}",
            e,
            fallback.display()
        ),
    }
}
