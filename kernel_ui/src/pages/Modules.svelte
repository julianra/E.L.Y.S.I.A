<!-- =========================================================
📍 FILE: src/pages/Modules.svelte
📝 ROLE:
  Modulebeheer pagina (UI-only, fase 1)
  - Leest modules van de Kernel via /modules
  - Laat admin manueel pairen / unpairen
  - Laat admin ZIP-modules uploaden
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

    // UI-only placeholders (fase 1)
    description: string;
    enabled: boolean;
    permissions: Permission[];
    lastLog: string;
  };

  let modules: Module[] = [];
  let selected: Module | null = null;
  let error: string | null = null;
  let pollTimer: number | null = null;

  let uploading = false;
  let uploadError: string | null = null;

  const ALL_PERMISSIONS: Permission[] = [
    "filesystem",
    "network",
    "events",
    "ui",
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
        lastLog: "Geen activiteit",
      }));

      if (selected) {
        selected = modules.find((m) => m.id === selected?.id) ?? null;
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

  async function uploadModule(file: File) {
  uploading = true;
  uploadError = null;

  try {
    const res = await fetch("http://127.0.0.1:2022/modules/upload", {
      method: "POST",
      headers: {
        "Content-Type": "application/octet-stream",
        "X-Filename": file.name,
        ...(localStorage.getItem("elysia_admin_token")
          ? { Authorization: `Bearer ${localStorage.getItem("elysia_admin_token")}` }
          : {}),
      },
      body: file, // 🔥 raw stream
    });

    if (!res.ok) {
      throw new Error(`Upload failed (${res.status})`);
    }

    await loadModules();
  } catch (e: any) {
    uploadError = e?.message ?? "Upload mislukt";
  } finally {
    uploading = false;
  }
}

  function onFileSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    uploadModule(input.files[0]);
    input.value = "";
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

  <!-- =========================
       MODULE UPLOAD
  ========================== -->
  <div class="upload-box">
    <label class="upload-btn">
      Upload module (.zip)
      <input
        type="file"
        accept=".zip"
        on:change={onFileSelected}
        disabled={uploading}
        hidden
      />
    </label>

    {#if uploading}
      <p class="muted">Upload bezig…</p>
    {/if}

    {#if uploadError}
      <p class="error">{uploadError}</p>
    {/if}
  </div>

  <div class="layout">
    <aside class="module-list">
      {#if modules.length === 0}
        <p class="muted">Geen modules gevonden.</p>
      {:else}
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
      {/if}
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
                <input type="checkbox" disabled />
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
  .modules-page {
    padding: 48px;
    color: white;
  }

  header {
    margin-bottom: 32px;
  }

  .layout {
    display: grid;
    grid-template-columns: 320px 1fr;
    gap: 28px;
  }

  .upload-box {
    margin-bottom: 24px;
  }

  .upload-btn {
    display: inline-block;
    padding: 10px 16px;
    border-radius: 12px;
    background: rgba(90,110,255,.9);
    cursor: pointer;
  }

  .upload-btn:hover {
    background: rgba(120,140,255,1);
  }

  .muted {
    opacity: .6;
    margin-top: 6px;
  }

  .error {
    color: #ff8a8a;
    margin-top: 6px;
  }

  .module-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .module-card {
    background: rgba(20,25,40,.85);
    border-radius: 14px;
    padding: 14px;
    text-align: left;
    cursor: pointer;
  }

  .module-card.active {
    box-shadow: 0 0 24px rgba(120,150,255,.18);
  }

  .top {
    display: flex;
    justify-content: space-between;
    margin-bottom: 6px;
  }

  .status.on {
    color: #7cffb2;
  }

  .status.off {
    color: #ff9a9a;
  }

  .details {
    background: rgba(15,18,30,.9);
    border-radius: 20px;
    padding: 28px;
  }

  .desc {
    opacity: .75;
    margin-bottom: 24px;
  }

  .section {
    margin-bottom: 26px;
  }

  .primary {
    background: #5b7cff;
    border: none;
    padding: 10px 16px;
    border-radius: 12px;
    color: white;
  }

  .danger {
    background: #ff5b5b;
    border: none;
    padding: 10px 16px;
    border-radius: 12px;
    color: white;
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
  }
</style>
