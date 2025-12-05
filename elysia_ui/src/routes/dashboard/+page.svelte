<script>
    import { onMount } from 'svelte';
    import orb from '$lib/assets/orb_enkel.JPG';

    // Fake "real" data
    let systemStatus = {
        kernel: "Online",
        modulesActive: 12,
        cpuLoad: 23,
        memoryUsage: 41,
        uptime: "12h 34m",
    };

    let activity = [
        { time: "10:32", text: "MARTHE scheduled: 'Workday planning updated'" },
        { time: "10:30", text: "JUNK indexed 243 new memories" },
        { time: "10:28", text: "CATNIP registered 4 new IoT events" },
        { time: "10:21", text: "NAVI recalculated preferred routes" },
        { time: "10:10", text: "CORE heartbeat stable" }
    ];

    let moduleStats = [
        { name: "MARTHE", load: 78 },
        { name: "JUNK", load: 55 },
        { name: "CATNIP", load: 32 },
        { name: "NAVI", load: 61 },
        { name: "HAVEN", load: 22 },
        { name: "SHIELD", load: 44 }
    ];

    let hovering = false;
</script>

<style>
    .page {
        height: 100vh;
        width: 100%;
        display: flex;
        flex-direction: column;
        background: radial-gradient(circle at top, #111622, #05060a 70%);
        color: white;
        overflow: hidden;
        animation: fadeIn 1s ease-out;
        padding: 20px 40px;
    }

    h1 {
        font-size: 2.2rem;
        font-weight: 600;
        text-shadow: 0 0 15px rgba(140,160,255,0.5);
        margin-bottom: 10px;
    }

    .subtitle {
        opacity: 0.7;
        margin-bottom: 30px;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 25px;
    }

    .card {
        background: rgba(255,255,255,0.05);
        border-radius: 14px;
        padding: 22px;
        backdrop-filter: blur(14px);
        border: 1px solid rgba(255,255,255,0.1);
        box-shadow: 0 0 25px rgba(0,0,0,0.4);
        animation: slideUp 0.8s ease-out;
    }

    .card h2 {
        font-size: 1.4rem;
        margin-bottom: 15px;
        text-shadow: 0 0 10px rgba(120,140,255,0.45);
    }

    .status-item {
        display: flex;
        justify-content: space-between;
        margin-bottom: 10px;
    }

    .activity-box {
        max-height: 250px;
        overflow-y: auto;
    }

    .activity-item {
        padding: 8px 0;
        border-bottom: 1px solid rgba(255,255,255,0.06);
        opacity: 0.9;
    }

    .progress-bar {
        width: 100%;
        height: 10px;
        background: rgba(255,255,255,0.08);
        border-radius: 6px;
        margin-top: 5px;
    }

    .progress-fill {
        height: 100%;
        border-radius: 6px;
        background: linear-gradient(90deg, #5a6bff, #8694ff);
        transition: width 0.4s ease;
    }

    /* Floating ORB assistant */
    .orb {
        position: fixed;
        bottom: 25px;
        right: 25px;
        width: 90px;
        height: 90px;
        border-radius: 50%;
        background-image: url(../../lib/assets/orb_enkel.JPG);
        background-size: cover;
        background-position: center;
        box-shadow: 0 0 25px rgba(110,140,255,0.6);
        animation: orbFloat 4s infinite ease-in-out;
        cursor: pointer;
        z-index: 999;
        transition: 0.3s ease;
    }

    .orb:hover {
        transform: scale(1.12);
        box-shadow: 0 0 35px rgba(140,170,255,0.9);
    }

    @keyframes orbFloat {
        0% { transform: translateY(0px); }
        50% { transform: translateY(-10px); }
        100% { transform: translateY(0px); }
    }

    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }

    @keyframes slideUp {
        from { opacity: 0; transform: translateY(25px); }
        to { opacity: 1; transform: translateY(0); }
    }
</style>

<div class="page">
    <h1>ELYSIA Dashboard</h1>
    <p class="subtitle">Your system is running smoothly ✔</p>

    <div class="grid">

        <!-- SYSTEM STATUS -->
        <div class="card">
            <h2>System Status</h2>
            <div class="status-item"><span>Kernel:</span><strong>{systemStatus.kernel}</strong></div>
            <div class="status-item"><span>Modules Active:</span><strong>{systemStatus.modulesActive}</strong></div>
            <div class="status-item"><span>CPU Load:</span><strong>{systemStatus.cpuLoad}%</strong></div>
            <div class="status-item"><span>Memory:</span><strong>{systemStatus.memoryUsage}%</strong></div>
            <div class="status-item"><span>Uptime:</span><strong>{systemStatus.uptime}</strong></div>
        </div>

        <!-- ACTIVITY FEED -->
        <div class="card">
            <h2>Recent Activity</h2>
            <div class="activity-box">
                {#each activity as a}
                    <div class="activity-item">
                        <strong>{a.time}</strong> — {a.text}
                    </div>
                {/each}
            </div>
        </div>

        <!-- MODULE LOAD -->
        <div class="card">
            <h2>Module Load</h2>
            {#each moduleStats as m}
                <div style="margin-bottom: 16px;">
                    <strong>{m.name}</strong>
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {m.load}%"></div>
                    </div>
                </div>
            {/each}
        </div>

    </div>

    <!-- Floating ORB assistant -->
    <div class="orb"></div>
</div>
