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

use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

pub fn init_database() -> Result<(Pool<SqliteConnectionManager>, String), String> {
    let data_dir = dirs::data_local_dir()
        .ok_or("No local data dir")?
        .join("elysia");

    std::fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data dir: {}", e))?;

    let db_path = data_dir.join("elysia.db");
    let db_path_str = db_path.to_string_lossy().to_string();

    let manager = SqliteConnectionManager::file(&db_path);
    let pool = Pool::new(manager)
        .map_err(|e| format!("Pool creation error: {}", e))?;

    // WAL mode:
    {
        let conn = pool.get().map_err(|e| e.to_string())?;
        conn.execute_batch("PRAGMA journal_mode = WAL;")
            .map_err(|e| format!("Failed to enable WAL: {}", e))?;
    }

    Ok((pool, db_path_str))
}
