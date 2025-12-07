// ======================================================================
// 📍 FILE: src/router.js
// ======================================================================

import Login from './pages/login/Login.svelte';
import Dashboard from './pages/Dashboard.svelte';
import Pairing from './pages/pairing/Pairing.svelte';
import AdminSetup from './pages/AdminSetup.svelte';

export const routes = {
	'/': Login,
	'/login': Login,
	'/dashboard': Dashboard,
	'/pairing': Pairing,
	'/admin/setup': AdminSetup
};
