<!-- ======================================================================
📍 FILE: src/components/CenterLayout.svelte
📝 ROLE:
  Volledig gecentreerde pagina-layout met ELYSIA background.
====================================================================== -->

<main>
  <slot />
</main>

<style>
  main {
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: radial-gradient(circle at center, #0d1119, #05060a);
    color: white;
    text-align: center;
    padding: 20px;
  }
</style>
