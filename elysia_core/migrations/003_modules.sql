-- ======================================================================
-- 📍 FILE: elysia_core/migrations/003_modules.sql
--
-- 📝 BESCHRIJVING:
--   Persistente module state voor ELYSIA.
--   Houdt pairing en permissions bij.
--   Wordt NIET automatisch gevuld.
-- ======================================================================

CREATE TABLE IF NOT EXISTS modules (
    id TEXT PRIMARY KEY,
    paired INTEGER NOT NULL DEFAULT 0,
    permissions_json TEXT NOT NULL DEFAULT '{}',
    paired_at TEXT NULL
);
