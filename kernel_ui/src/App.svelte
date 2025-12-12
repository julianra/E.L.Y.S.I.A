<!-- ======================================================================
📍 FILE: src/App.svelte
📝 ELYSIA Kernel UI – Admin flow
====================================================================== -->

<script lang="ts">
  import { onMount } from "svelte";
  import { hasAdmin, createAdmin, login } from "./api";

  let kernelOnline = false;
  let adminExists = false;
  let loading = true;
  let error: string | null = null;

  let username = "admin";
  let password = "";

  let token: string | null = null;

  onMount(async () => {
    try {
      const status = await fetch("http://127.0.0.1:2022/status");
      if (!status.ok) throw new Error("Kernel offline");

      kernelOnline = true;
      adminExists = await hasAdmin();
    } catch (e: any) {
      error = e.message;
    } finally {
      loading = false;
    }
  });

  async function handleCreateAdmin() {
    error = null;
    const res = await createAdmin(username, password);
    if (!res.success) {
      error = res.error;
    } else {
      adminExists = true;
      password = "";
    }
  }

  async function handleLogin() {
    error = null;
    const res = await login(username, password);
    if (!res.success) {
      error = res.error;
    } else {
      token = res.token;
    }
  }
</script>

<main>
  <div class="card">
    <h1>ELYSIA Kernel</h1>

    {#if loading}
      <p>Checking kernel…</p>

    {:else if !kernelOnline}
      <p class="error">Kernel offline</p>

    {:else}
      <p class="online">● ONLINE</p>

      {#if token}
        <p class="success">Logged in</p>
        <code>{token}</code>

      {:else if adminExists}
        <h2>Login</h2>

        <label for="login-pass">Password</label>
        <input
          id="login-pass"
          type="password"
          bind:value={password}
        />

        <button on:click={handleLogin}>
          Login
        </button>

      {:else}
        <h2>Create Admin</h2>

        <label for="create-user">Username</label>
        <input
          id="create-user"
          bind:value={username}
          disabled
        />

        <label for="create-pass">Password</label>
        <input
          id="create-pass"
          type="password"
          bind:value={password}
        />

        <button on:click={handleCreateAdmin}>
          Create Admin Account
        </button>
      {/if}

      {#if error}
        <p class="error">{error}</p>
      {/if}
    {/if}
  </div>
</main>

<style>
  main {
    height: 100vh;
    display: grid;
    place-items: center;
    background: radial-gradient(circle at center, #1b1f3b, #05050a);
    color: #eee;
    font-family: system-ui;
  }

  .card {
    background: rgba(20, 24, 45, 0.85);
    padding: 32px;
    border-radius: 16px;
    width: 360px;
    box-shadow: 0 0 40px rgba(120, 90, 255, 0.3);
  }

  h1 {
    text-align: center;
    margin-bottom: 12px;
  }

  h2 {
    margin-top: 20px;
  }

  label {
    display: block;
    margin-top: 12px;
    font-size: 0.9rem;
    opacity: 0.8;
  }

  input {
    width: 100%;
    margin-top: 4px;
    padding: 8px;
    border-radius: 8px;
    border: none;
  }

  button {
    margin-top: 16px;
    width: 100%;
    padding: 10px;
    border-radius: 10px;
    border: none;
    background: #7b6cff;
    color: white;
    font-weight: 600;
    cursor: pointer;
  }

  .online {
    color: #4cff8f;
    text-align: center;
  }

  .error {
    color: #ff6b6b;
    margin-top: 12px;
  }

  .success {
    color: #4cff8f;
    margin-top: 12px;
  }

  code {
    display: block;
    margin-top: 10px;
    word-break: break-all;
    font-size: 0.75rem;
    opacity: 0.8;
  }
</style>
