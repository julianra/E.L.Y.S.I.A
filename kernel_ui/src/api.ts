// ======================================================================
// 📍 FILE: src/api.ts
// 📝 Kernel API helpers (no framework assumptions)
// ======================================================================

const BASE_URL = "http://127.0.0.1:2022";

export async function hasAdmin(): Promise<boolean> {
  const res = await fetch(`${BASE_URL}/auth/has_admin`);
  const json = await res.json();
  return json.exists === true;
}

export async function createAdmin(username: string, password: string) {
  const res = await fetch(`${BASE_URL}/auth/create_admin`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password })
  });
  return res.json();
}

export async function login(username: string, password: string) {
  const res = await fetch(`${BASE_URL}/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password })
  });
  return res.json();
}
