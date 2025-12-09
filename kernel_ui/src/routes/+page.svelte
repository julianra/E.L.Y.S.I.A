<!-- kernel_ui/src-tauri/src/routes/+page.svelte -->
<script>
  import logo from '../lib/assets/logo.png';
  let statusText = "Connecting to ELYSIA Kernel…";

  // Later replaced with real kernel checks
  setTimeout(() => {
    statusText = "Loading interface…";
  }, 800);
</script>

<style>
  main {
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: radial-gradient(circle at center, #0d1119, #05060a);
    color: white;
  }

  .orb {
    width: 200px;
    height: 200px;
    border-radius: 50%;
    background-image: url({logo});
    background-size: cover;
    background-position: center;
    box-shadow: 0 0 40px rgba(120, 140, 255, 0.7);
    animation: orbPulse 4s infinite ease-in-out;
    margin-bottom: 24px;
  }

  @keyframes orbPulse {
    0% { box-shadow: 0 0 20px rgba(90,120,255,0.4); }
    50% { box-shadow: 0 0 50px rgba(140,170,255,0.9); }
    100% { box-shadow: 0 0 20px rgba(90,120,255,0.4); }
  }
</style>

<main>
<div class="orb" style="background-image: url('{logo}');"></div>
  <h1>ELYSIA Core</h1>
  <p>{statusText}</p>
</main>
