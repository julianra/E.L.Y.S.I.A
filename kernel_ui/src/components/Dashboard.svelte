<!-- ======================================================================
📍 FILE: src/components/Dashboard.svelte
📝 ROLE:
  ELYSIA Core Admin Dashboard.
  Port van vroegere SvelteKit dashboard (+page.svelte).

  Functionaliteit:
    - Leest kernelstatus via GET /status
    - Toont pairing state (placeholder / future)
    - Visuele module load (fake)
    - Logout
====================================================================== -->

<script lang="ts">
  import { onMount } from "svelte";
  import { kernelStatus as fetchKernelStatus } from "../api/kernel";
  import { authToken, isAuthenticated } from "../stores/kernel";

  let kernel = null;
  let statusError = "";

  // Fake module load (exact zoals vroeger)
  let moduleStats = [
    { name: "MARTHE", load: 78 },
    { name: "JUNK", load: 55 },
    { name: "CATNIP", load: 32 },
    { name: "NAVI", load: 61 },
    { name: "HAVEN", load: 22 },
    { name: "SHIELD", load: 44 }
  ];

  onMount(async () => {
    try {
      kernel = await fetchKernelStatus();
    } catch {
      statusError = "Could not reach ELYSIA Kernel.";
    }
  });

  function logout() {
    authToken.set(null);
    isAuthenticated.set(false);
  }
</script>

<div class="page">
  <header>
    <div class="title-block">
      <h1>ELYSIA Core Dashboard</h1>
      <div class="subtitle">
        Monitor your node, control pairing & modules.
      </div>
    </div>

    <div class="buttons">
      <button on:click={() => location.reload()}>Refresh</button>
      <button class="logout" on:click={logout}>Logout</button>
    </div>
  </header>

  <div class="grid">
    <!-- SYSTEM STATUS -->
    <div class="card">
      <h2>System Status</h2>

      {#if kernel}
        <div class="status-item"><span>Status</span><strong>{kernel.status}</strong></div>
        <div class="status-item"><span>Version</span><strong>{kernel.version}</strong></div>
        <div class="status-item"><span>Modules</span><strong>{kernel.modules}</strong></div>
        <div class="status-item"><span>Database</span><strong>{kernel.db}</strong></div>
      {:else if statusError}
        <p class="error-text">{statusError}</p>
      {:else}
        <p>Loading kernel status…</p>
      {/if}
    </div>

    <!-- PAIRING (PLACEHOLDER) -->
    <div class="card">
      <h2>Pairing Control</h2>
      <p style="opacity:0.8;">
        Pairing UI will appear here once enabled in kernel.
      </p>
    </div>

    <!-- MODULE LOAD -->
    <div class="card">
      <h2>Module Load</h2>

      {#each moduleStats as m}
        <div style="margin-bottom: 14px;">
          <div class="status-item">
            <span>{m.name}</span>
            <strong>{m.load}%</strong>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" style={`width:${m.load}%`}></div>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <!-- Floating ORB (visual only for now) -->
  <div class="orb-float" aria-hidden="true"></div>
</div>

<style>
  .page {
    height: 100vh;
    padding: 20px 40px;
    background: radial-gradient(circle at top, #111622, #05060a 70%);
    color: white;
    font-family: system-ui, sans-serif;
    box-sizing: border-box;
  }

  header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 20px;
  }

  h1 {
    font-size: 2.1rem;
    margin: 0;
    text-shadow: 0 0 15px rgba(140,160,255,0.5);
  }

  .subtitle {
    opacity: 0.8;
    font-size: 0.95rem;
  }

  .buttons {
    display: flex;
    gap: 10px;
  }

  .buttons button {
    padding: 8px 14px;
    border-radius: 999px;
    background: rgba(255,255,255,0.06);
    color: white;
  }

  .buttons .logout {
    background: linear-gradient(90deg, #ff5b7b, #ff8a6d);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 24px;
  }

  .card {
    background: rgba(255,255,255,0.05);
    border-radius: 14px;
    padding: 22px;
    backdrop-filter: blur(14px);
    border: 1px solid rgba(255,255,255,0.12);
  }

  .status-item {
    display: flex;
    justify-content: space-between;
    margin-bottom: 8px;
    font-size: 0.95rem;
  }

  .progress-bar {
    height: 10px;
    background: rgba(255,255,255,0.08);
    border-radius: 6px;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #5a6bff, #8694ff);
    border-radius: 6px;
  }

  .orb-float {
    position: fixed;
    bottom: 25px;
    right: 25px;
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background-image: url("../assets/orb_enkel.JPG");
    background-size: cover;
    box-shadow: 0 0 25px rgba(110,140,255,0.6);
    animation: orbFloat 4s infinite ease-in-out;
  }

  @keyframes orbFloat {
    0% { transform: translateY(0); }
    50% { transform: translateY(-8px); }
    100% { transform: translateY(0); }
  }

  .error-text {
    color: #ff6b6b;
  }
</style>
