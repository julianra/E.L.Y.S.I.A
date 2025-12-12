<!-- =========================================================
📍 FILE: src/components/Orb.svelte
📝 ROLE:
  Centrale ELYSIA Orb – schaalbaar & veilig
========================================================= -->

<script lang="ts">
  import logo from '../assets/logo.png';
  export let size = 140; // 👈 KLEINER
</script>

<div
  class="orb"
  style="
    width: {size}px;
    height: {size}px;
    background-image: url('{logo}');
  "
></div>

<style>
  .orb {
    border-radius: 50%;
    background-size: cover;
    background-position: center;

    box-shadow:
      0 0 20px rgba(120,140,255,0.6),
      0 0 40px rgba(90,120,255,0.4);

    animation: pulse 4s ease-in-out infinite;
    flex-shrink: 0; /* 👈 nooit squashen */
  }

  @keyframes pulse {
    0% { box-shadow: 0 0 16px rgba(90,120,255,0.4); }
    50% { box-shadow: 0 0 40px rgba(140,170,255,0.9); }
    100% { box-shadow: 0 0 16px rgba(90,120,255,0.4); }
  }
</style>
