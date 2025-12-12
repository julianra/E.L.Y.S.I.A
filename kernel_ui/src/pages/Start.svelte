<!-- =========================================================
📍 FILE: src/pages/Start.svelte
📝 ROLE:
  Startbeslisser van de UI.
  Bepaalt:
    - admin bestaat?
    - gebruiker ingelogd?
  Zet correct currentPage via store.
========================================================= -->

<script lang="ts">
  import { onMount } from "svelte";
  import { adminExists, isAuthenticated } from "../stores/kernel";
  import { currentPage } from "../stores/router";
  import Viewport from "../components/Viewport.svelte";
  import Orb from "../components/Orb.svelte";

  onMount(() => {
    if (!$adminExists) {
      currentPage.set("admin-setup");
      return;
    }

    if ($isAuthenticated) {
      currentPage.set("dashboard");
    } else {
      currentPage.set("login");
    }
  });
</script>

<Viewport>
  <Orb size={160} />
  <h1>ELYSIA Kernel</h1>
  <p>Initializing interface…</p>
</Viewport>

<style>
  h1 {
    margin: 0;
    font-size: 2rem;
  }

  p {
    opacity: 0.8;
  }
</style>
