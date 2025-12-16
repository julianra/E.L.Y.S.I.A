<!-- =========================================================
📍 FILE: src/pages/Dashboard.svelte
📝 ROL:
  ELYSIA Kernel – Centrale kern met orbitale navigatie
========================================================= -->

<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { isAuthenticated } from "../stores/kernel";
  import { currentPage } from "../stores/router";

  let kernstabiliteit = 88;
  let systeemdruk = 38;
  let geheugensamenhang = 79;
  let eventdoorvoer = 323;
  let uptime = 0;

  let interval: any;

  function variatie(waarde: number, min: number, max: number, stap: number) {
    const delta = Math.floor(Math.random() * stap * 2) - stap;
    return Math.min(max, Math.max(min, waarde + delta));
  }

  onMount(() => {
    interval = setInterval(() => {
      kernstabiliteit = variatie(kernstabiliteit, 85, 98, 1);
      systeemdruk = variatie(systeemdruk, 30, 65, 3);
      geheugensamenhang = variatie(geheugensamenhang, 75, 95, 2);
      eventdoorvoer = variatie(eventdoorvoer, 280, 520, 14);
      uptime++;
    }, 1000);
  });

  onDestroy(() => clearInterval(interval));

  function formatUptime(s: number) {
    const h = Math.floor(s / 3600);
    const m = Math.floor((s % 3600) / 60);
    const sec = s % 60;
    return `${h.toString().padStart(2,"0")}:${m.toString().padStart(2,"0")}:${sec.toString().padStart(2,"0")}`;
  }

  function logout() {
    localStorage.removeItem("elysia_admin_token");
    isAuthenticated.set(false);
    currentPage.set("login");
  }
</script>

<div class="elysia-ui">

  <!-- BOVENBALK -->
  <header class="topbar">
    <div class="merk">
      <span class="status-dot"></span>
      <strong>ELYSIA KERNEL</strong>
      <span class="sub">LOKALE NODE</span>
    </div>

    <button class="logout" on:click={logout}>Afmelden</button>
  </header>

  <!-- STATISTIEKEN -->
  <div class="stats links">
    <div><span>KERNSTABILITEIT</span><strong>{kernstabiliteit}%</strong></div>
    <div><span>SYSTEEMDRUK</span><strong>{systeemdruk}%</strong></div>
    <div><span>GEHEUGENSAMENHANG</span><strong>{geheugensamenhang}%</strong></div>
  </div>

  <div class="stats rechts">
    <div><span>EVENTDOORVOER</span><strong>{eventdoorvoer}/s</strong></div>
    <div><span>UPTIME</span><strong>{formatUptime(uptime)}</strong></div>
  </div>

  <!-- PODIUM -->
  <main class="podium">

    <!-- KERN -->
    <div class="kern">

      <div class="kern-gloed"></div>

      <!-- RONDE LOGO KERN -->
      <div class="kern-bol">
        <div class="logo-mask">
          <img src="/assets/logo.png" alt="ELYSIA logo" />
        </div>
      </div>

      <!-- BANEN -->
      <div class="baan baan-1"></div>
      <div class="baan baan-2"></div>
      <div class="baan baan-3"></div>

      <!-- ORBITEN -->
      <div class="orbit orbit-1">
        <span style="--a:0deg">OVERZICHT</span>
        <span style="--a:120deg">MODULES</span>
        <span style="--a:240deg">LOGS</span>
      </div>

      <div class="orbit orbit-2">
        <span style="--a:30deg">ENERGIE</span>
        <span style="--a:150deg">NETWERK</span>
        <span style="--a:270deg">VEILIGHEID</span>
      </div>

      <div class="orbit orbit-3">
        <span style="--a:90deg">SCENARIO’S</span>
        <span style="--a:270deg">DIAGNOSE</span>
      </div>

    </div>

  </main>

  <footer class="voet">
    ELYSIA Kernel • Local-first • Ambient systeeminterface
  </footer>

</div>

<style>
/* =========================================================
   BASIS
========================================================= */

.elysia-ui{
  height:100vh;
  background:
    radial-gradient(1200px 600px at 50% -10%, rgba(120,120,255,.22), transparent 60%),
    linear-gradient(180deg,#0a0d14,#07090e);
  color:#e6e8ee;
  font-family:system-ui,sans-serif;
  overflow:hidden;
}

/* =========================================================
   BOVENBALK
========================================================= */

.topbar{
  display:flex;
  justify-content:space-between;
  align-items:center;
  padding:16px 32px;
}

.merk{
  display:flex;
  gap:10px;
  letter-spacing:.2em;
}

.status-dot{
  width:10px;
  height:10px;
  border-radius:50%;
  background:#7affd8;
  box-shadow:0 0 16px #7affd8;
}

.sub{font-size:11px;opacity:.5;}

.logout{
  font-size:12px;
  padding:6px 14px;
  border-radius:999px;
  border:1px solid rgba(255,255,255,.15);
  background:transparent;
  color:#aaa;
}

/* =========================================================
   STATISTIEKEN
========================================================= */

.stats{
  position:absolute;
  top:72px;
  display:flex;
  gap:24px;
  font-size:11px;
  letter-spacing:.15em;
}

.stats strong{
  display:block;
  font-size:16px;
}

.links{left:32px;}
.rechts{right:32px;text-align:right;}

/* =========================================================
   PODIUM
========================================================= */

.podium{
  position:relative;
  height:100%;
  display:grid;
  place-items:center;
}

/* =========================================================
   KERN
========================================================= */

.kern{
  position:relative;
  width:520px;
  height:520px;
  display:grid;
  place-items:center;
}

/* --- kern bol --- */
.kern-bol{
  width:120px;
  height:120px;
  border-radius:50%;
  background:
    radial-gradient(circle at 30% 30%,#fff,#9cf 40%,#4f5cff 75%);
  box-shadow:0 0 90px rgba(120,120,255,.8);
  display:grid;
  place-items:center;
  animation:ademen 4s ease-in-out infinite;
  z-index:3;
}

/* logo netjes rond */
.logo-mask{
  width:64px;
  height:64px;
  border-radius:50%;
  overflow:hidden;
  display:grid;
  place-items:center;
  box-shadow:0 0 20px rgba(120,120,255,.6);
}

.logo-mask img{
  width:100%;
  height:100%;
  object-fit:cover;
}

/* gloed */
.kern-gloed{
  position:absolute;
  width:220px;
  height:220px;
  border-radius:50%;
  background:rgba(120,120,255,.35);
  filter:blur(45px);
}

/* =========================================================
   BANEN
========================================================= */

.baan{
  position:absolute;
  border-radius:50%;
  border:1px solid rgba(255,255,255,.12);
}

.baan-1{width:220px;height:220px;}
.baan-2{width:320px;height:320px;}
.baan-3{width:420px;height:420px;}

/* =========================================================
   ORBITEN (tekst altijd recht)
========================================================= */

.orbit{
  position:absolute;
  inset:0;
  animation:draaien linear infinite;
}

.orbit span{
  position:absolute;
  top:50%;
  left:50%;
  transform:
    rotate(var(--a))
    translate(var(--r))
    rotate(calc(-1 * var(--a)));
  font-size:11px;
  letter-spacing:.18em;
  color:#aaa;
  opacity:.75;
  white-space:nowrap;
}

.orbit-1{animation-duration:48s;}
.orbit-2{animation-duration:70s;}
.orbit-3{animation-duration:95s;}

.orbit-1 span{--r:110px;}
.orbit-2 span{--r:160px;}
.orbit-3 span{--r:210px;}

/* =========================================================
   VOET
========================================================= */

.voet{
  position:absolute;
  bottom:10px;
  width:100%;
  text-align:center;
  font-size:11px;
  opacity:.4;
}

/* =========================================================
   ANIMATIES
========================================================= */

@keyframes ademen{
  0%,100%{transform:scale(1);}
  50%{transform:scale(1.08);}
}

@keyframes draaien{
  from{transform:rotate(0deg);}
  to{transform:rotate(360deg);}
}
</style>
