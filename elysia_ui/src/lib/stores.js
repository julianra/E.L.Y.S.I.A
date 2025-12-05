// ============================================================================
// 📍 FILE: src/lib/stores.js
// 📝 BESCHRIJVING:
//   Globale Svelte stores voor UI state.
//   - pairing status
//   - kernel info
//   - device info
// ============================================================================
import { writable } from 'svelte/store';

export const paired = writable(false);
export const deviceInfo = writable(null);
export const kernelInfo = writable(null);
