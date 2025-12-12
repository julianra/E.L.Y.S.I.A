<!-- ======================================================================
📍 FILE: src/App.svelte
📝 ROLE:
  Root applicatiecontroller voor ELYSIA Kernel UI (Electron / Vite).

  Verantwoordelijkheden:
    - Detecteert of kernel online is
    - Detecteert of admin account bestaat
    - Stuurt UI-flow:
        1) Kernel offline → statusmelding
        2) Geen admin → AdminCreate
        3) Admin bestaat maar niet ingelogd → LoginForm
        4) Ingelogd → Dashboard

  ⚠️ Geen routing
  ⚠️ Geen SvelteKit
  ✔️ State-based flow
====================================================================== -->

<script lang="ts">
  import { onMount } from "svelte";

  // ------------------------------------------------------------------
  // API
  // ------------------------------------------------------------------
  import { kernelStatus, hasAdmin } from "./api/kernel";

  // ------------------------------------------------------------------
  // STORES
  // ------------------------------------------------------------------
  import {
    kernelOnline,
    adminExists,
    isAuthenticated
  } from "./stores/kernel";

  // ------------------------------------------------------------------
  // COMPONENTS
  // ------------------------------------------------------------------
  import AdminCreate from "./components/AdminCreate.svelte";
  import LoginForm from "./components/LoginForm.svelte";
  import Dashboard from "./components/Dashboard.svelte";

  let loading = true;
  let statusText = "Connecting to ELYSIA Kernel…";

  // ------------------------------------------------------------------
  // BOOTSTRAP
  // ------------------------------------------------------------------
  onMount(async () => {
    try {
      await kernelStatus();
      kernelOnline.set(true);

      const exists = await hasAdmin();
      adminExists.set(exists);

      statusText = "Kernel online.";
    } catch {
      kernelOnline.set(false);
      statusText = "Kernel offline. Waiting…";
    } finally {
      loading = false;
    }
  });
</script>

<!-- ======================================================================
  RENDER FLOW
====================================================================== -->

{#if loading}
  <main class="boot">
    <h1>ELYSIA Core</h1>
    <p>{statusText}</p>
  </main>

{:else if !$kernelOnline}
  <main class="boot">
    <h1>ELYSIA Core</h1>
    <p>Kernel offline. Waiting…</p>
  </main>

{:else}
  {#if !$adminExists}
    <!-- FIRST RUN -->
    <AdminCreate />

  {:else if !$isAuthenticated}
    <!-- LOGIN -->
    <LoginForm />

  {:else}
    <!-- DASHBOARD -->
    <Dashboard />
  {/if}
{/if}

<style>
  /* ====================================================================
     Boot / status screen
     ==================================================================== */
  .boot {
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: radial-gradient(circle at center, #0d1119, #05060a);
    color: white;
    font-family: system-ui, sans-serif;
    text-align: center;
  }

  .boot h1 {
    margin-bottom: 8px;
    font-size: 2rem;
    text-shadow: 0 0 18px rgba(120, 140, 255, 0.6);
  }

  .boot p {
    opacity: 0.8;
    font-size: 0.95rem;
  }
</style>
