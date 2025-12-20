<!-- =========================================================
📍 FILE: src/pages/AdminSetup.svelte
📝 ROLE:
  First-time administrator setup page.
  Creates admin and routes to login.
========================================================= -->

<script lang="ts">
import { api } from "../lib/api";

  import { adminExists } from "../stores/kernel";
  import { currentPage } from "../stores/router";

  import Viewport from "../components/Viewport.svelte";
  import Orb from "../components/Orb.svelte";

  let username = "admin";
  let password = "";
  let loading = false;
  let error = "";

  async function submit() {
    loading = true;
    error = "";

    const res = await api("/auth/create_admin", {
      method: "POST",
      body: JSON.stringify({ username, password })
    });

    if (!res?.success) {
      error = res?.error ?? "skill issue.";
    } else {
      adminExists.set(true);
      currentPage.set("login");
    }

    loading = false;
  }
</script>

<Viewport>
  <Orb size={180} />

  <div class="card">
    <h1>maak je administrator account</h1>
    <p class="subtitle">
      Welkom bij Elysia, maak hier je admin account aan en start je avontuur.
    </p>

    <input value={username} disabled />

    <input
      type="wachtwoord"
      placeholder="Admin wachtwoord"
      bind:value={password}
      on:keydown={(e) => e.key === "Enter" && submit()}
    />

    <button on:click={submit} disabled={loading}>
      {loading ? "maak een admin account…" : "Create Admin"}
    </button>

    {#if error}
      <p class="error">{error}</p>
    {/if}
  </div>
</Viewport>

<style>
  .card {
    width: 360px;
    max-width: 95%;
    padding: 26px;
    border-radius: 16px;
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    backdrop-filter: blur(14px);
    text-align: center;
  }

  .subtitle {
    opacity: 0.8;
    font-size: 0.95rem;
    margin-bottom: 12px;
  }

  input {
    width: 100%;
    padding: 12px;
    margin-top: 12px;
    border-radius: 8px;
    border: none;
    background: rgba(255,255,255,0.1);
    color: white;
  }

  button {
    width: 100%;
    padding: 12px;
    margin-top: 18px;
    border-radius: 10px;
    background: linear-gradient(
      90deg,
      var(--accent-strong),
      var(--accent)
    );
    color: white;
    border: none;
  }

  .error {
    margin-top: 10px;
    color: var(--danger);
  }
</style>
