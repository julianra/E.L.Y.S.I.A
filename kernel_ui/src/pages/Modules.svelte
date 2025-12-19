<!-- =========================================================
📍 FILE: src/pages/Modules.svelte
📝 ROLE:
  Modulebeheer pagina (UI-only, fase 1)
  - Leest native modules read-only van de Kernel via /modules
  - UI toont extra velden als placeholders (fase 2)
========================================================= -->

<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";


  type Permission = "filesystem" | "network" | "events" | "ui";

  type Module = {
    // Kernel truth (fase 1)
    id: string;
    name: string;
    kind: string;   // "native"
    status: string; // "loaded"

    // UI-only placeholders (fase 1)
    description: string;
    enabled: boolean;
    permissions: Permission[];
    lastLog: string;
  };

  let modules: Module[] = [];
  let selected: Module | null = null;
  let error: string | null = null;
const ALL_PERMISSIONS: Permission[] = [
  "filesystem",
  "network",
  "events",
  "ui"
];

 onMount(async () => {
  try {
    const data: Array<{
      id: string;
      name: string;
      kind: string;
      status: string;
    }> = await api("/modules");

    // Verrijk met UI-only defaults (fase 1 placeholders)
    modules = data.map((m) => ({
      ...m,
      description: "Geen beschrijving beschikbaar",
      enabled: m.status === "loaded",
      permissions: [],
      lastLog: "Geen activiteit"
    }));

    selected = modules[0] ?? null;
    error = null;
  } catch (e: any) {
    error = e?.message ?? "Kernel niet bereikbaar";
    modules = [];
    selected = null;
  }
});


  function togglePermission(p: Permission) {
    if (!selected) return;

    selected.permissions = selected.permissions.includes(p)
      ? selected.permissions.filter((x) => x !== p)
      : [...selected.permissions, p];

    // trigger reactivity
    modules = [...modules];
  }

  function addModule() {
    alert("Module toevoegen (pairing / marketplace – later)");
  }
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

    <button class="primary" on:click={addModule}>
      + Module toevoegen
    </button>
  </header>

  <div class="layout">
    <!-- =========================
      MODULE LIJST
    ========================== -->
    <aside class="module-list">
      {#if modules.length === 0}
        <div class="empty">
          <strong>Geen modules</strong>
          <small>Er zijn momenteel geen native modules geladen.</small>
        </div>
      {:else}
        {#each modules as m}
          <button
            class="module-card {selected?.id === m.id ? 'active' : ''}"
            on:click={() => (selected = m)}
          >
            <div class="top">
              <strong>{m.name}</strong>
              <span class="status {m.status === 'loaded' ? 'on' : 'off'}">
                {m.status === "loaded" ? "Actief" : "Inactief"}
              </span>
            </div>

            <small>{m.lastLog}</small>
          </button>
        {/each}
      {/if}
    </aside>

    <!-- =========================
      MODULE DETAILS
    ========================== -->
    <main class="details">
      {#if selected}
        <h2>{selected.name}</h2>
        <p class="desc">{selected.description}</p>

        <div class="section">
          <h3>Status</h3>
          <button class="toggle" disabled title="Fase 2">
            {selected.status === "loaded" ? "Actief" : "Inactief"}
          </button>
        </div>

        <div class="section">
          <h3>Permissies</h3>

          <div class="permissions">
            {#each ALL_PERMISSIONS as p}
              <label>
                <input
                  type="checkbox"
                  checked={selected.permissions.includes(p)}
                  on:change={() => togglePermission(p)}

                />
                <span>{p}</span>
              </label>
            {/each}
          </div>
        </div>

        <div class="section">
          <h3>Laatste activiteit</h3>
          <div class="log">{selected.lastLog}</div>
        </div>
      {:else}
        <p>Selecteer een module.</p>
      {/if}
    </main>
  </div>
</section>

<style>
  .modules-page {
    padding: 48px;
    color: white;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 32px;
  }

  h1 {
    font-size: 28px;
    margin: 0;
  }

  header p {
    opacity: 0.7;
    margin-top: 4px;
  }

  .error {
    margin-top: 10px;
    opacity: 0.9;
    color: #ff9a9a;
    font-size: 13px;
  }

  .primary {
    background: linear-gradient(135deg, #5b7cff, #7a5cff);
    border: none;
    border-radius: 14px;
    padding: 10px 16px;
    color: white;
    cursor: pointer;
  }

  .layout {
    display: grid;
    grid-template-columns: 320px 1fr;
    gap: 28px;
  }

  .module-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .empty {
    background: rgba(20, 25, 40, 0.55);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 14px;
    padding: 14px;
  }

  .empty small {
    display: block;
    margin-top: 6px;
    opacity: 0.7;
  }

  .module-card {
    background: rgba(20, 25, 40, 0.85);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 14px;
    padding: 14px;
    text-align: left;
    cursor: pointer;
    transition: all 160ms ease;
  }

  .module-card.active {
    border-color: rgba(120, 150, 255, 0.4);
    box-shadow: 0 0 24px rgba(120, 150, 255, 0.18);
  }

  .module-card .top {
    display: flex;
    justify-content: space-between;
    margin-bottom: 6px;
  }

  .status {
    font-size: 11px;
    opacity: 0.8;
  }

  .status.on {
    color: #7cffb2;
  }

  .status.off {
    color: #ff9a9a;
  }

  .details {
    background: rgba(15, 18, 30, 0.9);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 20px;
    padding: 28px;
  }

  .desc {
    opacity: 0.75;
    margin-bottom: 24px;
  }

  .section {
    margin-bottom: 26px;
  }

  .toggle {
    background: none;
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 12px;
    padding: 8px 14px;
    color: white;
    cursor: not-allowed;
    opacity: 0.7;
  }

  .permissions {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
  }

  label {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 14px;
  }

  .log {
    background: rgba(0, 0, 0, 0.35);
    border-radius: 12px;
    padding: 12px;
    font-family: monospace;
    font-size: 13px;
    opacity: 0.85;
  }
</style>
