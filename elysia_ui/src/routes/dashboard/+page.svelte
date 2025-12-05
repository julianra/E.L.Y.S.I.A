<!-- =========================================================================
📍 FILE: src/routes/dashboard/+page.svelte
📝 BESCHRIJVING:
  Homepagina van de ELYSIA OS UI.
  Toont kernel-informatie na succesvolle pairing.
============================================================================ -->

<script>
  import { kernelInfo } from '$lib/stores';
</script>

<h1>Elysia Dashboard</h1>

{#if $kernelInfo}
  <pre>{JSON.stringify($kernelInfo, null, 2)}</pre>
{:else}
  <p>Loading…</p>
{/if}
