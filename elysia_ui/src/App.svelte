<script>
  import { onMount } from 'svelte';

  // Simpele state-variabelen om de status bij te houden
  let healthText = '';
  let loading = true;
  let error = '';

  // ⚠️ Tijdelijk hardcoded naar portal/kernel health endpoint
  // In dev draait je UI op poort 5173 en portal/kernel bv. op 7070.
  const HEALTH_URL = 'http://localhost:7070/health';

  // Deze functie wordt één keer uitgevoerd wanneer de component wordt "gemount"
  onMount(async () => {
    try {
      const res = await fetch(HEALTH_URL);
      if (!res.ok) {
        throw new Error(`Health endpoint returned status ${res.status}`);
      }
      const text = await res.text();
      healthText = text.trim();
    } catch (e) {
      console.error(e);
      error = 'Kan /health niet bereiken (is portal/kernel wel gestart?)';
    } finally {
      loading = false;
    }
  });

  // Afgeleide status-string voor weergave
  $: statusLabel = loading
    ? 'Bezig met controleren...'
    : error
    ? 'Offline of onbereikbaar'
    : healthText || 'Onbekende status';
</script>

<main class="page">
  <header class="topbar">
    <div class="brand">
      <div class="brand-logo">E</div>
      <div class="brand-text">
        <div class="brand-title">ELYSIA Node</div>
        <div class="brand-subtitle">Local Supervisor UI</div>
      </div>
    </div>
  </header>

  <section class="content">
    <div class="card">
      <h1>Node status</h1>
      <p class="muted">
        Dit is de eerste versie van je ELYSIA node-interface.
        Hier zie je of de kernel/portal bereikbaar is.
      </p>

      <div class="status-row">
        <span class="label">Health endpoint</span>
        <code class="value">{HEALTH_URL}</code>
      </div>

      <div class="status-row">
        <span class="label">Status</span>

        {#if loading}
          <span class="pill pill-neutral">{statusLabel}</span>
        {:else if error}
          <span class="pill pill-bad">{statusLabel}</span>
        {:else}
          <span class="pill pill-good">{statusLabel}</span>
        {/if}
      </div>

      {#if error}
        <p class="error-text">{error}</p>
      {/if}

      <div class="hint">
        <h2>Wat gebeurt hier?</h2>
        <ul>
          <li>De UI draait in Svelte via Vite op <strong>http://localhost:5173</strong>.</li>
          <li>We doen een <code>fetch()</code> naar <strong>{HEALTH_URL}</strong>.</li>
          <li>Als portal/kernel een health string terugstuurt (bv. <code>PORTAL OK</code>), tonen we die hier.</li>
        </ul>
      </div>
    </div>
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    background: radial-gradient(circle at top, #020617, #020617);
    color: #e5e7eb;
  }

  .page {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .topbar {
    padding: 0.75rem 1.5rem;
    border-bottom: 1px solid rgba(148, 163, 184, 0.25);
    background: linear-gradient(to right, #020617, #020617);
    display: flex;
    align-items: center;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.8rem;
  }

  .brand-logo {
    width: 32px;
    height: 32px;
    border-radius: 999px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: radial-gradient(circle at 30% 0%, #22d3ee, #0f172a);
    font-weight: 700;
    font-size: 1.1rem;
  }

  .brand-title {
    font-weight: 600;
    letter-spacing: 0.04em;
  }

  .brand-subtitle {
    font-size: 0.75rem;
    color: #9ca3af;
  }

  .content {
    flex: 1;
    display: flex;
    justify-content: center;
    padding: 2rem 1rem;
  }

  .card {
    width: 100%;
    max-width: 720px;
    background: #020617;
    border-radius: 1rem;
    padding: 1.5rem 1.75rem;
    border: 1px solid rgba(148, 163, 184, 0.35);
    box-shadow: 0 25px 60px rgba(15, 23, 42, 0.75);
  }

  h1 {
    margin: 0;
    font-size: 1.35rem;
  }

  .muted {
    margin-top: 0.35rem;
    margin-bottom: 1.5rem;
    font-size: 0.9rem;
    color: #9ca3af;
  }

  .status-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
    margin-bottom: 0.85rem;
  }

  .label {
    min-width: 120px;
    font-size: 0.85rem;
    color: #9ca3af;
  }

  .value {
    font-size: 0.85rem;
    padding: 0.15rem 0.4rem;
    border-radius: 0.35rem;
    background: #020617;
    border: 1px solid rgba(51, 65, 85, 0.8);
  }

  .pill {
    font-size: 0.85rem;
    padding: 0.25rem 0.75rem;
    border-radius: 999px;
    border: 1px solid transparent;
  }

  .pill-neutral {
    background: rgba(30, 64, 175, 0.25);
    border-color: rgba(59, 130, 246, 0.7);
  }

  .pill-good {
    background: rgba(22, 163, 74, 0.2);
    border-color: rgba(34, 197, 94, 0.8);
  }

  .pill-bad {
    background: rgba(127, 29, 29, 0.4);
    border-color: rgba(248, 113, 113, 0.85);
  }

  .error-text {
    font-size: 0.8rem;
    color: #fca5a5;
    margin-top: 0.3rem;
  }

  .hint {
    margin-top: 1.5rem;
    padding-top: 1rem;
    border-top: 1px dashed rgba(148, 163, 184, 0.5);
    font-size: 0.85rem;
  }

  .hint ul {
    margin: 0.3rem 0 0;
    padding-left: 1.1rem;
  }
</style>
