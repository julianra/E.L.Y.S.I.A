// ============================================================================
// 📍 FILE: src/lib/api.js
// 📝 BESCHRIJVING:
//   Basisklasse om te communiceren met de ELYSIA Kernel (HTTP).
//   Alle UI-requests naar kernel gaan via deze wrapper.
// ============================================================================

export const KERNEL_PORT = 2022;

export async function kernelFetch(path, options = {}) {
    const url = `http://localhost:${KERNEL_PORT}${path}`;

    const res = await fetch(url, {
        headers: {
            'Content-Type': 'application/json',
            ...(options.headers || {})
        },
        ...options
    });

    return res.json();
}
