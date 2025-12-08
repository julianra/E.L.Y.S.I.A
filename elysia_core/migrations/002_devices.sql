-- ======================================================================
-- 📍 FILE: elysia_core/migrations/002_devices.sql
--
-- 📝 BESCHRIJVING:
--   Device registry tabel voor ELYSIA Pairing 2.0 (High Security).
--
--   Iedere Orbit client (gsm, tablet, desktop-app) wordt geregistreerd
--   als een 'device' met:
--     - device_id   (publieke ID – UUID)
--     - name        (bv. "Julian's Galaxy")
--     - secret_hash (argon2id hash van het device secret)
--     - created_at  (registratie)
--     - last_seen   (laatste connectie)
--     - os          (android, windows, linux, ios)
--     - model       (S21+, Pixel 7, etc.)
-- ======================================================================

CREATE TABLE IF NOT EXISTS devices (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    secret_hash TEXT NOT NULL,
    os TEXT NOT NULL,
    model TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

