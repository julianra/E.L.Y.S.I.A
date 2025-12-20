<!-- =========================================================
📍 FILE: src/App.svelte
📝 ROLE:
  Centrale router / bootstrapper van de ELYSIA Kernel UI.
  Bevat GEEN visuele overlays (Orb, HUD, alerts).
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

  import AdminSetup from "./pages/AdminSetup.svelte";
  import Login from "./pages/Login.svelte";
  import Dashboard from "./pages/Dashboard.svelte";
  import Modules from "./pages/Modules.svelte";
  import Loading from "./components/Loading.svelte";

  let loading = true;

  onMount(async () => {
    loading = true;

    try {
      // ==================================================
      // BOOTSTRAP — ONLY CHECK IF ADMIN EXISTS
      // ==================================================
      const adminRes = await api("/auth/has_admin");

      if (!adminRes || adminRes.success === false) {
        kernelOnline.set(false);
        currentPage.set("start");
        loading = false;
        return;
      }

      kernelOnline.set(true);
      adminExists.set(adminRes.exists);

      // ==================================================
      // ROUTING DECISION
      // ==================================================
      if (!adminRes.exists) {
        currentPage.set("admin-setup");
        loading = false;
        return;
      }

      const token = localStorage.getItem("elysia_admin_token");

      if (token) {
        authToken.set(token);
        isAuthenticated.set(true);
        currentPage.set("dashboard");
      } else {
        isAuthenticated.set(false);
        currentPage.set("login");
      }

    } catch (err) {
      console.error("Bootstrap error:", err);
      kernelOnline.set(false);
      currentPage.set("start");
    } finally {
      loading = false;
    }
  });
</script>

<!-- =========================================================
  MAIN VIEWPORT
========================================================= -->
<Viewport>
  {#if loading}
    <Loading />
  {:else if $currentPage === "admin-setup"}
    <AdminSetup />
  {:else if $currentPage === "login"}
    <Login />
  {:else if $currentPage === "dashboard"}
    <Dashboard />
  {:else if $currentPage === "modules"}
    <Modules />
  {:else}
    <Loading />
  {/if}
</Viewport>
