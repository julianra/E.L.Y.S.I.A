<!-- kernel_ui/src-tauri/src/routes/login/+page.svelte -->
<script>
  let username = "";
  let password = "";
  let loading = false;
  let error = "";
  import logo from '../../lib/assets/logo.png';

  async function submit() {
    loading = true;
    error = "";
    setTimeout(() => loading = false, 600);
  }
</script>

<style>
  main {
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: radial-gradient(circle at center, #0d1119, #05060a);
    color: white;
  }

  .orb {
    width: 150px;
    height: 150px;
    border-radius: 50%;
    background-image: url({logo});
    background-size: cover;
    margin-bottom: 20px;
    box-shadow: 0 0 35px rgba(120,140,255,0.7);
    animation: orbPulse 4s infinite ease-in-out;
  }

  .card {
    width: 340px;
    padding: 26px;
    border-radius: 14px;
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.14);
    backdrop-filter: blur(12px);
    text-align: center;
  }

  input {
    width: 90%;
    padding: 11px;
    margin: 6px 0;
    background: rgba(255,255,255,0.1);
    border-radius: 8px;
    color: white;
    border: none;
  }

  button {
    width: 95%;
    padding: 12px;
    margin-top: 12px;
    border-radius: 10px;
    background: linear-gradient(90deg, #465cff, #6a82ff);
    color: white;
    border: none;
  }
</style>

<main>
<div class="orb" style="background-image: url('{logo}');"></div>

  <div class="card">
    <h1>Admin Login</h1>

    <input placeholder="Username" bind:value={username} />
    <input type="password" placeholder="Password" bind:value={password} />

    <button on:click={submit} disabled={loading}>
      {loading ? "Connecting…" : "Login"}
    </button>

    {#if error}
      <p style="color:#ff6b6b;">{error}</p>
    {/if}
  </div>
</main>
