// ======================================================================
// 📍 FILE: elysia_core/src/db.rs
//
// 📝 BESCHRIJVING:
//   Voert ALLE SQL migraties uit:
//     - elysia_core/migrations/*.sql
//     - modules/*/migrations/*.sql
//
//   Voert elk bestand uit in alfabetische volgorde.
// ======================================================================

use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run_migrations(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut all_sql_files = vec![];

    // 1. Core migraties
    collect_sql_files("elysia_core/migrations", &mut all_sql_files);

    // 2. Module migraties
    collect_module_migrations("modules", &mut all_sql_files);

    // 3. Sorteren op naam
    all_sql_files.sort();

    // 4. SQL uitvoeren
    for file in all_sql_files {
        let sql = fs::read_to_string(&file)?;
        conn.execute_batch(&sql)?;
    }

    Ok(())
}

// ======================================================================
// Verzamelt alle *.sql files in een directory
// ======================================================================
fn collect_sql_files(dir: &str, out: &mut Vec<String>) {
    let path = Path::new(dir);
    if !path.exists() { return; }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().and_then(|x| x.to_str()) == Some("sql") {
                out.push(p.to_string_lossy().to_string());
            }
        }
    }
}

// ======================================================================
// Verzamelt migraties van ALLE modules
// modules/<module>/migrations/*.sql
// ======================================================================
fn collect_module_migrations(base_dir: &str, out: &mut Vec<String>) {
    let base = Path::new(base_dir);
    if !base.exists() { return; }

    for entry in fs::read_dir(base).unwrap() {
        let module_dir = entry.unwrap().path();
        let migration_dir = module_dir.join("migrations");
        collect_sql_files(migration_dir.to_str().unwrap(), out);
    }
}
