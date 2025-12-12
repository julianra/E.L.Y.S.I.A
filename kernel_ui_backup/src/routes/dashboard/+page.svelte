<!-- ======================================================================
📍 FILE: src/routes/dashboard/+page.svelte
📝 BESCHRIJVING:
  ELYSIA Core Admin Dashboard.

  Functionaliteit:
    - Leest kernelstatus via GET /status
    - Leest pairingstatus via GET /pairing/status (met admin-token)
    - Kan pairing aanzetten voor 5 minuten via POST /pairing/enable
    - Kan pairing manueel uitschakelen via POST /pairing/disable

  UI:
    - 3 cards:
        1) System Status
        2) Pairing Control
        3) Fake Module Load (voor "het voelt al af")
    - Floating ORB rechts-onderaan
====================================================================== -->

<script>
	import { onMount } from 'svelte';
	import { kernelFetch } from '$lib/api';
	import orb from '$lib/assets/orb_enkel.JPG';
    import logo from '$lib/assets/logo.png';

	let kernelStatus = null;
	let statusError = '';

	let pairingLoading = true;
	let pairingActive = false;
	let pairingCode = null;
	let pairingMessage = '';

	let pairingBusy = false;

	// Fake module load data
	let moduleStats = [
		{ name: 'MARTHE', load: 78 },
		{ name: 'JUNK', load: 55 },
		{ name: 'CATNIP', load: 32 },
		{ name: 'NAVI', load: 61 },
		{ name: 'HAVEN', load: 22 },
		{ name: 'SHIELD', load: 44 }
	];

	onMount(async () => {
		// Kernel status
		try {
			const status = await kernelFetch('/status');
			if (typeof status === 'string') {
				kernelStatus = null;
				statusError = status;
			} else {
				kernelStatus = status;
			}
		} catch (e) {
			console.error(e);
			statusError = 'Could not reach ELYSIA Kernel.';
		}

		// Pairing status
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
			console.error(e);
			pairingMessage = 'Could not read pairing status.';
		} finally {
			pairingLoading = false;
		}
	}

	async function enablePairing() {
		pairingBusy = true;
		pairingMessage = '';

		try {
			const res = await kernelFetch('/pairing/enable', {
				method: 'POST',
				body: JSON.stringify({ minutes: 5 })
			});

			if (!res || res.success !== true) {
				pairingMessage = res?.error ?? 'Could not enable pairing.';
			} else {
				pairingActive = true;
				pairingCode = res.code ?? null;
				pairingMessage = `Pairing enabled for ${res.minutes ?? 5} minutes.`;
			}
		} catch (e) {
			console.error(e);
			pairingMessage = 'Could not enable pairing.';
		} finally {
			pairingBusy = false;
		}
	}

	async function disablePairing() {
		pairingBusy = true;
		pairingMessage = '';

		try {
			const res = await kernelFetch('/pairing/disable', {
				method: 'POST',
				body: JSON.stringify({})
			});

			if (!res || res.success !== true) {
				pairingMessage = res?.error ?? 'Could not disable pairing.';
			} else {
				pairingActive = false;
				pairingCode = null;
				pairingMessage = 'Pairing disabled.';
			}
		} catch (e) {
			console.error(e);
			pairingMessage = 'Could not disable pairing.';
		} finally {
			pairingBusy = false;
		}
	}

	function logout() {
		try {
			if (typeof localStorage !== 'undefined') {
				localStorage.removeItem('elysia_admin_token');
			}
		} catch {
			// ignore
		}
		window.location.href = '/login';
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
		overflow: hidden;
		padding: 20px 40px;
		font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		box-sizing: border-box;
	}

	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 20px;
	}

	header .title-block {
		display: flex;
		flex-direction: column;
	}

	header h1 {
		font-size: 2.1rem;
		font-weight: 600;
		text-shadow: 0 0 15px rgba(140, 160, 255, 0.5);
		margin: 0;
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
		cursor: pointer;
		background: rgba(255, 255, 255, 0.06);
		color: white;
		font-size: 0.9rem;
		transition: 0.15s ease;
	}

	header button:hover {
		background: rgba(255, 255, 255, 0.12);
	}

	header button.logout {
		background: linear-gradient(90deg, #ff5b7b, #ff8a6d);
	}

	header button.logout:hover {
		filter: brightness(1.1);
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 24px;
	}

	.card {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 14px;
		padding: 22px;
		backdrop-filter: blur(14px);
		border: 1px solid rgba(255, 255, 255, 0.12);
		box-shadow: 0 0 25px rgba(0, 0, 0, 0.45);
	}

	.card h2 {
		font-size: 1.3rem;
		margin-bottom: 12px;
		text-shadow: 0 0 10px rgba(120, 140, 255, 0.45);
	}

	.status-item {
		display: flex;
		justify-content: space-between;
		margin-bottom: 8px;
		font-size: 0.95rem;
	}

	.status-item span {
		opacity: 0.85;
	}

	.status-item strong {
		font-weight: 600;
	}

	.error-text {
		color: #ff6b6b;
		font-size: 0.9rem;
		margin-top: 8px;
	}

	.pairing-code {
		font-size: 2.2rem;
		font-weight: 700;
		letter-spacing: 0.25em;
		text-align: center;
		margin: 10px 0 4px 0;
		text-shadow: 0 0 20px rgba(140, 170, 255, 0.85);
	}

	.pairing-status {
		font-size: 0.95rem;
		opacity: 0.8;
		margin-bottom: 8px;
		text-align: center;
	}

	.pairing-buttons {
		display: flex;
		gap: 10px;
		justify-content: center;
		margin-top: 10px;
	}

	.pairing-buttons button {
		flex: 1;
		padding: 9px 12px;
		border-radius: 999px;
		border: none;
		cursor: pointer;
		font-size: 0.9rem;
		transition: 0.15s ease;
	}

	.pairing-buttons button.enable {
		background: linear-gradient(90deg, #465cff, #6a82ff);
		color: white;
	}

	.pairing-buttons button.disable {
		background: rgba(255, 255, 255, 0.08);
		color: white;
	}

	.pairing-buttons button:disabled {
		opacity: 0.6;
		cursor: default;
	}

	.pairing-message {
		text-align: center;
		font-size: 0.9rem;
		margin-top: 8px;
		opacity: 0.9;
	}

	.activity-box {
		max-height: 250px;
		overflow-y: auto;
		font-size: 0.95rem;
	}

	.progress-bar {
		width: 100%;
		height: 10px;
		background: rgba(255, 255, 255, 0.08);
		border-radius: 6px;
		margin-top: 4px;
	}

	.progress-fill {
		height: 100%;
		border-radius: 6px;
		background: linear-gradient(90deg, #5a6bff, #8694ff);
		transition: width 0.4s ease;
	}

	/* Floating ORB assistant */
	.orb-float {
		position: fixed;
		bottom: 25px;
		right: 25px;
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background-image: url('../../lib/assets/orb_enkel.JPG');
		background-size: cover;
		background-position: center;
		box-shadow: 0 0 25px rgba(110, 140, 255, 0.6);
		animation: orbFloat 4s infinite ease-in-out;
		cursor: pointer;
		z-index: 999;
		transition: 0.3s ease;
	}

	.orb-float:hover {
		transform: scale(1.1) translateY(-4px);
		box-shadow: 0 0 35px rgba(140, 170, 255, 0.9);
	}

	@keyframes orbFloat {
		0% {
			transform: translateY(0px);
		}
		50% {
			transform: translateY(-8px);
		}
		100% {
			transform: translateY(0px);
		}
	}
</style>

<div class="page">
	<header>
		<div class="title-block">
			<h1>ELYSIA Core Dashboard</h1>
			<div class="subtitle">Monitor your node, control pairing & modules.</div>
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
				<div class="status-item">
					<span>Status</span>
					<strong>{kernelStatus.status}</strong>
				</div>
				<div class="status-item">
					<span>Version</span>
					<strong>{kernelStatus.version}</strong>
				</div>
				<div class="status-item">
					<span>Modules</span>
					<strong>{kernelStatus.modules}</strong>
				</div>
				<div class="status-item">
					<span>Database</span>
					<strong>{kernelStatus.db}</strong>
				</div>
				<div class="status-item">
					<span>Port</span>
					<strong>{kernelStatus.port}</strong>
				</div>
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
				<p class="pairing-status">
					{#if pairingActive}
						Pairing is <strong>ACTIVE</strong>.
					{:else}
						Pairing is <strong>DISABLED</strong>.
					{/if}
				</p>

				{#if pairingActive && pairingCode}
					<div class="pairing-code">
						{pairingCode}
					</div>
					<p class="pairing-status">Enter this code on your device within the next few minutes.</p>
				{:else if pairingActive}
					<p class="pairing-status">Pairing active, waiting for device…</p>
				{:else}
					<p class="pairing-status">Enable pairing to connect new ORBIT / CARE devices.</p>
				{/if}

				<div class="pairing-buttons">
					<button
						class="enable"
						on:click={enablePairing}
						disabled={pairingBusy}
					>
						Enable 5 min
					</button>
					<button
						class="disable"
						on:click={disablePairing}
						disabled={pairingBusy || !pairingActive}
					>
						Disable
					</button>
				</div>

				{#if pairingMessage}
					<p class="pairing-message">{pairingMessage}</p>
				{/if}
			{/if}
		</div>

		<!-- MODULE LOAD (FAKE / PRETEND) -->
		<div class="card">
			<h2>Module Load</h2>
			{#each moduleStats as m}
				<div style="margin-bottom: 14px;">
					<div class="status-item">
						<span>{m.name}</span>
						<strong>{m.load}%</strong>
					</div>
					<div class="progress-bar">
						<div class="progress-fill" style={`width: ${m.load}%`}></div>
					</div>
				</div>
			{/each}
		</div>
	</div>

	<!-- Floating ORB assistant -->
	<div class="orb-float" aria-hidden="true"></div>
</div>
