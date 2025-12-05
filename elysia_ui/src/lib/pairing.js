// ============================================================================
// 📍 FILE: src/lib/pairing.js
// 📝 Pairing helper tussen ORBIT UI en de ELYSIA Kernel.
//    UI-implementatie van /pair/status, /pair/init, /pair/complete.
//    Extra helper: checkPairStatus() voor routing.
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
// → Gebruikt in root +page.svelte
// --------------------------------------------------------------
export async function checkPairStatus() {
    try {
        const res = await getPairStatus();
        return res.paired === true;
    } catch (e) {
        return false;
    }
}
