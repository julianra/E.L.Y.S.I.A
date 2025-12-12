<!-- ======================================================================
📍 FILE: src/components/AdminCreate.svelte
📝 ROLE:
  UI flow voor eerste installatie.
  Laat toe een admin account aan te maken
  ALS er nog geen admin bestaat.
====================================================================== -->

<script lang="ts">
  import { createAdmin } from "../api/kernel";
  import { adminExists } from "../stores/kernel";

  let password = "";
  let error: string | null = null;

  async function submit() {
    error = null;
    const res = await createAdmin("admin", password);

    if (!res.success) {
      error = res.error;
    } else {
      adminExists.set(true);
    }
  }
</script>

<h2>Create Admin</h2>

<label for="pw">Password</label>
<input id="pw" type="password" bind:value={password} />

<button on:click={submit}>Create Admin Account</button>

{#if error}
  <p class="error">{error}</p>
{/if}
