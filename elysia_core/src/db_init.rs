// ======================================================================
// 📍 FILE: elysia_core/src/db_init.rs
//
// 📝 BESCHRIJVING:
//   Initialiseert de ELYSIA database:
//     - maakt lokale data folder
//     - opent SQLite file
//     - activeert WAL mode (sneller + minder locks)
//     - maakt een r2d2 connection pool
//
//   Deze DB-setup is future-proof en ondersteunt zware modules
//   zoals CARE, SHIELD en JUNK zonder bottlenecks.
// ======================================================================

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::fs;
use std::path::PathBuf;

pub fn init_database() -> Result<(Pool<SqliteConnectionManager>, String), anyhow::Error> {
    // 1. Data folder bepalen
    let data_dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot locate local data directory"))?
        .join("elysia");

    fs::create_dir_all(&data_dir)?;

    // 2. Database pad
    let db_path = data_dir.join("elysia.db");
    let db_str = db_path.to_string_lossy().to_string();

    // 3. Connection manager
    let manager = SqliteConnectionManager::file(&db_path);

    // 4. Pool maken
    let pool = Pool::new(manager)?;

    // 5. WAL mode activeren
    {
        let conn = pool.get()?;
        conn.execute_batch("PRAGMA journal_mode = WAL;")?;
    }

    Ok((pool, db_str))
}
