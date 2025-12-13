<!-- =========================================================
📍 FILE: src/App.svelte
📝 ROLE:
  Centrale router / bootstrapper van de Kernel UI.
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

  import Viewport from "./components/Viewport.svelte";
  import OrbNav from "./components/OrbNav.svelte";

  import AdminSetup from "./pages/AdminSetup.svelte";
  import Login from "./pages/Login.svelte";
  import Dashboard from "./pages/Dashboard.svelte";
  import Loading from "./components/Loading.svelte";

  let loading = true;

  onMount(async () => {
    loading = true;

    const status = await api("/status");

    if (!status || status.success === false) {
      kernelOnline.set(false);
      currentPage.set("loading");
      loading = false;
      return;
    }

    kernelOnline.set(true);

    const res = await api("/auth/has_admin");

    if (!res || res.success === false) {
      currentPage.set("loading");
      loading = false;
      return;
    }

    adminExists.set(res.exists);

    if (!res.exists) {
      currentPage.set("admin_setup");
      loading = false;
      return;
    }

    const token = localStorage.getItem("elysia_admin_token");
    if (token) {
      authToken.set(token);
      isAuthenticated.set(true);
      currentPage.set("dashboard");
    } else {
      currentPage.set("login");
    }

    loading = false;
  });
</script>

<!-- MAIN VIEW -->
<Viewport>
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
</Viewport>

<!-- ORB NAVIGATION (NA LOGIN) -->
{#if $isAuthenticated}
  <OrbNav />
{/if}
