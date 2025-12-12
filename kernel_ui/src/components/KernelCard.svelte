<!-- ======================================================================
📍 FILE: src/components/KernelCard.svelte
📝 ROLE:
  Visuele statuskaart van de ELYSIA Kernel.
  Toont:
    - Online / Offline
    - Poort
    - Laatste check
====================================================================== -->

<script lang="ts">
  import { kernelOnline, lastCheck } from "../stores/kernel";
</script>

<div class="card">
  <h1>ELYSIA Kernel</h1>

  {#if $kernelOnline}
    <p class="ok">● ONLINE</p>
  {:else}
    <p class="err">● OFFLINE</p>
  {/if}

  <p>Port: 2022</p>
  <p>
    Last check:
    {$lastCheck ? $lastCheck.toLocaleTimeString() : "—"}
  </p>
</div>
