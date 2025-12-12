// =========================================================
// 📍 FILE: src/stores/router.ts
// 📝 Simple UI router state
// =========================================================

import { writable } from "svelte/store";

export type Page =
  | "start"
  | "admin-setup"
  | "login"
  | "dashboard";

export const currentPage = writable<Page>("start");
