// ============================================================================
// 📍 FILE: src/lib/pairing.js
// 📝 BESCHRIJVING:
//   Pairing helper tussen ORBIT UI en de ELYSIA Kernel.
//   UI-implementatie van:
//     - GET  /pair/status
//     - GET  /pair/init
//     - POST /pair/complete
//
//   Extra helper: checkPairStatus() voor routing/logica.
// ============================================================================

import { kernelFetch } from './api';

export function getPairStatus() {
	return kernelFetch('/pair/status');
}

export function startPairing() {
	return kernelFetch('/pair/init');
}

export function completePairing(body) {
	return kernelFetch('/pair/complete', {
		method: 'POST',
		body: JSON.stringify(body)
	});
}

// --------------------------------------------------------------
// CHECK PAIR STATUS
// → Gebruikt door pairing UI of future router
// --------------------------------------------------------------
export async function checkPairStatus() {
	try {
		const res = await getPairStatus();
		return res.pairing_active === true;
	} catch (e) {
		return false;
	}
}
