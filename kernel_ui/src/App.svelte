<!-- =========================================================
📍 FILE: src/App.svelte
📝 ROLE:
  Centrale router / bootstrapper van de Kernel UI.
  Beslist:
   - is kernel online?
   - bestaat admin?
   - is gebruiker ingelogd?
========================================================= -->

<script lang="ts">
  import { onMount } from "svelte";

  import { api } from "./lib/api";
  import {
    kernelOnline,
    adminExists,
    isAuthenticated,
    authToken
  } from "./stores/kernel";

  import { currentPage } from "./stores/router";

  import AdminSetup from "./pages/AdminSetup.svelte";
  import Login from "./pages/Login.svelte";
  import Dashboard from "./pages/Dashboard.svelte";
  import Loading from "./components/Loading.svelte";

  let loading = true;

  onMount(async () => {
    loading = true;

    try {
      // 1️⃣ Kernel check
      await api("/status");
      kernelOnline.set(true);

      // 2️⃣ Admin exists check (SOURCE OF TRUTH)
      const res = await api("/auth/has_admin");
      adminExists.set(res.exists);

      if (!res.exists) {
        currentPage.set("admin_setup");
        loading = false;
        return;
      }

      // 3️⃣ Auth check
      const token = localStorage.getItem("elysia_admin_token");

      if (token) {
        authToken.set(token);
        isAuthenticated.set(true);
        currentPage.set("dashboard");
      } else {
        currentPage.set("login");
      }
    } catch (e) {
      kernelOnline.set(false);
      currentPage.set("loading");
    }

    loading = false;
  });
</script>

{#if loading}
  <Loading />

{:else if $currentPage === "admin_setup"}
  <AdminSetup />

{:else if $currentPage === "login"}
  <Login />

{:else if $currentPage === "dashboard"}
  <Dashboard />

{:else}
  <Loading />
{/if}
