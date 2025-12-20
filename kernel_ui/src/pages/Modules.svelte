<!-- =========================================================
📍 FILE: src/pages/Modules.svelte
📝 ROLE:
  Modulebeheer pagina (UI-only, fase 1)
  - Leest modules van de Kernel via /modules
  - Laat admin manueel pairen / unpairen
========================================================= -->

<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";

  type Permission = "filesystem" | "network" | "events" | "ui";

  type Module = {
    id: string;
    name: string;
    installed: boolean;
    loaded: boolean;
    paired: boolean;

    // UI-only placeholders
    description: string;
    enabled: boolean;
    permissions: Permission[];
    lastLog: string;
  };

  let modules: Module[] = [];
  let selected: Module | null = null;
  let error: string | null = null;
  let pollTimer: number | null = null;

  const ALL_PERMISSIONS: Permission[] = [
    "filesystem",
    "network",
    "events",
    "ui"
  ];

  async function loadModules() {
    try {
      const data: Array<{
        id: string;
        name: string;
        installed: boolean;
        loaded: boolean;
        paired: boolean;
      }> = await api("/modules");

      modules = data.map((m) => ({
        id: m.id,
        name: m.name,
        installed: m.installed,
        loaded: m.loaded,
        paired: m.paired,

        description: "Geen beschrijving beschikbaar",
        enabled: m.loaded,
        permissions: [],
        lastLog: "Geen activiteit"
      }));

      if (selected) {
        selected = modules.find(m => m.id === selected?.id) ?? null;
      } else {
        selected = modules[0] ?? null;
      }

      error = null;
    } catch (e: any) {
      error = e?.message ?? "Kernel niet bereikbaar";
      modules = [];
      selected = null;
    }
  }

  async function pairSelected() {
    if (!selected) return;
    await api(`/modules/${selected.id}/pair`, { method: "POST" });
    await loadModules();
  }

  async function unpairSelected() {
    if (!selected) return;
    await api(`/modules/${selected.id}/unpair`, { method: "POST" });
    await loadModules();
  }

  function togglePermission(p: Permission) {
    if (!selected) return;

    selected.permissions = selected.permissions.includes(p)
      ? selected.permissions.filter((x) => x !== p)
      : [...selected.permissions, p];

    modules = [...modules];
  }

  onMount(() => {
    loadModules();

    pollTimer = window.setInterval(() => {
      loadModules();
    }, 2000);

    return () => {
      if (pollTimer) {
        clearInterval(pollTimer);
        pollTimer = null;
      }
    };
  });
</script>

<section class="modules-page">
  <header>
    <div>
      <h1>Modules</h1>
      <p>Beheer plugins, permissies en recente activiteit.</p>
      {#if error}
        <p class="error">Kernel niet bereikbaar: {error}</p>
      {/if}
    </div>
  </header>

  <div class="layout">
    <aside class="module-list">
      {#each modules as m}
        <button
          class="module-card {selected?.id === m.id ? 'active' : ''}"
          on:click={() => (selected = m)}
        >
          <div class="top">
            <strong>{m.name}</strong>
            <span class="status {m.paired ? 'on' : 'off'}">
              {m.paired ? "Gepaird" : "Niet gepaird"}
            </span>
          </div>
          <small>{m.lastLog}</small>
        </button>
      {/each}
    </aside>

    <main class="details">
      {#if selected}
        <h2>{selected.name}</h2>
        <p class="desc">{selected.description}</p>

        <div class="section">
          <h3>Status</h3>

          {#if selected.paired}
            <button class="danger" on:click={unpairSelected}>
              Unpair module
            </button>
          {:else}
            <button class="primary" on:click={pairSelected}>
              Pair module
            </button>
          {/if}
        </div>

        <div class="section">
          <h3>Permissies</h3>
          <div class="permissions">
            {#each ALL_PERMISSIONS as p}
              <label>
                <input
                  type="checkbox"
                  disabled
                  checked={selected.permissions.includes(p)}
                />
                <span>{p}</span>
              </label>
            {/each}
          </div>
        </div>
      {:else}
        <p>Selecteer een module.</p>
      {/if}
    </main>
  </div>
</section>

<style>
  .modules-page { padding: 48px; color: white; }
  header { margin-bottom: 32px; }
  .layout { display: grid; grid-template-columns: 320px 1fr; gap: 28px; }
  .module-card { background: rgba(20,25,40,.85); border-radius: 14px; padding: 14px; }
  .module-card.active { box-shadow: 0 0 24px rgba(120,150,255,.18); }
  .status.on { color: #7cffb2; }
  .status.off { color: #ff9a9a; }
  .details { background: rgba(15,18,30,.9); border-radius: 20px; padding: 28px; }
  .primary { background: #5b7cff; border: none; padding: 10px 16px; border-radius: 12px; }
  .danger { background: #ff5b5b; border: none; padding: 10px 16px; border-radius: 12px; }
</style>
