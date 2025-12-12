<!-- kernel_ui/src-tauri/src/routes/admin/setup/+page.svelte -->
<script>
  import logo from "../../../lib/assets/logo.png";
  import { api } from '$lib/api';

  let username = "";
  let password = "";
  let loading = false;
  let success = false;
  let error = "";

  async function submit() {
    loading = true;
    error = "";
    success = false;

    const data = await api('/auth/create_admin', {
      method: "POST",
      body: JSON.stringify({ username, password })
    });

    if (!data.success) {
      error = data.error;
    } else {
      success = true;
      setTimeout(() => window.location.href = "/login", 600);
    }

    loading = false;
  }
</script>

<!-- UI unchanged -->

<style>
  main {
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: radial-gradient(circle at center, #0d1119, #05060a);
    color: white;
    font-family: system-ui, sans-serif;
    text-align: center;
    padding: 20px;
  }

  .orb {
    width: 200px;
    height: 200px;
    border-radius: 50%;
    background-image: url({logo});
    background-size: cover;
    background-position: center;
    margin-bottom: 24px;
    box-shadow: 0 0 40px rgba(120,140,255,0.6);
    animation: orbPulse 4s infinite ease-in-out;
  }

  @keyframes orbPulse {
    0% { box-shadow: 0 0 20px rgba(90,120,255,0.4); }
    50% { box-shadow: 0 0 50px rgba(140,170,255,0.9); }
    100% { box-shadow: 0 0 20px rgba(90,120,255,0.4); }
  }

  .card {
    width: 360px;
    max-width: 95%;
    padding: 26px;
    border-radius: 14px;
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.12);
    backdrop-filter: blur(12px);
  }

  input {
    width: 90%;
    padding: 12px;
    margin-top: 12px;
    border-radius: 8px;
    border: none;
    background: rgba(255,255,255,0.1);
    color: white;
    font-size: 1rem;
  }

  input:focus {
    outline: none;
    box-shadow: 0 0 12px rgba(120,140,255,0.8);
  }

  button {
    width: 95%;
    padding: 12px;
    margin-top: 20px;
    background: linear-gradient(90deg, #465CFF, #6A82FF);
    border: none;
    border-radius: 10px;
    color: white;
    cursor: pointer;
    font-size: 1rem;
    transition: 0.2s ease;
  }

  button:hover {
    transform: translateY(-2px);
    box-shadow: 0 0 20px rgba(120,140,255,0.6);
  }

  button:disabled {
    opacity: 0.6;
    transform: none;
    box-shadow: none;
    cursor: default;
  }

  .msg {
    margin-top: 12px;
    font-size: 0.95rem;
  }
  .error { color: #ff6b6b; }
  .success { color: #6bff9c; }
</style>

<main>
<div class="orb" style="background-image: url('{logo}');"></div>

  <div class="card">
    <h1>Create Administrator</h1>
    <p style="opacity:0.8;font-size:0.95rem;margin-bottom:10px;">
      This is the first-time setup for your ELYSIA Kernel node.
    </p>

    <input
      placeholder="Admin username"
      bind:value={username}
      on:keydown={(e) => e.key === "Enter" && submit()}
    />

    <input
      type="password"
      placeholder="Admin password"
      bind:value={password}
      on:keydown={(e) => e.key === "Enter" && submit()}
    />

    <button on:click={submit} disabled={loading}>
      {loading ? "Creating admin…" : "Create Admin"}
    </button>

    {#if error}
      <div class="msg error">{error}</div>
    {/if}

    {#if success}
      <div class="msg success">
        Admin created! (placeholder response)
      </div>
    {/if}
  </div>
</main>
