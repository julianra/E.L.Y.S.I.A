// ============================================================================
// 📍 FILE: src/lib/pairing.js
// 📝 BESCHRIJVING:
//   Pairing helper tussen ORBIT UI en de ELYSIA Kernel.
//   Deze file implementeert de UI-zijde van:
//     GET /pair/status
//     GET /pair/init
//     POST /pair/complete
// ============================================================================
import { kernelFetch } from '$lib/api';
import { paired, deviceInfo, kernelInfo } from '$lib/stores';

export async function checkPairStatus() {
    try {
        const status = await kernelFetch('/pair/status');
        paired.set(status.paired);
        deviceInfo.set(status.device);
        kernelInfo.set(status.kernel);
        return status.paired;
    } catch (e) {
        return false;
    }
}

export async function beginPair() {
    const res = await kernelFetch('/pair/init');
    kernelInfo.set(res);
    return res;
}

export async function completePair(payload) {
    return kernelFetch('/pair/complete', {
        method: 'POST',
        body: JSON.stringify(payload)
    });
}
