-- ======================================================================
-- 📍 FILE: elysia_core/migrations/001_users.sql
--
-- 📝 BESCHRIJVING:
--   Eerste user-migratie voor ELYSIA Core.
--   Definieert de `users` tabel voor authenticatie en autorisatie.
--
--   Kolommen:
--     - id            → Primaire sleutel
--     - username      → Unieke gebruikersnaam
--     - password_hash → Gehashte wachtwoorden (argon2)
--     - role          → 'admin', 'user', 'device', ...
--     - created_at    → Tijdstip van aanmaak
--
--   Deze migratie wordt automatisch uitgevoerd door `db::run_migrations`.
-- ======================================================================

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'user',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
