<!-- =========================================================
📍 FILE: src/components/Orb.svelte
📝 ROLE:
  ELYSIA Orb – navigatie-entiteit.
  Render image als echte <img> voor maximale betrouwbaarheid.
========================================================= -->

<script lang="ts">
  export let size = 72;
  const SRC = `${import.meta.env.BASE_URL}assets/orb_enkel.jpg`;
</script>


<div class="orb" style="width: {size}px; height: {size}px;">
  <div class="glow"></div>

  <img class="orb-img" src={SRC} alt="ELYSIA Orb" draggable="false" />
</div>

<style>
  .orb {
    position: relative;
    border-radius: 50%;
    overflow: hidden; /* 🔑 clip image netjes rond */
    z-index: 1;

    box-shadow:
      0 0 18px rgba(90,120,255,0.55),
      0 0 36px rgba(90,120,255,0.35),
      inset 0 0 14px rgba(255,255,255,0.12);

    animation: orb-float 6s ease-in-out infinite;
  }

  .orb-img {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;        /* 🔑 altijd gevuld */
    object-position: center;
    display: block;

    /* tijdelijke fallback zichtbaar maken zelfs als image donker is */
    background: rgba(20, 24, 40, 0.9);
  }

  .glow {
    position: absolute;
    inset: -10px;
    border-radius: 50%;

    background:
      radial-gradient(
        circle,
        rgba(120,150,255,0.45),
        rgba(120,150,255,0.15) 40%,
        transparent 70%
      );

    filter: blur(14px);
    z-index: -1; /* mag nu wél, want orb heeft overflow hidden en img zit bovenin */
    pointer-events: none;
  }

  @keyframes orb-float {
    0%   { transform: translateY(0); }
    50%  { transform: translateY(-6px); }
    100% { transform: translateY(0); }
  }
</style>
