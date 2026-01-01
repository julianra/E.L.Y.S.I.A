// ======================================================================
// 📍 FILE: elysia_core/src/security/modules.rs
// 📝 ROLE:
//   Persistente module state helpers voor ELYSIA.
//   - Opslag van pairingstatus
//   - Opslag van permissies (nog geen enforcement)
//
//   ❗ Pure DB-helpers
//   ❗ Geen HTTP
//   ❗ Geen flows
// ======================================================================

use anyhow::Result;
use rusqlite::{Connection, params};

#[derive(Debug, Clone)]
pub struct ModuleState {
    pub id: String,
    pub paired: bool,
    pub permissions_json: String,
    pub paired_at: Option<String>,
}

pub fn get_module_state(conn: &Connection, id: &str) -> Result<Option<ModuleState>> {
    let mut stmt =
        conn.prepare("SELECT id, paired, permissions_json, paired_at FROM modules WHERE id = ?")?;

    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(ModuleState {
            id: row.get(0)?,
            paired: row.get::<_, i64>(1)? != 0,
            permissions_json: row.get(2)?,
            paired_at: row.get(3)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn ensure_module_row(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("INSERT OR IGNORE INTO modules (id) VALUES (?)", params![id])?;
    Ok(())
}

pub fn set_module_paired(conn: &Connection, id: &str, paired: bool) -> Result<()> {
    ensure_module_row(conn, id)?;

    if paired {
        conn.execute(
            "UPDATE modules SET paired = 1, paired_at = CURRENT_TIMESTAMP WHERE id = ?",
            params![id],
        )?;
    } else {
        conn.execute(
            "UPDATE modules SET paired = 0, paired_at = NULL WHERE id = ?",
            params![id],
        )?;
    }

    Ok(())
}
