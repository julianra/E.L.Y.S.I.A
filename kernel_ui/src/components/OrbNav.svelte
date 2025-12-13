<!-- =========================================================
📍 FILE: src/components/OrbNav.svelte
📝 ROLE:
  Vaste navigatie-wrapper rond bestaande Orb.
  Positioneert Orb altijd rechts-onder na login.
========================================================= -->

<script lang="ts">
  import Orb from "./Orb.svelte";
</script>

<div class="orb-nav">
  <!-- bestaande Orb, NIET dupliceren -->
  <Orb size={72} />
</div>

<style>
  .orb-nav {
    position: fixed;
    right: 24px;
    bottom: 24px;

    z-index: 9999; /* 🔑 hoger dan panels, viewport, backgrounds */

    display: flex;
    align-items: center;
    justify-content: center;

    pointer-events: auto;
  }
</style>

