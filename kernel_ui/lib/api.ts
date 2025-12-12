// ======================================================================
// 100% WERKENDE API-LAAG VOOR DEV + TAURI EXE
// ======================================================================

// Detecteer Tauri EXE
const isTauri =
  typeof window !== "undefined" &&
  typeof (window as any).__TAURI_INTERNALS__ !== "undefined";

// In EXE altijd rechtstreeks naar de kernel
// In DEV via Vite proxy → "/api"
export const API_BASE = isTauri
  ? "http://127.0.0.1:2022"
  : "/api";

interface FetchOptions extends RequestInit {
  headers?: Record<string, string>;
}

// RAW FETCH
export async function api(path: string, options: FetchOptions = {}) {
  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      ...(options.headers ?? {})
    }
  });

  return res.json();
}

// ADMIN FETCH MET TOKEN
export async function kernelFetch(path: string, options: FetchOptions = {}) {
  const token = localStorage.getItem("elysia_admin_token") || "";

  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${token}`,
      ...(options.headers ?? {})
    }
  });

  return res.json();
}
