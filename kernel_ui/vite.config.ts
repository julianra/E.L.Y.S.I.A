// ======================================================================
// 📍 FILE: vite.config.ts
// 📝 ROLE:
//   Vite configuratie voor ELYSIA Kernel UI
//   - Alleen bundler + dev server voor Electron
//   - GEEN webmodus
//   - GEEN proxy
//   - GEEN API routing
// ======================================================================

import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig({
  base: "./",
  plugins: [svelte()],
  build: {
    outDir: "dist",
    emptyOutDir: true,
  },
});
