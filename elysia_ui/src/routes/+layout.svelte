<!-- ======================================================================
📍 FILE: src/routes/+layout.svelte
📝 Modern SvelteKit layout syntax (2025)
====================================================================== -->

<script>
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<slot />
