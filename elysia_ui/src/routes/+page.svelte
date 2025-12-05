<!-- =========================================================================
📍 FILE: src/routes/+page.svelte
📝 BESCHRIJVING:
  Rootpagina. Controleert automatisch of het toestel gepaird is.
  - Indien niet gepaird → onboarding flow
  - Indien gepaird → dashboard
============================================================================ -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { checkPairStatus } from '$lib/pairing';

  onMount(async () => {
    const isPaired = await checkPairStatus();

    if (!isPaired) {
      goto('/onboarding');
    } else {
      goto('/dashboard');
    }
  });
</script>

<p>Loading Elysia…</p>
