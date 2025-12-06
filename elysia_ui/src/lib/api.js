// ============================================================================
// 📍 FILE: src/lib/api.js
// 📝 BESCHRIJVING:
//   Fetch-wrapper voor ELYSIA Core.
//   ALLE requests gaan via de Vite proxy:
//
//       UI → /api/... → Vite → http://localhost:2022
//
//   Hierdoor:
//     - GEEN CORS problemen
//     - Browser ziet alles als “zelfde domein”
// ============================================================================

const CORE_URL = '/api';

/**
 * kernelFetch
 *
 * @param {string} path  - bv. "/status", "/auth/login", "/pair/status"
 * @param {RequestInit} options - extra fetch opties
 */
export async function kernelFetch(path, options = {}) {
	const headers = {
		'Content-Type': 'application/json',
		...(options.headers || {})
	};

	// Public endpoints: geen Authorization header
	const isPublic =
		path.startsWith('/auth') ||
		path.startsWith('/pair/') ||
		path.startsWith('/health') ||
		path.startsWith('/status');

	// Token automatisch toevoegen
	if (!isPublic) {
		try {
			const token = localStorage?.getItem('elysia_admin_token');
			if (token) {
				headers['Authorization'] = `Bearer ${token}`;
			}
		} catch {
			// SSR fallback
		}
	}

	const res = await fetch(`${CORE_URL}${path}`, {
		...options,
		headers
	});

	const text = await res.text();
	if (!text) return {};

	try {
		return JSON.parse(text);
	} catch {
		return text;
	}
}
