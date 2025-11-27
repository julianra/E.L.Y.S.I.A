// ======================================================================
// 📍 FILE: elysia_core/src/db.rs
//
// SQLite migraties voor:
//   - Core (elysia_core/migrations/*.sql)
//   - Modules (modules/<name>/migrations/*.sql)
// ======================================================================

use sqlx::{SqlitePool, query};
use std::{fs, path::Path};

pub async fn run_migrations(pool: &SqlitePool) {
    // -------------------------
    // 1. CORE MIGRATIONS
    // -------------------------
    let core_migration_path = Path::new("elysia_core/migrations");

    if core_migration_path.exists() {
        for file in fs::read_dir(core_migration_path).unwrap() {
            let file = file.unwrap();
            let sql = fs::read_to_string(file.path())
                .expect("Could not read core migration file");
            execute_sql_batch(pool, &sql).await;
        }
    }

    // -------------------------
    // 2. MODULE MIGRATIONS
    // -------------------------
    let modules_dir = Path::new("modules");

    if modules_dir.exists() {
        for module in fs::read_dir(modules_dir).unwrap() {
            let module = module.unwrap();
            let migration_path = module.path().join("migrations");

            if migration_path.exists() {
                for file in fs::read_dir(migration_path).unwrap() {
                    let file = file.unwrap();
                    let sql = fs::read_to_string(file.path())
                        .expect("Could not read module migration file");
                    execute_sql_batch(pool, &sql).await;
                }
            }
        }
    }
}

async fn execute_sql_batch(pool: &SqlitePool, sql: &str) {
    for statement in sql.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() {
            query(trimmed).execute(pool).await.unwrap();
        }
    }
}
