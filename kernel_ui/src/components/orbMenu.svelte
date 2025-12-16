<!-- =========================================================
📍 FILE: src/components/OrbMenu.svelte
📝 ROLE:
  Radiaal Orb-menu (halve cirkel)
========================================================= -->

<script lang="ts">
  import Orb from "./Orb.svelte";

  let open = false;

  const actions = [
    { id: "overview", label: "Overzicht" },
    { id: "modules", label: "Modules" },
    { id: "logs", label: "Logs" },
    { id: "security", label: "Veiligheid" },
    { id: "energy", label: "Energie" }
  ];

  function toggle() {
    open = !open;
  }

  function angleFor(index: number, total: number) {
    // 180° boog van links naar rechts
    const start = -180;
    const end = 0;
    const step = (end - start) / (total - 1);
    return start + index * step;
  }
</script>

<div class="orb-container">
  {#if open}
    <div class="menu">
      {#each actions as action, i}
        <button
          class="menu-item"
          style="
            --angle: {angleFor(i, actions.length)}deg;
          "
        >
          {action.label}
        </button>
      {/each}
    </div>
  {/if}

  <Orb size={88} on:toggle={toggle} />
</div>

<style>
  .orb-container {
    position: relative;
    width: 200px;
    height: 200px;
  }

  .menu {
    position: absolute;
    inset: 0;
    pointer-events: none;
  }

  .menu-item {
    position: absolute;
    bottom: 50%;
    left: 50%;

    transform:
      rotate(var(--angle))
      translate(90px)
      rotate(calc(-1 * var(--angle)));

    transform-origin: center;
    pointer-events: auto;

    padding: 8px 14px;
    border-radius: 999px;
    border: none;

    background: rgba(30, 40, 80, 0.85);
    color: white;
    font-size: 12px;
    backdrop-filter: blur(6px);

    opacity: 0;
    animation: pop 0.35s ease forwards;
  }

  @keyframes pop {
    from {
      opacity: 0;
      transform:
        rotate(var(--angle))
        translate(60px)
        rotate(calc(-1 * var(--angle)));
    }
    to {
      opacity: 1;
      transform:
        rotate(var(--angle))
        translate(90px)
        rotate(calc(-1 * var(--angle)));
    }
  }
</style>
