// ======================================================================
// 📍 FILE: src/api/kernel.ts
// 📝 ROLE:
//   Centrale HTTP client voor communicatie met de ELYSIA Kernel.
//   - Geen UI-logica
//   - Geen state
//   - Enkel fetch + JSON
// ======================================================================

const BASE = "http://127.0.0.1:2022";

export async function kernelStatus() {
  const r = await fetch(`${BASE}/status`);
  if (!r.ok) throw new Error("Kernel offline");
  return r.json();
}

export async function hasAdmin(): Promise<boolean> {
  const r = await fetch(`${BASE}/auth/has_admin`);
  const j = await r.json();
  return j.exists === true;
}

export async function createAdmin(username: string, password: string) {
  const r = await fetch(`${BASE}/auth/create_admin`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password }),
  });
  return r.json();
}

export async function login(username: string, password: string) {
  const r = await fetch(`${BASE}/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password }),
  });
  return r.json();
}
