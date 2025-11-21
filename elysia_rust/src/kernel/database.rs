use rusqlite::{Connection};
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

        conn.execute(
            "CREATE TABLE IF NOT EXISTS agenda (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                duration_minutes INTEGER,
                created_at TEXT
            )",
            [],
        )?;

        Ok(())
    }
}
