<!-- ======================================================================
📍 FILE: src/pages/AdminSetup.svelte
📝 First-Time Setup voor ELYSIA Desktop SPA.
====================================================================== -->

<script>
    import { onMount } from "svelte";
    import { getInitialState, createAdmin } from "../lib/auth.js";
    import orb from "../assets/orb_enkel.JPG";

    let loading = true;
    let adminExists = null;
    let username = "";
    let password = "";
    let error = "";
    let success = false;

    onMount(async () => {
        try {
            const state = await getInitialState();
            adminExists = state.admin_exists;
        } catch {
            adminExists = true; // fallback → ga naar login
        }

        loading = false;

        if (adminExists === true) {
            window.location.hash = "/login";
        }
    });

    async function submit() {
        error = "";
        success = false;

        const res = await createAdmin(username, password);

        if (!res?.success) {
            error = res?.error ?? "Unknown error.";
            return;
        }

        success = true;
        setTimeout(() => window.location.hash = "/login", 1200);
    }
</script>

<style>
    .page {
        height: 100vh;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        background: radial-gradient(circle at center, #0d1119, #05060a);
        color: white;
        text-align: center;
        font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    }

    .orb {
        width: 200px;
        height: 200px;
        border-radius: 50%;
        background-image: url('../assets/orb_enkel.JPG');
        background-size: cover;
        background-position: center;
        margin-bottom: 24px;
        box-shadow: 0 0 40px rgba(120,140,255,0.6);
    }

    .card {
        width: 360px;
        padding: 26px;
        border-radius: 14px;
        background: rgba(255,255,255,0.06);
        border: 1px solid rgba(255,255,255,0.12);
        backdrop-filter: blur(12px);
    }

    input {
        width: 90%;
        padding: 12px;
        margin-top: 12px;
        border-radius: 8px;
        border: none;
        background: rgba(255,255,255,0.1);
        color: white;
    }

    button {
        width: 95%;
        padding: 12px;
        margin-top: 20px;
        background: linear-gradient(90deg, #465CFF, #6A82FF);
        border: none;
        border-radius: 10px;
        color: white;
        cursor: pointer;
        font-size: 1rem;
    }
</style>

<div class="page">
    {#if loading}
        <p>Checking system state…</p>

    {:else if adminExists === false}
        <div class="orb"></div>

        <div class="card">
            <h1>Create Administrator</h1>

            <input
                placeholder="Username"
                bind:value={username}
            />

            <input
                type="password"
                placeholder="Password"
                bind:value={password}
            />

            <button on:click={submit}>Create Admin</button>

            {#if error}
                <p style="color:#ff6b6b;margin-top:10px;">{error}</p>
            {/if}

            {#if success}
                <p style="color:#6bff9c;margin-top:10px;">
                    Admin created! Redirecting…
                </p>
            {/if}
        </div>

    {:else}
        <p>Redirecting…</p>
    {/if}
</div>
