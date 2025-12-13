<!-- =========================================================
📍 FILE: src/pages/Dashboard.svelte
📝 ROLE:
  ELYSIA Kernel Dashboard.
  Functionele control-interface (status, pairing, modules).
========================================================= -->

<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { isAuthenticated } from "../stores/kernel";
  import { currentPage } from "../stores/router";

  let status: any = null;

  onMount(async () => {
    status = await api("/status");
  });

  function logout() {
    localStorage.removeItem("elysia_admin_token");
    isAuthenticated.set(false);
    currentPage.set("login");
  }
</script>

<div class="dashboard">

  <!-- HEADER -->
  <header class="header">
    <div>
      <h1>ELYSIA Core Dashboard</h1>
      <p>Monitor your node, control pairing & modules.</p>
    </div>

    <div class="actions">
      <button class="ghost">Refresh</button>
      <button class="danger" on:click={logout}>Logout</button>
    </div>
  </header>

  <!-- GRID -->
  <main class="grid">

    <!-- SYSTEM STATUS -->
    <section class="panel">
      <h2>System Status</h2>

      {#if status}
        <div class="row"><span>Status</span><strong>{status.status}</strong></div>
        <div class="row"><span>Version</span><strong>{status.version}</strong></div>
        <div class="row"><span>Modules</span><strong>{status.modules}</strong></div>
        <div class="row"><span>Database</span><strong>{status.db}</strong></div>
        <div class="row"><span>Port</span><strong>2022</strong></div>
      {/if}
    </section>

    <!-- PAIRING -->
    <section class="panel center">
      <h2>Pairing Control</h2>

      <p class="label">Pairing is <strong>ACTIVE</strong></p>

      <div class="pair-code">467730</div>

      <p class="hint">
        Enter this code on your device within the next few minutes.
      </p>

      <div class="pair-actions">
        <button class="primary">Enable 5 min</button>
        <button class="ghost">Disable</button>
      </div>
    </section>

    <!-- MODULE LOAD -->
    <section class="panel">
      <h2>Module Load</h2>

      <div class="module">
        <span>MARTHE</span><div class="bar"><div style="width:78%"></div></div><span>78%</span>
      </div>
      <div class="module">
        <span>JUNK</span><div class="bar"><div style="width:55%"></div></div><span>55%</span>
      </div>
      <div class="module">
        <span>CATNIP</span><div class="bar"><div style="width:32%"></div></div><span>32%</span>
      </div>
      <div class="module">
        <span>NAVI</span><div class="bar"><div style="width:61%"></div></div><span>61%</span>
      </div>
      <div class="module">
        <span>SHIELD</span><div class="bar"><div style="width:44%"></div></div><span>44%</span>
      </div>
    </section>

  </main>

</div>

<style>
  .dashboard {
    padding: 32px 48px;
    color: #e6e8ee;
  }

  /* HEADER */
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 32px;
  }

  .header h1 {
    margin: 0;
    font-size: 1.8rem;
  }

  .header p {
    margin: 4px 0 0;
    opacity: 0.6;
    font-size: 0.95rem;
  }

  .actions {
    display: flex;
    gap: 12px;
  }

  /* GRID */
  .grid {
    display: grid;
    grid-template-columns: 1.2fr 1.2fr 1.4fr;
    gap: 24px;
  }

  /* PANELS */
  .panel {
    background: #12151c;
    border-radius: 16px;
    padding: 24px;
    border: 1px solid #1e2230;
  }

  .panel.center {
    text-align: center;
  }

  .panel h2 {
    margin-top: 0;
    margin-bottom: 20px;
    font-size: 1.2rem;
  }

  /* ROWS */
  .row {
    display: flex;
    justify-content: space-between;
    margin-bottom: 10px;
    font-size: 0.95rem;
  }

  .row span {
    opacity: 0.6;
  }

  /* PAIRING */
  .pair-code {
    font-size: 2.4rem;
    letter-spacing: 0.4em;
    margin: 20px 0;
    font-weight: bold;
  }

  .hint {
    opacity: 0.6;
    font-size: 0.9rem;
    margin-bottom: 20px;
  }

  .pair-actions {
    display: flex;
    justify-content: center;
    gap: 12px;
  }

  /* MODULES */
  .module {
    display: grid;
    grid-template-columns: 80px 1fr 40px;
    align-items: center;
    gap: 10px;
    margin-bottom: 12px;
    font-size: 0.9rem;
  }

  .bar {
    height: 8px;
    background: #1f2433;
    border-radius: 999px;
    overflow: hidden;
  }

  .bar div {
    height: 100%;
    background: linear-gradient(90deg, #6f7cff, #8fa2ff);
  }

  /* BUTTONS */
  button {
    border-radius: 999px;
    padding: 8px 16px;
    border: none;
    cursor: pointer;
  }

  .primary {
    background: #5b6cff;
    color: white;
  }

  .ghost {
    background: transparent;
    color: #cdd1ff;
    border: 1px solid #2a2f42;
  }

  .danger {
    background: #ff5b7b;
    color: white;
  }
</style>
