<!-- =========================================================================
📍 FILE: src/routes/onboarding/+page.svelte
📝 BESCHRIJVING:
  Eerste setup flow voor een nieuw toestel.
  Haalt pairing info op uit kernel en maakt de koppeling.
============================================================================ -->
<script>
  import { onMount } from 'svelte';
  import { beginPair, completePair } from '$lib/pairing';

  let info = null;

  onMount(async () => {
    info = await beginPair();
  });

  async function pair() {
    await completePair({
      device_name: navigator.userAgent,
      timestamp: Date.now()
    });

    location.href = '/dashboard';
  }
</script>

{#if info}
  <h1>Connect with ELYSIA</h1>
  <p>Node ID: {info.node_id}</p>
  <p>Version: {info.version}</p>
  <button on:click={pair}>Pair Device</button>
{:else}
  <p>Loading pairing info...</p>
{/if}
