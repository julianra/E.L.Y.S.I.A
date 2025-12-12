// ======================================================================
// 📍 FILE: src/stores/kernel.ts
// 📝 ROLE:
//   Centrale UI-auth en kernel state
// ======================================================================

import { writable } from "svelte/store";

export const kernelOnline = writable(false);
export const adminExists = writable(false);

export const authToken = writable<string | null>(null);
export const isAuthenticated = writable(false);

export const lastCheck = writable<Date | null>(null);
