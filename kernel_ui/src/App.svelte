<script>
  import { onDestroy, onMount } from 'svelte';

  let status = 'initializing';
  let interval;

  async function updateStatus() {
    if (window.kernel && window.kernel.getStatus) {
      status = await window.kernel.getStatus();
    } else {
      status = 'no-bridge';
    }
  }

  onMount(() => {
    updateStatus();
    interval = setInterval(updateStatus, 1000);
  });

  onDestroy(() => {
    clearInterval(interval);
  });
</script>

<style>
  :global(body) {
    margin: 0;
    background: #0b0e14;
    color: #e6e6eb;
    font-family: system-ui, sans-serif;
  }

  .running {
    color: #4ade80;
  }

  .starting {
    color: #facc15;
  }

  .error {
    color: #f87171;
  }
</style>

<div class="app">
  <h1>ELYSIA Kernel UI</h1>
  <p class={status}>Kernel status: {status}</p>
</div>
