// ======================================================================
// 📍 FILE: src/main.ts
// 📝 ROLE:
//   Vite entrypoint voor ELYSIA Kernel UI.
//   Laadt globale theme + root component.
// ======================================================================

import "./styles/theme.css";
import App from "./App.svelte";

console.log("MAIN.TS LOADED");

const target = document.getElementById("app");

if (!target) {
  throw new Error("❌ #app not found");
}

console.log("MOUNTING APP");

new App({
  target,
});
