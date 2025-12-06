<!-- ======================================================================
📍 FILE: src/routes/login/+page.svelte
📝 BESCHRIJVING:
  Admin login pagina voor de ELYSIA Core Console.

  Functionaliteit:
    - Als er al een admin-token is → direct redirect naar /dashboard
    - Anders → loginformulier tonen
    - POST naar /auth/login via $lib/auth
    - Bij success: token bewaren in localStorage en naar /dashboard
====================================================================== -->

<script>
	import { onMount } from 'svelte';
	import { login } from '$lib/auth';
	import orb from '$lib/assets/orb_enkel.JPG';

	let username = '';
	let password = '';
	let error = '';
	let loading = false;

	onMount(() => {
		try {
			if (typeof localStorage !== 'undefined') {
				const token = localStorage.getItem('elysia_admin_token');
				if (token) {
					window.location.href = '/dashboard';
				}
			}
		} catch {
			// als localStorage faalt → gewoon login tonen
		}
	});

	async function submit() {
		error = '';
		loading = true;

		try {
			const res = await login(username, password);

			if (!res || res.success !== true || !res.token) {
				error = res?.error ?? 'Invalid login credentials';
				loading = false;
				return;
			}

			if (typeof localStorage !== 'undefined') {
				localStorage.setItem('elysia_admin_token', res.token);
			}

			window.location.href = '/dashboard';
		} catch (e) {
			console.error(e);
			error = 'Could not reach ELYSIA Kernel.';
			loading = false;
		}
	}
</script>

<style>
	main.page {
		height: 100vh;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background: radial-gradient(circle at center, #0d1119, #05060a);
		color: white;
		font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
	}

	.orb {
		width: 150px;
		height: 150px;
		border-radius: 50%;
		background-image: url('../../lib/assets/logo.png');
		background-size: cover;
		background-position: center;
		margin-bottom: 20px;
		box-shadow: 0 0 35px rgba(120, 140, 255, 0.7);
		animation: orbPulse 4s infinite ease-in-out;
	}

	.card {
		width: 340px;
		padding: 26px;
		border-radius: 14px;
		background: rgba(255, 255, 255, 0.06);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.14);
		text-align: center;
		box-shadow: 0 0 25px rgba(0, 0, 0, 0.5);
	}

	h1 {
		font-size: 1.6rem;
		margin-bottom: 8px;
		text-shadow: 0 0 12px rgba(140, 160, 255, 0.7);
	}

	p.subtitle {
		opacity: 0.8;
		font-size: 0.9rem;
		margin-bottom: 18px;
	}

	input {
		width: 90%;
		padding: 11px;
		margin: 6px 0;
		border-radius: 8px;
		border: none;
		background: rgba(255, 255, 255, 0.1);
		color: white;
		outline: none;
		font-size: 0.95rem;
	}

	input:focus {
		box-shadow: 0 0 10px rgba(120, 160, 255, 0.9);
	}

	button {
		width: 95%;
		padding: 12px;
		margin-top: 12px;
		border-radius: 10px;
		border: none;
		cursor: pointer;
		background: linear-gradient(90deg, #465cff, #6a82ff);
		color: white;
		font-size: 1rem;
		font-weight: 500;
		box-shadow: 0 0 16px rgba(80, 120, 255, 0.6);
		transition: transform 0.15s ease, box-shadow 0.15s ease;
	}

	button:hover {
		transform: translateY(-1px);
		box-shadow: 0 0 24px rgba(120, 160, 255, 0.9);
	}

	button:disabled {
		opacity: 0.6;
		cursor: default;
		transform: none;
		box-shadow: none;
	}

	.error {
		color: #ff6b6b;
		margin-top: 10px;
		font-size: 0.9rem;
	}

	@keyframes orbPulse {
		0% {
			box-shadow: 0 0 18px rgba(90, 120, 255, 0.4);
		}
		50% {
			box-shadow: 0 0 40px rgba(140, 170, 255, 0.9);
		}
		100% {
			box-shadow: 0 0 18px rgba(90, 120, 255, 0.4);
		}
	}
</style>

<main class="page">
	<div class="orb" aria-hidden="true"></div>

	<div class="card">
		<h1>Admin Login</h1>
		<p class="subtitle">Sign in to manage your ELYSIA Core node.</p>

		<input
			placeholder="Username"
			bind:value={username}
			on:keydown={(e) => e.key === 'Enter' && submit()}
		/>
		<input
			type="password"
			placeholder="Password"
			bind:value={password}
			on:keydown={(e) => e.key === 'Enter' && submit()}
		/>

		<button on:click={submit} disabled={loading}>
			{#if loading}
				Connecting…
			{:else}
				Login
			{/if}
		</button>

		{#if error}
			<p class="error">{error}</p>
		{/if}
	</div>
</main>
