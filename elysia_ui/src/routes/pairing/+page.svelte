<!-- ======================================================================
📍 FILE: src/routes/pairing/+page.svelte
📝 BESCHRIJVING:
  Device-side pairing wizard.

  Functionaliteit:
    - Leest /pair/status (public)
    - Indien pairing_active = false → uitleg tonen
    - Indien pairing_active = true → formulier om:
        device_name, os, model, code in te geven
    - POST naar /pair/complete
    - Bij success: toont device_id + device_token

  Dit is het scherm dat een toestel (gsm, tablet, laptop)
  kan gebruiken om zich te koppelen aan de ELYSIA Core node.
====================================================================== -->

<script>
	import { onMount } from 'svelte';
	import { getPairStatus, completePairing } from '$lib/pairing';
	import orb from '$lib/assets/orb_enkel.JPG';

	let loading = true;
	let kernelInfo = null;
	let pairingActive = false;
	let loadError = '';

	let deviceName = "Julian's Device";
	let os = 'android';
	let model = 'Unknown';

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

		// Af en toe herchecken of pairing ondertussen aan/uit gaat
		refreshInterval = setInterval(loadStatus, 5000);

		return () => {
			if (refreshInterval) clearInterval(refreshInterval);
		};
	});

	async function submit() {
		submitError = '';
		success = false;
		deviceId = null;
		deviceToken = null;

		if (!code || code.length < 4) {
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

			if (!res || res.success !== true) {
				submitError = res?.error ?? 'Pairing failed.';
				success = false;
			} else {
				success = true;
				deviceId = res.device_id ?? null;
				deviceToken = res.device_token ?? null;
			}
		} catch (e) {
			console.error(e);
			submitError = 'Pairing request failed.';
		} finally {
			submitting = false;
		}
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
		background-image: url('../../lib/assets/orb_enkel.JPG');
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
		background: rgba(255, 255, 255, 0.06);
		backdrop-filter: blur(14px);
		border: 1px solid rgba(255, 255, 255, 0.14);
		box-shadow: 0 0 28px rgba(0, 0, 0, 0.5);
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
		background: rgba(255, 255, 255, 0.1);
		color: white;
		outline: none;
		font-size: 0.95rem;
	}

	input:focus,
	select:focus {
		box-shadow: 0 0 10px rgba(120, 160, 255, 0.9);
	}

	.code-input {
		width: 100%;
		text-align: center;
		letter-spacing: 0.4em;
		font-weight: 600;
		font-size: 1.2rem;
	}

	button {
		width: 100%;
		padding: 12px;
		margin-top: 10px;
		border-radius: 10px;
		border: none;
		cursor: pointer;
		background: linear-gradient(90deg, #465cff, #6a82ff);
		color: white;
		font-weight: 500;
		font-size: 1rem;
		box-shadow: 0 0 18px rgba(80, 120, 255, 0.7);
		transition: 0.15s ease;
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
		font-size: 0.9rem;
		margin-top: 8px;
	}

	.success {
		color: #6bff9c;
		font-size: 0.9rem;
		margin-top: 8px;
		word-break: break-all;
		text-align: left;
	}

	@keyframes orbPulse {
		0% {
			box-shadow: 0 0 20px rgba(90, 120, 255, 0.4);
		}
		50% {
			box-shadow: 0 0 45px rgba(140, 170, 255, 0.9);
		}
		100% {
			box-shadow: 0 0 20px rgba(90, 120, 255, 0.4);
		}
	}
</style>

<div class="page">
	<div class="orb" aria-hidden="true"></div>

	<div class="card">
		{#if loading}
			<h1>Connecting…</h1>
			<p class="subtitle">Checking if ELYSIA is ready for pairing.</p>

		{:else if loadError}
			<h1>Pairing</h1>
			<p class="subtitle">{loadError}</p>

		{:else if !pairingActive}
			<h1>Pairing disabled</h1>
			<p class="subtitle">
				An admin needs to enable pairing in the ELYSIA Dashboard first.<br />
				Once pairing is enabled, this screen will update automatically.
			</p>

			{#if kernelInfo}
				<div class="kernel-info">
					Core node: {kernelInfo.node_id} • v{kernelInfo.version}<br />
					Capabilities: {kernelInfo.capabilities.join(', ')}
				</div>
			{/if}

		{:else}
			<h1>Pair your device</h1>
			<p class="subtitle">
				Enter your device info and the 6-digit pairing code shown on the ELYSIA Dashboard.
			</p>

			{#if kernelInfo}
				<div class="kernel-info">
					Pairing with: {kernelInfo.node_id} • v{kernelInfo.version}<br />
					Capabilities: {kernelInfo.capabilities.join(', ')}
				</div>
			{/if}

			<div class="form-row">
				<input placeholder="Device name" bind:value={deviceName} />
			</div>

			<div class="form-row">
				<select bind:value={os}>
					<option value="android">Android</option>
					<option value="ios">iOS</option>
					<option value="windows">Windows</option>
					<option value="linux">Linux</option>
					<option value="macos">macOS</option>
				</select>
				<input placeholder="Model" bind:value={model} />
			</div>

			<input
				class="code-input"
				placeholder="PAIR CODE"
				bind:value={code}
				maxlength="6"
				on:input={() => (code = code.replace(/[^0-9]/g, ''))}
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
					{#if deviceId}
						<p>Device ID: {deviceId}</p>
					{/if}
					{#if deviceToken}
						<p>Device token:</p>
						<p>{deviceToken}</p>
					{/if}
				</div>
			{/if}
		{/if}
	</div>
</div>
