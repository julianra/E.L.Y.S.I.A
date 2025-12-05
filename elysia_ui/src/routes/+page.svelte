<!-- =========================================================================
📍 FILE: src/routes/+page.svelte
📝 BESCHRIJVING:
  Rootpagina van de UI.
  - Checkt pairing-status bij opstart
  - Routeert automatisch naar onboarding of dashboard
============================================================================ -->

<script>
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { checkPairStatus } from '$lib/pairing';

    onMount(async () => {
        const paired = await checkPairStatus();

        if (paired) {
            goto('/dashboard');
        } else {
            goto('/onboarding');
        }
    });
</script>

<p>Loading Elysia…</p>
