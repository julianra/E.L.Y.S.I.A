// ============================================================================
// 📍 FILE: src/lib/api.js
// ============================================================================

export async function kernelFetch(path, options = {}) {
    const url = `/kernel${path}`;

    const res = await fetch(url, {
        headers: {
            'Content-Type': 'application/json',
            ...(options.headers || {})
        }, 
        ...options
    });

    return res.json();
}
