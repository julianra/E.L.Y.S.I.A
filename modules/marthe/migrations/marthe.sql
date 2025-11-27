-- ======================================================================
-- 📍 FILE: elysia/modules/marthe/migrations/marthe.sql
--
-- 📝 BESCHRIJVING:
--   Database migraties voor de MARTHE-module.
--   Bevat de tabellen nodig voor planning, tasks en metadata.
--
-- 🔧 TAKEN:
--   - Aanmaken van de marthe_tasks tabel
--   - Basisstructuur voor dagschema's / tijdsblokken
--   - Wordt automatisch uitgevoerd door ELYSIA Core tijdens boot
-- ======================================================================

CREATE TABLE IF NOT EXISTS marthe_tasks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    exact_start TEXT,
    exact_end TEXT,
    duration_minutes INTEGER,
    priority TEXT,
    location TEXT,
    energy_cost INTEGER,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);
