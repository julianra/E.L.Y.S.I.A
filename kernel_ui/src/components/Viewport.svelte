<!-- =========================================================
📍 FILE: src/components/Viewport.svelte
📝 ROLE:
  Enige echte fullscreen container.
  Beheert viewport en globale overlays (Orb, alerts, HUD).
========================================================= -->

<script lang="ts">
  import { isAuthenticated } from "../stores/kernel";
  import OrbNav from "./OrbNav.svelte";
</script>

<div class="viewport">
  <slot />

  {#if $isAuthenticated}
    <OrbNav />
  {/if}
</div>

<style>
  .viewport {
    position: relative;
    min-height: 100vh;

    background: radial-gradient(
      ellipse at center,
      #0b0f1a 0%,
      #06080f 100%
    );
  }
</style>
