<script>
  let kernelOnline = false
  let lastSeen = null

  if (window.elysia) {
    window.elysia.onKernelStatus((status) => {
      kernelOnline = status.online
      lastSeen = status.timestamp
    })
  }
</script>

<style>
  :global(body) {
    margin: 0;
    background: #0b0e14;
    color: #e6e6eb;
    font-family: system-ui, sans-serif;
  }

  .root {
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .panel {
    padding: 2.5rem 3rem;
    border-radius: 16px;
    background: #151a23;
    box-shadow: 0 0 50px rgba(120, 80, 255, 0.35);
    text-align: center;
    min-width: 360px;
  }

  .status {
    margin-top: 1rem;
    font-weight: 600;
    font-size: 1.1rem;
  }

  .online {
    color: #4ade80;
  }

  .offline {
    color: #f87171;
  }

  .dot {
    display: inline-block;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    margin-right: 8px;
    background: currentColor;
  }
</style>

<div class="root">
  <div class="panel">
    <h1>ELYSIA Kernel</h1>

    <div class="status {kernelOnline ? 'online' : 'offline'}">
      <span class="dot"></span>
      {kernelOnline ? 'ONLINE' : 'OFFLINE'}
    </div>

    {#if lastSeen}
      <p>Last check: {new Date(lastSeen).toLocaleTimeString()}</p>
    {/if}
  </div>
</div>
