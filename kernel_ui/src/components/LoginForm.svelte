<!-- ======================================================================
📍 FILE: src/components/LoginForm.svelte
📝 ROLE:
  Admin login UI.
  Zet expliciet authenticated state bij succes.
====================================================================== -->

<script lang="ts">
  import { login } from "../api/kernel";
  import { authToken, isAuthenticated } from "../stores/kernel";

  let password = "";
  let error: string | null = null;

  async function submit() {
    error = null;

    const res = await login("admin", password);

    if (!res.success) {
      error = res.error;
      return;
    }

    authToken.set(res.token);
    isAuthenticated.set(true);
  }
</script>

<h2>Admin Login</h2>

<label for="pw">Password</label>
<input id="pw" type="password" bind:value={password} />

<button on:click={submit}>Login</button>

{#if error}
  <p class="error">{error}</p>
{/if}
