// ============================================================================
// 📍 FILE: src/lib/auth.js
// 📝 BESCHRIJVING:
//   Auth API wrapper voor de ELYSIA Kernel.
//   Wordt gebruikt in onboarding & login UI.
// ============================================================================

import { kernelFetch } from './api';

// -------------------------------------------------------
// ADMIN CHECK
// -------------------------------------------------------
export async function getInitialState() {
	return kernelFetch('/auth/initial_state');
}

// -------------------------------------------------------
// ADMIN AANMAKEN
// -------------------------------------------------------
export async function createAdmin(username, password) {
	return kernelFetch('/auth/create_admin', {
		method: 'POST',
		body: JSON.stringify({ username, password })
	});
}

// -------------------------------------------------------
// LOGIN
// -------------------------------------------------------
export async function login(username, password) {
	return kernelFetch('/auth/login', {
		method: 'POST',
		body: JSON.stringify({ username, password })
	});
}
