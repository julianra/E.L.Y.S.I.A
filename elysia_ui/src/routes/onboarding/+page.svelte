<!-- =========================================================================
📍 FILE: src/routes/onboarding/+page.svelte
📝 Futuristische onboarding interface met ORB visual & neon assistant rechts-onderaan
=========================================================================== -->

<script>
    import { onMount } from 'svelte';
    import { getInitialState, createAdmin } from '$lib/auth';
    import orbImage from '$lib/assets/orb_enkel.JPG';

    let loading = true;
    let adminExists = null;

    let username = '';
    let password = '';
    let createError = '';
    let createSuccess = false;

    onMount(async () => {
        const res = await getInitialState();
        adminExists = res.admin_exists;
        loading = false;
    });

    async function handleCreateAdmin() {
        createError = '';
        createSuccess = false;

        const res = await createAdmin(username, password);

        if (!res.success) {
            createError = res.error ?? 'Unknown error';
            return;
        }

        createSuccess = true;
        setTimeout(() => (window.location.href = '/login'), 1200);
    }
</script>

<style>
    .container {
        width: 100%;
        height: 100vh;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        background: radial-gradient(circle at center, #0d1119 0%, #05060a 100%);
        color: white;
        animation: fadeIn 0.7s ease-out;
    }

    /* -------- Grote orb bovenaan (logo) -------- */
    .orb {
        width: 220px;
        height: 220px;
        border-radius: 50%;
        background-image: url('../../lib/assets/orb_enkel.JPG');
        background-size: cover;
        background-position: center;
        margin-bottom: 30px;
        box-shadow: 0 0 40px rgba(80,120,255,0.55);
        animation: orbPulse 4s infinite ease-in-out;
        transition: transform 0.25s ease;
    }

    .orb:hover {
        transform: scale(1.05);
        box-shadow: 0 0 55px rgba(120,160,255,0.8);
    }

    @keyframes orbPulse {
        0% { box-shadow: 0 0 30px rgba(70,110,255,0.45); }
        50% { box-shadow: 0 0 60px rgba(120,160,255,0.85); }
        100% { box-shadow: 0 0 30px rgba(70,110,255,0.45); }
    }

    /* -------- Onboarding card -------- */
    .card {
        width: 380px;
        padding: 32px;
        border-radius: 14px;
        background: rgba(255,255,255,0.05);
        backdrop-filter: blur(14px);
        border: 1px solid rgba(255,255,255,0.13);
        box-shadow: 0 0 25px rgba(0,0,0,0.4);
        text-align: center;
        animation: slideUp 0.7s ease-out;
    }

    h1 {
        margin-bottom: 20px;
        font-weight: 600;
        letter-spacing: 1px;
        text-shadow: 0 0 10px rgba(120,140,255,0.55);
    }

    input {
        width: 90%;
        padding: 12px;
        margin: 10px 0;
        border-radius: 8px;
        border: none;
        background: rgba(255,255,255,0.08);
        color: white;
        outline: none;
    }

    input:focus {
        box-shadow: 0 0 10px rgba(100,140,255,0.6);
    }

    button {
        width: 95%;
        padding: 14px;
        margin-top: 20px;
        background: linear-gradient(90deg, #465CFF, #6A82FF);
        border: none;
        border-radius: 10px;
        color: white;
        font-size: 1.1rem;
        cursor: pointer;
        transition: 0.25s ease;
        box-shadow: 0 0 15px rgba(70,110,255,0.45);
    }

    button:hover {
        transform: translateY(-2px);
        box-shadow: 0 0 25px rgba(100,140,255,0.7);
    }

    /* -------- Kleine ORB rechts-onderaan -------- */
    .orb-small {
        position: fixed;
        bottom: 26px;
        right: 26px;
        width: 85px;
        height: 85px;
        border-radius: 50%;
        background-image: url('../../lib/assets/orb_enkel.JPG');
        background-size: cover;
        background-position: center;
        box-shadow: 0 0 22px rgba(100,140,255,0.5);
        cursor: pointer;
        z-index: 999;
        animation: orbSmallPulse 4s infinite ease-in-out;
        transition: transform 0.25s ease;
    }

    .orb-small:hover {
        transform: scale(1.10);
        box-shadow: 0 0 32px rgba(140,180,255,0.7);
    }

    @keyframes orbSmallPulse {
        0%   { box-shadow: 0 0 16px rgba(80,120,255,0.4); }
        50%  { box-shadow: 0 0 28px rgba(120,160,255,0.85); }
        100% { box-shadow: 0 0 16px rgba(80,120,255,0.4); }
    }

    /* -------- Animations -------- */
    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }

    @keyframes slideUp {
        from { transform: translateY(30px); opacity: 0; }
        to { transform: translateY(0); opacity: 1; }
    }
</style>

<div class="container">
    {#if loading}
        <p>Loading…</p>

    {:else if adminExists === false}
        <!-- Grote ORB -->
        <div class="orb"></div>

        <!-- Onboarding card -->
        <div class="card">
            <h1>Welcome to ELYSIA</h1>
            <p style="opacity:0.8;margin-bottom:20px;">
                Please create your administrator account
            </p>

            <input placeholder="Username" bind:value={username} />
            <input type="password" placeholder="Password" bind:value={password} />

            <button on:click={handleCreateAdmin}>Create Admin</button>

            {#if createError}
                <p style="color:#ff6b6b;margin-top:10px;">{createError}</p>
            {/if}

            {#if createSuccess}
                <p style="color:#6bff9c;margin-top:10px;">Admin created! Redirecting…</p>
            {/if}
        </div>

    {:else}
        <h1>Admin already exists — redirecting…</h1>
        <script> window.location.href = '/login'; </script>
    {/if}

    <!-- ⭐ Floating ORB assistant -->
    <div class="orb-small"></div>
</div>
