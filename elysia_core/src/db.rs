// ======================================================================
// 📍 FILE: elysia_core/src/db.rs
//
// 📝 BESCHRIJVING:
//   Migration loader voor SQLite. Voert AUTOMATISCH alle SQL scripts uit:
//      - elysia_core/migrations/*.sql
//      - modules/<module>/migrations/*.sql
//
//   Bestandsnamen worden alfabetisch uitgevoerd. Perfect voor versiebeheer.
//
// ======================================================================

use rusqlite::Connection;
use std::fs;
use std::path::{Path};

/// Voer ALLE migrations uit: eerst core, dan module-specifiek.
pub fn run_migrations(conn: &Connection) -> anyhow::Result<()> {
    let mut migrations = vec![];

    collect_sql_files("elysia_core/migrations", &mut migrations);
    collect_module_migrations("modules", &mut migrations);

    migrations.sort();

    for path in migrations {
        let sql = fs::read_to_string(&path)?;
        conn.execute_batch(&sql)?;
    }

    Ok(())
}

// ---------------------------------------------------------
// Helpers
// ---------------------------------------------------------

fn collect_sql_files(dir: &str, out: &mut Vec<String>) {
    let path = Path::new(dir);

    if !path.exists() {
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().and_then(|x| x.to_str()) == Some("sql") {
                out.push(p.to_string_lossy().to_string());
            }
        }
    }
}

fn collect_module_migrations(base: &str, out: &mut Vec<String>) {
    let base = Path::new(base);
    if !base.exists() {
        return;
    }

    for entry in fs::read_dir(base).unwrap() {
        let module_dir = entry.unwrap().path();
        let migration_dir = module_dir.join("migrations");

        if migration_dir.exists() {
            collect_sql_files(migration_dir.to_str().unwrap(), out);
        }
    }
}
