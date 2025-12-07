<!-- ======================================================================
📍 FILE: src/App.svelte
====================================================================== -->

<script>
	import './app.css';
	import Router from 'svelte-spa-router';
	import { routes } from './router.js';
</script>

<svelte:head>
	<title>ELYSIA – Core Console</title>
</svelte:head>

<Router {routes} />
