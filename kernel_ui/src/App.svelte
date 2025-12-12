<!-- ======================================================================
📍 FILE: src/App.svelte
📝 ROLE:
  Root orchestrator voor ELYSIA Kernel UI.
  Stuurt:
    - Kernel status
    - Admin flow
    - Authenticated flow
====================================================================== -->

<script lang="ts">
  import { onMount } from "svelte";

  // API
  import { kernelStatus, hasAdmin } from "./api/kernel";

  // STORES  ✅ VERPLICHT
  import {
    kernelOnline,
    adminExists,
    isAuthenticated,
    lastCheck
  } from "./stores/kernel";

  // COMPONENTS
  import KernelCard from "./components/KernelCard.svelte";
  import AdminCreate from "./components/AdminCreate.svelte";
  import LoginForm from "./components/LoginForm.svelte";
  import Dashboard from "./components/Dashboard.svelte";
  import Loading from "./components/Loading.svelte";

  let loading = true;

  onMount(async () => {
    try {
      await kernelStatus();
      kernelOnline.set(true);
      adminExists.set(await hasAdmin());
      lastCheck.set(new Date());
    } catch {
      kernelOnline.set(false);
    } finally {
      loading = false;
    }
  });
</script>

{#if loading}
  <Loading />
{:else}
  <KernelCard />

  {#if $kernelOnline}
    {#if $isAuthenticated}
      <Dashboard />
    {:else}
      {#if $adminExists}
        <LoginForm />
      {:else}
        <AdminCreate />
      {/if}
    {/if}
  {/if}
{/if}
