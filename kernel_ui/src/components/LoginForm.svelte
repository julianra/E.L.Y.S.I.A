<!-- ======================================================================
📍 FILE: src/components/LoginForm.svelte
📝 ROLE:
  Admin login UI.
====================================================================== -->

<script lang="ts">
  import { login } from "../api/kernel";
  import { authToken, isAuthenticated } from "../stores/kernel";

  import CenterLayout from "./CenterLayout.svelte";
  import Orb from "./Orb.svelte";

  let username = "admin";
  let password = "";
  let loading = false;
  let error = "";
</script>

<CenterLayout>
  <Orb size={150} />

  <div class="card">
    <h1>Admin Login</h1>

    <input disabled bind:value={username} />
    <input type="password" bind:value={password} />

    <button
      on:click={async () => {
        loading = true;
        error = "";
        const res = await login(username, password);
        if (!res.success) error = res.error;
        else {
          authToken.set(res.token);
          isAuthenticated.set(true);
        }
        loading = false;
      }}
      disabled={loading}
    >
      {loading ? "verbinden..." : "Login"}
    </button>

    {#if error}<p class="error">{error}</p>{/if}
  </div>
</CenterLayout>

<style>
  .card {
    width: 340px;
    padding: 26px;
    border-radius: 14px;
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.14);
    backdrop-filter: blur(12px);
  }

  input {
    width: 90%;
    padding: 11px;
    margin: 6px 0;
    background: rgba(255,255,255,0.1);
    border-radius: 8px;
    border: none;
    color: white;
  }

  .error {
    color: #ff6b6b;
    margin-top: 8px;
  }
</style>
