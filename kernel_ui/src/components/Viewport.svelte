<!-- =========================================================
📍 FILE: src/components/Viewport.svelte
📝 ROLE:
  Viewport die altijd past binnen Electron window
  zonder verticale scroll.
========================================================= -->

<main class="viewport">
  <div class="content">
    <slot />
  </div>
</main>

<style>
  .viewport {
    /* GEEN 100vh */
    min-height: 100%;
    width: 100%;

    display: flex;
    align-items: center;
    justify-content: center;

    padding: 24px;
    box-sizing: border-box;

    background: var(--bg-main);
    overflow: hidden; /* 👈 voorkomt scrollbars */
  }

  .content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;

    max-width: 420px;
    width: 100%;
  }
</style>
