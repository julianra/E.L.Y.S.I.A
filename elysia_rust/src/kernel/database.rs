/// Database module for E.L.Y.S.I.A application
/// This module handles the initialization and connection to the SQLite database.
/// It ensures that the necessary tables are created if they do not already exist.
// elysia_rust/src/kernel/database.rs
// ===============================================

use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::fs;

pub struct Database {
    pub conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new() -> rusqlite::Result<Self> {
        fs::create_dir_all("data").ok();
        let conn = Connection::open("data/elysia.db")?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn init(&self) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();

        // ----------------------------------------------------
        // TABLE: agenda (nu met calendar_day_id, start_time, end_time)
        // ----------------------------------------------------
        conn.execute(
            "CREATE TABLE IF NOT EXISTS agenda (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                duration_minutes INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                calendar_day_id TEXT,
                start_time TEXT,
                end_time TEXT
            )",
            [],
        )?;

        // ----------------------------------------------------
        // TABLE: calendar_day
        // ----------------------------------------------------
        conn.execute(
            "CREATE TABLE IF NOT EXISTS calendar_day (
                id TEXT PRIMARY KEY,
                date TEXT NOT NULL UNIQUE,
                total_tasks INTEGER DEFAULT 0,
                total_duration INTEGER DEFAULT 0,
                energy_load INTEGER DEFAULT 0
            )",
            [],
        )?;

        Ok(())
    }

}
