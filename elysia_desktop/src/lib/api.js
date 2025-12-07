// ============================================================================
// 📍 FILE: src/lib/api.js
// ELYSIA Core Fetch Wrapper (Desktop SPA Version)
// ============================================================================

const CORE_URL = '/api';

/**
 * kernelFetch
 * 
 * UI → /api/... → Vite → http://localhost:2022/...
 */
export async function kernelFetch(path, options = {}) {
	const headers = {
		'Content-Type': 'application/json',
		...(options.headers || {})
	};

	// Public routes
	const isPublic =
		path.startsWith('/auth') ||
		path.startsWith('/pair/') ||
		path.startsWith('/health') ||
		path === '/status';

	// Append admin token
	if (!isPublic) {
		const token = localStorage?.getItem('elysia_admin_token');
		if (token) headers['Authorization'] = `Bearer ${token}`;
	}

	let res;
	try {
		res = await fetch(`${CORE_URL}${path}`, {
			...options,
			headers
		});
	} catch (err) {
		return { error: 'Kernel unreachable', detail: err };
	}

	const text = await res.text();
	if (!text) return {};

	// Try to parse JSON first
	try {
		return JSON.parse(text);
	} catch {
		// If HTML → this means the proxy is NOT routing
		if (text.startsWith('<!doctype html')) {
			return {
				error: 'Vite proxy returned HTML instead of JSON',
				hint: 'Check vite.config.js proxy settings.'
			};
		}

		return text;
	}
}
