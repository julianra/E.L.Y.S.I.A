CREATE TABLE IF NOT EXISTS marthe_tasks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    duration_minutes INTEGER,
    exact_start TEXT,
    exact_end TEXT,
    priority TEXT,
    location TEXT,
    energy_cost INTEGER,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);
