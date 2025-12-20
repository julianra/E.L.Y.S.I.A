<!-- =========================================================
📍 FILE: src/pages/Login.svelte
📝 ROLE:
  Admin login page.
========================================================= -->

<script lang="ts">
  import { api } from "../lib/api";

  import { authToken, isAuthenticated } from "../stores/kernel";
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

    const res = await api("/auth/login", {
      method: "POST",
      body: JSON.stringify({ username, password })
    });

    if (!res?.success) {
      error = res?.error ?? "Login gefaald.";
    } else {
      localStorage.setItem("elysia_admin_token", res.token);

authToken.set(res.token);
isAuthenticated.set(true);
currentPage.set("dashboard");

    }

    loading = false;
  }
</script>

<Viewport>
  <Orb size={150} />

  <div class="card">
    <h1>Admin Login</h1>

    <input value={username} disabled />
    <input
      type="wachtwoord"
      placeholder="wachtwoord"
      bind:value={password}
      on:keydown={(e) => e.key === "Enter" && submit()}
    />

    <button on:click={submit} disabled={loading}>
      {loading ? "verbinden..." : "Login"}
    </button>

    {#if error}
      <p class="error">{error}</p>
    {/if}
  </div>
</Viewport>

<style>
  .card {
    width: 340px;
    padding: 26px;
    border-radius: 16px;
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    backdrop-filter: blur(14px);
    text-align: center;
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
