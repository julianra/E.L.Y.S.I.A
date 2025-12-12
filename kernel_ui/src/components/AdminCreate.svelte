<!-- ======================================================================
📍 FILE: src/components/AdminCreate.svelte
📝 ROLE:
  Eerste admin setup UI.
====================================================================== -->

<script lang="ts">
  import { createAdmin } from "../api/kernel";
  import { adminExists } from "../stores/kernel";

  import CenterLayout from "./CenterLayout.svelte";
  import Orb from "./Orb.svelte";

  let username = "admin";
  let password = "";
  let loading = false;
  let error = "";
  let success = false;

  async function submit() {
    loading = true;
    error = "";
    success = false;

    const res = await createAdmin(username, password);

    if (!res.success) {
      error = res.error;
    } else {
      success = true;
      adminExists.set(true);
    }

    loading = false;
  }
</script>

<CenterLayout>
  <Orb size={200} />

  <div class="card">
    <h1>Create Administrator</h1>
    <p class="subtitle">
      This is the first-time setup for your ELYSIA Kernel node.
    </p>

    <input disabled bind:value={username} />
    <input
      type="password"
      placeholder="Admin password"
      bind:value={password}
      on:keydown={(e) => e.key === "Enter" && submit()}
    />

    <button on:click={submit} disabled={loading}>
      {loading ? "Creating admin…" : "Create Admin"}
    </button>

    {#if error}<div class="msg error">{error}</div>{/if}
    {#if success}<div class="msg success">Admin created.</div>{/if}
  </div>
</CenterLayout>

<style>
  .card {
    width: 360px;
    padding: 26px;
    border-radius: 14px;
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.12);
    backdrop-filter: blur(12px);
  }

  .subtitle { opacity: 0.8; font-size: 0.95rem; }

  input {
    width: 90%;
    padding: 12px;
    margin-top: 12px;
    background: rgba(255,255,255,0.1);
    border-radius: 8px;
    border: none;
    color: white;
  }

  button {
    width: 95%;
    margin-top: 20px;
    padding: 12px;
    border-radius: 10px;
    background: linear-gradient(90deg, #465CFF, #6A82FF);
    color: white;
  }

  .error { color: #ff6b6b; }
  .success { color: #6bff9c; }
</style>
