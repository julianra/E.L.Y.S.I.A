// ======================================================================
// 📍 FILE: src/main.ts
// 📝 ROLE:
//   Vite entrypoint voor ELYSIA Kernel UI.
//   Laadt globale theme + root component.
// ======================================================================

import "./styles/theme.css";
import App from "./App.svelte";

new App({
  target: document.getElementById("app")!,
});
