<!-- ======================================================================
📍 FILE: src/pages/Dashboard.svelte
📝 ELYSIA Core Admin Dashboard (Desktop SPA Version)
====================================================================== -->

<script>
	import { onMount } from 'svelte';
	import { kernelFetch } from '../lib/api.js';
	import orb from '../assets/orb_enkel.JPG';

	let kernelStatus = null;
	let statusError = '';

	let pairingLoading = true;
	let pairingActive = false;
	let pairingCode = null;
	let pairingMessage = '';
	let pairingBusy = false;

	// Fake module load data (UI only)
	let moduleStats = [
		{ name: 'MARTHE', load: 78 },
		{ name: 'JUNK', load: 55 },
		{ name: 'CATNIP', load: 32 },
		{ name: 'NAVI', load: 61 },
		{ name: 'HAVEN', load: 22 },
		{ name: 'SHIELD', load: 44 }
	];

	onMount(async () => {
		// Kernel info
		try {
			const status = await kernelFetch('/status');
			if (typeof status === 'string') {
				statusError = status;
				kernelStatus = null;
			} else {
				kernelStatus = status;
			}
		} catch (e) {
			statusError = 'Could not reach ELYSIA Kernel.';
		}

		await refreshPairingStatus();
	});

	async function refreshPairingStatus() {
		pairingLoading = true;
		pairingMessage = '';

		try {
			const res = await kernelFetch('/pairing/status');
			pairingActive = !!res.active;
			pairingCode = res.code ?? null;
		} catch (e) {
			pairingMessage = 'Could not read pairing status.';
		}

		pairingLoading = false;
	}

	async function enablePairing() {
		pairingBusy = true;

		try {
			const res = await kernelFetch('/pairing/enable', {
				method: 'POST',
				body: JSON.stringify({ minutes: 5 })
			});

			if (!res?.success) {
				pairingMessage = res?.error ?? 'Failed to enable pairing.';
			} else {
				pairingActive = true;
				pairingCode = res.code ?? null;
				pairingMessage = `Pairing enabled for ${res.minutes ?? 5} minutes.`;
			}
		} catch {
			pairingMessage = 'Failed to enable pairing.';
		}

		pairingBusy = false;
	}

	async function disablePairing() {
		pairingBusy = true;

		try {
			const res = await kernelFetch('/pairing/disable', {
				method: 'POST',
				body: JSON.stringify({})
			});

			if (!res?.success) {
				pairingMessage = res?.error ?? 'Failed to disable pairing.';
			} else {
				pairingActive = false;
				pairingCode = null;
				pairingMessage = 'Pairing disabled.';
			}
		} catch {
			pairingMessage = 'Failed to disable pairing.';
		}

		pairingBusy = false;
	}

	function logout() {
		try {
			localStorage.removeItem('elysia_admin_token');
		} catch {}
		window.location.hash = '/login';
	}
</script>

<style>
	.page {
		height: 100vh;
		width: 100%;
		display: flex;
		flex-direction: column;
		background: radial-gradient(circle at top, #111622, #05060a 70%);
		color: white;
		padding: 20px 40px;
		box-sizing: border-box;
		overflow: hidden;
		font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
	}

	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 20px;
	}

	header h1 {
		font-size: 2.1rem;
		margin: 0;
		font-weight: 600;
		text-shadow: 0 0 15px rgba(140, 160, 255, 0.5);
	}

	header .subtitle {
		opacity: 0.8;
		font-size: 0.95rem;
	}

	header .buttons {
		display: flex;
		gap: 10px;
	}

	header button {
		padding: 8px 14px;
		border-radius: 999px;
		border: none;
		background: rgba(255,255,255,0.06);
		color: white;
		cursor: pointer;
		transition: 0.15s ease;
	}

	header button:hover {
		background: rgba(255,255,255,0.12);
	}

	header button.logout {
		background: linear-gradient(90deg, #ff5b7b, #ff8a6d);
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 24px;
	}

	.card {
		background: rgba(255,255,255,0.05);
		border-radius: 14px;
		padding: 22px;
		backdrop-filter: blur(14px);
		border: 1px solid rgba(255,255,255,0.12);
		box-shadow: 0 0 25px rgba(0,0,0,0.45);
	}

	.card h2 {
		font-size: 1.3rem;
		margin-bottom: 12px;
		text-shadow: 0 0 10px rgba(120,140,255,0.45);
	}

	.status-item {
		display: flex;
		justify-content: space-between;
		margin-bottom: 8px;
	}

	.status-item span {
		opacity: 0.85;
	}

	.status-item strong {
		font-weight: 600;
	}

	.error-text {
		color: #ff6b6b;
		margin-top: 8px;
		font-size: 0.9rem;
	}

	.pairing-code {
		font-size: 2rem;
		font-weight: 700;
		text-align: center;
		letter-spacing: 0.25em;
		text-shadow: 0 0 20px rgba(140,170,255,0.85);
		margin: 8px 0;
	}

	.orb-float {
		position: fixed;
		bottom: 25px;
		right: 25px;
		width: 80px;
		height: 80px;
		background-image: url('../assets/orb_enkel.JPG');
		background-size: cover;
		background-position: center;
		border-radius: 50%;
		box-shadow: 0 0 25px rgba(110,140,255,0.6);
		animation: orbFloat 4s infinite ease-in-out;
		cursor: pointer;
		z-index: 999;
	}

	@keyframes orbFloat {
		0% { transform: translateY(0px); }
		50% { transform: translateY(-8px); }
		100% { transform: translateY(0px); }
	}

	.progress-bar {
		width: 100%;
		height: 10px;
		border-radius: 6px;
		background: rgba(255,255,255,0.08);
	}

	.progress-fill {
		height: 100%;
		border-radius: 6px;
		background: linear-gradient(90deg, #5a6bff, #8694ff);
	}
</style>

<div class="page">
	<header>
		<div>
			<h1>ELYSIA Core Dashboard</h1>
			<div class="subtitle">Monitor your node, manage pairing & modules.</div>
		</div>

		<div class="buttons">
			<button on:click={refreshPairingStatus}>Refresh</button>
			<button class="logout" on:click={logout}>Logout</button>
		</div>
	</header>

	<div class="grid">

		<!-- SYSTEM STATUS -->
		<div class="card">
			<h2>System Status</h2>

			{#if kernelStatus}
				<div class="status-item"><span>Status</span> <strong>{kernelStatus.status}</strong></div>
				<div class="status-item"><span>Version</span> <strong>{kernelStatus.version}</strong></div>
				<div class="status-item"><span>Modules</span> <strong>{kernelStatus.modules}</strong></div>
				<div class="status-item"><span>DB</span> <strong>{kernelStatus.db}</strong></div>
				<div class="status-item"><span>Port</span> <strong>{kernelStatus.port}</strong></div>

			{:else if statusError}
				<p class="error-text">{statusError}</p>

			{:else}
				<p>Loading kernel status…</p>
			{/if}
		</div>

		<!-- PAIRING CONTROL -->
		<div class="card">
			<h2>Pairing Control</h2>

			{#if pairingLoading}
				<p>Checking pairing state…</p>

			{:else}
				<p>
					{#if pairingActive}
						Pairing is <strong>ACTIVE</strong>.
					{:else}
						Pairing is <strong>DISABLED</strong>.
					{/if}
				</p>

				{#if pairingActive && pairingCode}
					<div class="pairing-code">{pairingCode}</div>
					<p>Use this code in the ORBIT / CARE app.</p>

				{:else if pairingActive}
					<p>Waiting for device…</p>

				{:else}
					<p>Enable pairing to connect new devices.</p>
				{/if}

				<div style="display:flex; gap:10px; margin-top:10px;">
					<button on:click={enablePairing} disabled={pairingBusy}>Enable 5 min</button>
					<button on:click={disablePairing} disabled={!pairingActive || pairingBusy}>Disable</button>
				</div>

				{#if pairingMessage}
					<p style="margin-top:6px; opacity:0.85;">{pairingMessage}</p>
				{/if}
			{/if}
		</div>

		<!-- MODULE LOAD -->
		<div class="card">
			<h2>Module Load</h2>

			{#each moduleStats as m}
				<div style="margin-bottom: 14px;">
					<div class="status-item">
						<span>{m.name}</span>
						<strong>{m.load}%</strong>
					</div>

					<div class="progress-bar">
						<div class="progress-fill" style="width:{m.load}%"></div>
					</div>
				</div>
			{/each}
		</div>

	</div>

	<!-- FLOATING ORB -->
	<div class="orb-float"></div>
</div>
