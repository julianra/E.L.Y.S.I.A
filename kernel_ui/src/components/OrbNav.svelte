<!-- =========================================================
📍 FILE: src/components/OrbNav.svelte
📝 ROLE:
  Orb Navigation Shell (UI-only, fase 1)
  - Toont Orb altijd (na login via Viewport)
  - Klik op Orb → opent/sluit halve-cirkel menu
  - GEEN router, GEEN kernel calls, GEEN AI (nog)
========================================================= -->

<script lang="ts">
  import Orb from "./Orb.svelte";

  let open = false;

  type OrbAction = {
    id: string;
    label: string;
    hint?: string;
  };

  // UI placeholders (later: kernel-intents + capability gating)
  const actions: OrbAction[] = [
    { id: "overview", label: "Overzicht", hint: "Kernel status samenvatting" },
    { id: "modules", label: "Modules", hint: "Installed / paired modules" },
    { id: "logs", label: "Logs", hint: "Kernel logs & events" },
    { id: "security", label: "Veiligheid", hint: "Auth, pairing, policies" },
    { id: "energy", label: "Energie", hint: "Node load / health" },
    { id: "settings", label: "Instellingen", hint: "UI instellingen" }
  ];

  function toggle() {
    open = !open;
  }

  function close() {
    open = false;
  }

  // 180° boog van links naar rechts (boven de orb)
  function angleFor(index: number, total: number) {
    if (total <= 1) return -90;
    const start = -180;
    const end = 0;
    const step = (end - start) / (total - 1);
    return start + index * step;
  }

  function onAction(action: OrbAction) {
    // UI-only: voorlopig enkel sluiten (later kernel-intent)
    console.log("[ORB] action:", action.id);
    close();
  }
</script>

<!-- Floating layer -->
<div class="orb-layer">
  <!-- Click-catcher: sluit menu als je naast de orb klikt -->
  {#if open}
    <button class="backdrop" aria-label="Sluit Orb menu" on:click={close}></button>
  {/if}

  <div class="orb-stack {open ? 'open' : ''}">

    {#if open}
      <div class="menu" role="menu" aria-label="Orb menu">
        {#each actions as action, i}
          <button
            class="menu-item"
            style="--angle:{angleFor(i, actions.length)}deg;"
            role="menuitem"
            title={action.hint ?? action.label}
            on:click={() => onAction(action)}
          >
            <span class="label">{action.label}</span>
          </button>
        {/each}
      </div>
    {/if}

    <div class="orb-hit">
      <Orb size={84} on:toggle={toggle} />
    </div>
  </div>
</div>

<style>
  /* ===== Global floating layer ===== */
  .orb-layer {
    position: fixed;
    inset: 0;
    pointer-events: none; /* default: niets blokkeert je UI */
    z-index: 9999;
  }

  /* Click-catcher enkel actief wanneer open */
  .backdrop {
    position: absolute;
    inset: 0;
    pointer-events: auto;
    background: transparent;
    border: 0;
    padding: 0;
    margin: 0;
  }

  /* ===== Orb anchor position =====
     Pas dit aan: right/bottom voor docking
  */
  .orb-stack {
    position: absolute;
    right: 26px;
    bottom: 26px;

    width: 220px;
    height: 220px;

    pointer-events: none;
  }
.orb-stack.open .orb-hit {
  right: 64px;
}

  /* Zorg dat Orb zelf klikbaar is */
  .orb-hit {
    position: absolute;
    right: 0;
    bottom: 0;
    pointer-events: auto;
  }

  /* ===== Menu layout (halve cirkel boven de orb) ===== */
  .menu {
    position: absolute;
    inset: 0;
    pointer-events: none;
  }

  .menu-item {
    position: absolute;
    right: 42px;   /* centrering t.o.v. orb (afhankelijk van size) */
    bottom: 42px;

    transform:
      rotate(var(--angle))
      translate(98px)
      rotate(calc(-1 * var(--angle)));

    transform-origin: center;
    pointer-events: auto;

    border: 1px solid rgba(255, 255, 255, 0.10);
    border-radius: 999px;
    padding: 10px 14px;

    background: rgba(14, 18, 34, 0.82);
    backdrop-filter: blur(10px);

    color: rgba(240, 245, 255, 0.92);
    font-size: 12px;
    letter-spacing: 0.2px;

    box-shadow:
      0 0 0 rgba(0,0,0,0),
      0 10px 30px rgba(0, 0, 0, 0.35);

    opacity: 0;
    animation: pop 220ms ease-out forwards;
  }

  .menu-item:hover {
    border-color: rgba(120, 150, 255, 0.35);
    box-shadow:
      0 0 18px rgba(90, 120, 255, 0.22),
      0 12px 34px rgba(0, 0, 0, 0.38);
  }

  .label {
    display: inline-block;
    transform: translateY(0.5px);
    white-space: nowrap;
  }

  @keyframes pop {
    from {
      opacity: 0;
      transform:
        rotate(var(--angle))
        translate(72px)
        rotate(calc(-1 * var(--angle)))
        scale(0.98);
      filter: blur(1px);
    }
    to {
      opacity: 1;
      transform:
        rotate(var(--angle))
        translate(98px)
        rotate(calc(-1 * var(--angle)))
        scale(1);
      filter: blur(0px);
    }
  }

  /* Kleine responsiviteit */
  @media (max-width: 520px) {
    .orb-stack {
      right: 16px;
      bottom: 16px;
      width: 200px;
      height: 200px;
    }

    .menu-item {
      right: 40px;
      bottom: 40px;
      transform:
        rotate(var(--angle))
        translate(90px)
        rotate(calc(-1 * var(--angle)));
    }
  }
</style>
