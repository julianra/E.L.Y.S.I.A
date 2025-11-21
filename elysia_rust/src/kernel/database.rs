use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

pub struct Database {
    pub conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new() -> Result<Self> {
        // Absoluut pad bepalen naar de map waar de executable draait
        let mut path = std::env::current_exe().unwrap();

        // Executable folder:
        //  target/debug/elysia_rust.exe
        //  -> ga 1 niveau omhoog naar target/debug/
        path.pop();

        // /data folder
        path.push("data");

        // zorg dat map bestaat
        std::fs::create_dir_all(&path)
            .expect("Kon data folder niet aanmaken");

        // naar database file
        path.push("elysia.db");

        println!("[DB] Database pad = {:?}", path);

        // Open of creëer database op exact deze locatie
        let conn = Connection::open(path)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn init(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // Tabel 'agenda' bestaat zeker
        conn.execute(
            "CREATE TABLE IF NOT EXISTS agenda (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                duration_minutes INTEGER NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }
}
