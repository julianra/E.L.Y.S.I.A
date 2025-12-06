<!-- ======================================================================
📍 FILE: src/routes/+layout.svelte
📝 BESCHRIJVING:
  Globale layout voor ELYSIA UI.
  - Laadt globale CSS (app.css)
  - Zet favicon
  - Render child routes via <slot/>
====================================================================== -->

<script>
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>ELYSIA – Core Console</title>
</svelte:head>

<slot />
