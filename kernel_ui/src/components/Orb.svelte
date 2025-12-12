<!-- ======================================================================
📍 FILE: src/components/Orb.svelte
📝 ROLE:
  ELYSIA ORB met pulse animatie.
====================================================================== -->

<script lang="ts">
  import logo from "../assets/logo.png";
  export let size = 180;
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
    margin-bottom: 24px;
    box-shadow: 0 0 40px rgba(120,140,255,0.6);
    animation: orbPulse 4s infinite ease-in-out;
  }

  @keyframes orbPulse {
    0% { box-shadow: 0 0 20px rgba(90,120,255,0.4); }
    50% { box-shadow: 0 0 50px rgba(140,170,255,0.9); }
    100% { box-shadow: 0 0 20px rgba(90,120,255,0.4); }
  }
</style>
