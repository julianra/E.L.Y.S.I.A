<!-- =========================================================================
📍 FILE: src/routes/login/+page.svelte
📝 BESCHRIJVING:
  Loginpagina voor bestaande admin accounts.
============================================================================ -->

<script>
    import { login } from '$lib/auth';

    let username = '';
    let password = '';
    let error = '';

    async function doLogin() {
        error = '';

        const res = await login(username, password);

        if (!res.success) {
            error = res.error ?? "Invalid credentials";
            return;
        }

        localStorage.setItem('elysia_token', res.token);
        window.location.href = '/dashboard';
    }
</script>

<h1>Login</h1>

<input placeholder="Username" bind:value={username} />
<input type="password" placeholder="Password" bind:value={password} />

<button on:click={doLogin}>Login</button>

{#if error}
    <p style="color:red;">{error}</p>
{/if}
