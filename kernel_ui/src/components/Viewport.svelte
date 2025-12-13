<!-- =========================================================
📍 FILE: src/components/Viewport.svelte
📝 ROLE:
  Enige echte fullscreen container.
  Beheert viewport, centrering en voorkomt scroll.
========================================================= -->

<div class="viewport">
  <slot />
</div>

<style>
  .viewport {
    position: relative;
    min-height: 100vh;

    /* ❌ NIET */
    /* overflow: hidden; */

    background: radial-gradient(
      ellipse at center,
      #0b0f1a 0%,
      #06080f 100%
    );
  }
</style>

