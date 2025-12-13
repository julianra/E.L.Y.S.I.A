// ======================================================================
// 📍 FILE: src/lib/api.ts
// 📝 ROLE:
//   Centrale API-helper voor communicatie met de ELYSIA Kernel.
//   - Automatische JSON parsing
//   - Token injectie
//   - Error-safe responses
// ======================================================================

const BASE_URL =
  import.meta.env.DEV ? '/api' : 'http://127.0.0.1:2022';


export async function api(
  path: string,
  options: RequestInit = {}
): Promise<any> {
  try {
    const token =
      typeof localStorage !== 'undefined'
        ? localStorage.getItem('elysia_admin_token')
        : null;

    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...(options.headers || {}),
    };

    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    const res = await fetch(`${BASE_URL}${path}`, {
      ...options,
      headers,
    });

    const text = await res.text();

    try {
      return JSON.parse(text);
    } catch {
      return {
        success: false,
        error: 'Invalid JSON response from kernel',
      };
    }
  } catch (err) {
    return {
      success: false,
      error: 'Kernel unreachable',
    };
  }
}
