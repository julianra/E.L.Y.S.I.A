<!-- ======================================================================
📍 FILE: src/routes/+page.svelte
====================================================================== -->

<script>
    import { onMount } from 'svelte';
    import { getInitialState } from '$lib/auth';
    import logo from '$lib/assets/logo.png';

    let statusText = "Connecting to ELYSIA Kernel…";

    onMount(async () => {
        try {
            const state = await getInitialState();

            if (state.admin_exists === false) {
                statusText = "No admin found — starting system setup…";
                window.location.href = "/admin/setup";
                return;
            }

            const token = localStorage.getItem("elysia_admin_token");

            if (token) {
                statusText = "Admin session found — opening dashboard…";
                window.location.href = "/dashboard";
                return;
            }

            statusText = "Redirecting to login…";
            window.location.href = "/login";
        } catch (e) {
            statusText = "Kernel offline — retrying…";
            console.error(e);
        }
    });
</script>

<style>
    main.splash {
        height: 100vh;
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        background: radial-gradient(circle at center, #0d1119, #05060a);
        color: white;
        text-align: center;
        font-family: system-ui, sans-serif;
    }

    .orb {
    width: 200px;
    height: 200px;
    border-radius: 50%;
    background-size: cover;
    background-position: center;
    margin-bottom: 24px;
    box-shadow: 0 0 40px rgba(120, 140, 255, 0.7);
    animation: orbPulse 4s infinite ease-in-out;
}


    @keyframes orbPulse {
        0% { box-shadow: 0 0 20px rgba(90,120,255,0.4); }
        50% { box-shadow: 0 0 50px rgba(140,170,255,0.9); }
        100% { box-shadow: 0 0 20px rgba(90,120,255,0.4); }
    }
</style>

<main class="splash">
    <div class="orb" style="background-image: url({logo});"></div>
    <h1>ELYSIA Core</h1>
    <p>{statusText}</p>
</main>
