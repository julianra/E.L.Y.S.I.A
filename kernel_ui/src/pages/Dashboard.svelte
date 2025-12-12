<!-- =========================================================
📍 FILE: src/pages/Dashboard.svelte
📝 ROLE:
  ELYSIA Kernel Dashboard (post-login).
  Centrale beheerpagina voor kernelstatus.
========================================================= -->

<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { isAuthenticated } from "../stores/kernel";
  import { currentPage } from "../stores/router";

  let status: any = null;
  let error = "";

  onMount(async () => {
    try {
      status = await api("/status");
    } catch (e) {
      error = "Failed to reach kernel.";
    }
  });

  function logout() {
    localStorage.removeItem("elysia_admin_token");
    isAuthenticated.set(false);
    currentPage.set("login");
  }
</script>

<div class="page">
  <!-- HEADER -->
  <header class="topbar">
    <div class="title">
      <h1>ELYSIA Kernel</h1>
      <span class="subtitle">lokaal systeem dashboard</span>
    </div>

    <button class="logout" on:click={logout}>
      Logout
    </button>
  </header>

  <!-- CONTENT -->
  <main class="content">
    <div class="grid">

      <!-- KERNEL STATUS -->
      <section class="card primary">
        <h2>Systeem Status</h2>

        {#if error}
          <p class="error">{error}</p>
        {:else if status}
          <div class="row"><span>Status</span><strong>{status.status}</strong></div>
          <div class="row"><span>Version</span><strong>{status.version}</strong></div>
          <div class="row"><span>Modules</span><strong>{status.modules}</strong></div>
          <div class="row"><span>Database</span><strong>{status.db}</strong></div>
        {:else}
          <p>kernel status laden…</p>
        {/if}
      </section>

      <!-- PLACEHOLDER: MODULES -->
      <section class="card">
        <h2>Modules</h2>
        <p class="muted">ni te veel willen he. die modules komen nog wel. heb geduld.</p>
      </section>

      <!-- PLACEHOLDER: SYSTEM -->
      <section class="card">
        <h2>System</h2>
        <p class="muted">Pairing, logs en updates komen later.</p>
      </section>

    </div>
  </main>
</div>

<style>
  /* ===== PAGE ===== */
  .page {
    min-height: 100vh;
    background: var(--bg-main);
    color: white;
    padding: 32px 40px;
    box-sizing: border-box;
  }

  /* ===== HEADER ===== */
  .topbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 36px;
  }

  .title h1 {
    margin: 0;
    font-size: 2.2rem;
    text-shadow: 0 0 16px rgba(140,160,255,0.6);
  }

  .subtitle {
    opacity: 0.75;
    font-size: 0.95rem;
  }

  .logout {
    padding: 10px 18px;
    border-radius: 999px;
    background: linear-gradient(90deg, #ff5b7b, #ff8a6d);
    color: white;
    border: none;
    cursor: pointer;
  }

  /* ===== CONTENT ===== */
  .content {
    max-width: 1200px;
    margin: 0 auto;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 26px;
  }

  /* ===== CARDS ===== */
  .card {
    padding: 26px;
    border-radius: 18px;
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    backdrop-filter: blur(14px);
  }

  .card.primary {
    box-shadow: 0 0 30px rgba(120,140,255,0.35);
  }

  .card h2 {
    margin-top: 0;
    margin-bottom: 16px;
    font-size: 1.3rem;
    text-shadow: 0 0 10px rgba(120,140,255,0.45);
  }

  .row {
    display: flex;
    justify-content: space-between;
    margin-bottom: 10px;
    font-size: 0.95rem;
  }

  .row span {
    opacity: 0.75;
  }

  .row strong {
    font-weight: 600;
  }

  .muted {
    opacity: 0.7;
    font-size: 0.95rem;
  }

  .error {
    color: var(--danger);
  }
</style>
