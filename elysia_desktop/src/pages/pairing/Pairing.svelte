<!-- ======================================================================
📍 FILE: src/pages/pairing/Pairing.svelte
📝 Device-side pairing wizard for ELYSIA Desktop.
====================================================================== -->

<script>
	import { onMount } from 'svelte';
	import { getPairStatus, completePairing } from '../../lib/pairing.js';
	import orb from '../../assets/orb_enkel.JPG';

	let loading = true;
	let kernelInfo = null;
	let pairingActive = false;
	let loadError = '';

	let deviceName = "Julian's Device";
	let os = 'windows';
	let model = 'Desktop';

	let code = '';
	let submitting = false;
	let submitError = '';
	let success = false;
	let deviceId = null;
	let deviceToken = null;

	let refreshInterval = null;

	async function loadStatus() {
		try {
			const res = await getPairStatus();
			pairingActive = !!res.pairing_active;
			kernelInfo = res.kernel ?? null;
			loadError = '';
		} catch (e) {
			console.error(e);
			loadError = 'Could not reach ELYSIA Kernel.';
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		loadStatus();
		refreshInterval = setInterval(loadStatus, 5000);
		return () => clearInterval(refreshInterval);
	});

	async function submit() {
		submitError = '';
		success = false;

		if (!code || code.length < 6) {
			submitError = 'Please enter the 6-digit pairing code.';
			return;
		}

		submitting = true;

		try {
			const res = await completePairing({
				device_name: deviceName,
				os,
				model,
				code
			});

			if (!res?.success) {
				submitError = res?.error ?? 'Pairing failed.';
			} else {
				success = true;
				deviceId = res.device_id;
				deviceToken = res.device_token;
			}
		} catch (e) {
			submitError = 'Pairing request failed.';
		}

		submitting = false;
	}
</script>

<style>
	.page {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background: radial-gradient(circle at center, #0d1119, #05060a);
		color: white;
		font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		padding: 20px;
		box-sizing: border-box;
	}

	.orb {
		width: 160px;
		height: 160px;
		border-radius: 50%;
		background-image: url('../../assets/orb_enkel.JPG');
		background-size: cover;
		background-position: center;
		margin-bottom: 20px;
		box-shadow: 0 0 35px rgba(120, 140, 255, 0.8);
		animation: orbPulse 4s infinite ease-in-out;
	}

	.card {
		max-width: 420px;
		width: 100%;
		padding: 24px;
		border-radius: 16px;
		background: rgba(255,255,255,0.06);
		backdrop-filter: blur(14px);
		border: 1px solid rgba(255,255,255,0.14);
		box-shadow: 0 0 28px rgba(0,0,0,0.5);
		text-align: center;
	}

	h1 {
		font-size: 1.6rem;
		margin-bottom: 8px;
		text-shadow: 0 0 12px rgba(140, 160, 255, 0.7);
	}

	p.subtitle {
		opacity: 0.8;
		font-size: 0.95rem;
		margin-bottom: 16px;
	}

	.kernel-info {
		font-size: 0.85rem;
		opacity: 0.8;
		margin-bottom: 16px;
	}

	.form-row {
		display: flex;
		gap: 8px;
		margin-bottom: 8px;
	}

	input,
	select {
		flex: 1;
		padding: 10px;
		border-radius: 8px;
		border: none;
		background: rgba(255,255,255,0.1);
		color: white;
		font-size: 0.95rem;
	}

	.code-input {
		width: 100%;
		text-align: center;
		font-weight: 600;
		font-size: 1.2rem;
		letter-spacing: 0.4em;
	}

	button {
		width: 100%;
		padding: 12px;
		margin-top: 10px;
		border-radius: 10px;
		border: none;
		background: linear-gradient(90deg, #465cff, #6a82ff);
		color: white;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		box-shadow: 0 0 18px rgba(80,120,255,0.7);
	}

	button:hover {
		transform: translateY(-1px);
		box-shadow: 0 0 24px rgba(120,160,255,0.9);
	}

	button:disabled {
		opacity: 0.6;
		cursor: default;
	}

	.error {
		color: #ff6b6b;
		margin-top: 8px;
		font-size: 0.9rem;
	}

	.success {
		color: #6bff9c;
		font-size: 0.9rem;
		margin-top: 8px;
		word-break: break-all;
		text-align: left;
	}

	@keyframes orbPulse {
		0% { box-shadow: 0 0 20px rgba(90,120,255,0.4); }
		50% { box-shadow: 0 0 45px rgba(140,170,255,0.9); }
		100% { box-shadow: 0 0 20px rgba(90,120,255,0.4); }
	}
</style>

<div class="page">
	<div class="orb"></div>

	<div class="card">
		{#if loading}
			<h1>Connecting…</h1>
			<p class="subtitle">Checking ELYSIA pairing status…</p>

		{:else if loadError}
			<h1>Error</h1>
			<p class="subtitle">{loadError}</p>

		{:else if !pairingActive}
			<h1>Pairing disabled</h1>
			<p class="subtitle">
				Enable pairing from the admin dashboard.<br />
				This page refreshes automatically.
			</p>

			{#if kernelInfo}
				<div class="kernel-info">
					Node: {kernelInfo.node_id}<br />
					v{kernelInfo.version}<br />
					Capabilities: {kernelInfo.capabilities.join(', ')}
				</div>
			{/if}

		{:else}
			<h1>Pair your device</h1>
			<p class="subtitle">Enter the 6-digit pairing code shown on your dashboard.</p>

			<div class="form-row">
				<input placeholder="Device name" bind:value={deviceName} />
			</div>

			<div class="form-row">
				<select bind:value={os}>
					<option value="windows">Windows</option>
					<option value="linux">Linux</option>
					<option value="macos">MacOS</option>
					<option value="android">Android</option>
					<option value="ios">iOS</option>
				</select>

				<input placeholder="Model" bind:value={model} />
			</div>

			<input
				class="code-input"
				placeholder="PAIR CODE"
				bind:value={code}
				maxlength="6"
				on:input={() => code = code.replace(/[^0-9]/g, '')}
			/>

			<button on:click={submit} disabled={submitting}>
				{#if submitting}
					Pairing…
				{:else}
					Complete pairing
				{/if}
			</button>

			{#if submitError}
				<p class="error">{submitError}</p>
			{/if}

			{#if success}
				<div class="success">
					<p><strong>Paired successfully!</strong></p>
					<p>Device ID: {deviceId}</p>
					<p>Token: {deviceToken}</p>
				</div>
			{/if}
		{/if}
	</div>
</div>
