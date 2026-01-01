// ======================================================================
// 📍 FILE: src/lib/api.ts
// 📝 ROLE:
//   Centrale API-helper voor communicatie met de ELYSIA Kernel.
//   - Electron-only
//   - Geen proxy
//   - Directe fetch naar kernel
//   - Respecteert raw uploads (ZIP streaming)
// ======================================================================

const KERNEL_BASE = "http://127.0.0.1:2022";

export async function api(
  path: string,
  options: RequestInit = {}
): Promise<any> {
  const token =
    typeof localStorage !== "undefined"
      ? localStorage.getItem("elysia_admin_token")
      : null;

  const headers: HeadersInit = {
    ...(options.headers || {}),
  };

  // ❗ Alleen JSON header zetten als body leeg is
  if (!options.body && !headers["Content-Type"]) {
    headers["Content-Type"] = "application/json";
  }

  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  try {
    const res = await fetch(`${KERNEL_BASE}${path}`, {
      ...options,
      headers,
    });

    const text = await res.text();

    try {
      return JSON.parse(text);
    } catch {
      return text;
    }
  } catch {
    return {
      success: false,
      error: "Kernel unreachable",
    };
  }
}
