/**
 * ======================================================================
 * 📍 FILE: kernel_ui/electron/main.cjs
 * 📝 ELYSIA Kernel UI – Electron Main Process
 *
 * Fixes:
 * - Try multiple kernel ports (2022 + 3131)
 * - Start kernel with stable working directory (userData)
 * - Capture kernel stdout/stderr to a log file
 * - Safe IPC (no destroyed window errors)
 * ======================================================================
 */

const { app, BrowserWindow } = require('electron')
const path = require('path')
const { spawn } = require('child_process')
const http = require('http')
const fs = require('fs')

let mainWindow = null
let kernelProcess = null
let pollTimer = null

const isDev = !app.isPackaged
const PORT_CANDIDATES = [2022, 3131] // 2022 volgens je mDNS; 3131 als fallback
function copyDir(src, dest) {
  if (!fs.existsSync(src)) return

  fs.mkdirSync(dest, { recursive: true })

  for (const entry of fs.readdirSync(src, { withFileTypes: true })) {
    const srcPath = path.join(src, entry.name)
    const destPath = path.join(dest, entry.name)

    if (entry.isDirectory()) {
      copyDir(srcPath, destPath)
    } else {
      fs.copyFileSync(srcPath, destPath)
    }
  }
}

// ----------------------------------------------------------------------
// Paths
// ----------------------------------------------------------------------
function getKernelPath() {
  return isDev
    ? path.join(__dirname, '..', 'bin', 'kernel.exe')
    : path.join(process.resourcesPath, 'bin', 'kernel.exe')
}

function getKernelWorkDir() {
  // Stabiele locatie voor db/config/logs
  const dir = path.join(app.getPath('userData'), 'kernel')
  fs.mkdirSync(dir, { recursive: true })
  return dir
}

function getKernelLogPath() {
  return path.join(getKernelWorkDir(), 'kernel.log')
}

// ----------------------------------------------------------------------
// Safe IPC
// ----------------------------------------------------------------------
function safeSend(channel, payload) {
  if (!mainWindow) return
  if (mainWindow.isDestroyed()) return
  mainWindow.webContents.send(channel, payload)
}

function sendKernelStatus(online, port, detail) {
  safeSend('kernel-status', {
    online,
    port: port ?? null,
    detail: detail ?? null,
    timestamp: Date.now()
  })
}

// ----------------------------------------------------------------------
// Kernel process control
// ----------------------------------------------------------------------
function startKernel() {
  if (kernelProcess) return

  const kernelPath = getKernelPath()
  const cwd = getKernelWorkDir()
  const logPath = getKernelLogPath()
  // --------------------------------------------------
  // Copy bundled plugins → runtime kernel directory
  // --------------------------------------------------
  const bundledPlugins = isDev
    ? path.join(__dirname, '..', '..', 'plugins')
    : path.join(process.resourcesPath, 'plugins')

  const runtimePlugins = path.join(cwd, 'plugins')

  if (!fs.existsSync(runtimePlugins)) {
    copyDir(bundledPlugins, runtimePlugins)
  }

  const out = fs.createWriteStream(logPath, { flags: 'a' })
  out.write(`\n\n[UI] Starting kernel at ${new Date().toISOString()}\n`)
  out.write(`[UI] kernelPath=${kernelPath}\n`)
  out.write(`[UI] cwd=${cwd}\n`)

  kernelProcess = spawn(kernelPath, [], {
    cwd,
    windowsHide: true,
    stdio: ['ignore', 'pipe', 'pipe']
  })

  kernelProcess.stdout.on('data', (buf) => out.write(buf))
  kernelProcess.stderr.on('data', (buf) => out.write(buf))

  kernelProcess.on('error', (err) => {
    out.write(`[UI] spawn error: ${err?.message ?? err}\n`)
    sendKernelStatus(false, null, `spawn error: ${err?.message ?? err}`)
  })

  kernelProcess.on('exit', (code) => {
    out.write(`[UI] kernel exit: code=${code}\n`)
    kernelProcess = null
    sendKernelStatus(false, null, `kernel exited (${code})`)
  })
}

function stopKernel() {
  if (!kernelProcess) return
  try {
    kernelProcess.kill()
  } catch (_) {
    // ignore
  }
  kernelProcess = null
}

// ----------------------------------------------------------------------
// HTTP status polling (tries multiple ports)
// ----------------------------------------------------------------------
function httpGetJson(url) {
  return new Promise((resolve, reject) => {
    const req = http.get(url, (res) => {
      let data = ''
      res.on('data', (c) => (data += c))
      res.on('end', () => {
        try {
          const json = JSON.parse(data)
          resolve({ ok: res.statusCode === 200, statusCode: res.statusCode, json })
        } catch (e) {
          reject(new Error(`Invalid JSON from ${url}`))
        }
      })
    })
    req.on('error', reject)
    req.end()
  })
}

async function pollKernelOnce() {
  for (const port of PORT_CANDIDATES) {
    try {
      const { ok, json } = await httpGetJson(`http://127.0.0.1:${port}/status`)
      if (ok && json && json.status === 'online') {
        sendKernelStatus(true, port, `status=online`)
        return
      }
      // endpoint antwoordt maar status niet ok
      sendKernelStatus(false, port, `status endpoint reachable but not online`)
    } catch (_) {
      // try next port
    }
  }

  sendKernelStatus(false, null, `no response on ports: ${PORT_CANDIDATES.join(', ')}`)
}

function startPolling() {
  stopPolling()
  // kleine boot-wacht: kernel heeft tijd nodig om te starten
  setTimeout(() => {
    pollKernelOnce()
    pollTimer = setInterval(pollKernelOnce, 2000)
  }, 800)
}

function stopPolling() {
  if (pollTimer) {
    clearInterval(pollTimer)
    pollTimer = null
  }
}

// ----------------------------------------------------------------------
// Window
// ----------------------------------------------------------------------
function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    backgroundColor: '#0b0e14',
    webPreferences: {
      preload: path.join(__dirname, 'preload.cjs'),
      contextIsolation: true,
      nodeIntegration: false
    }
  })

  mainWindow.on('closed', () => {
    mainWindow = null
    stopPolling()
  })

  if (isDev) {
    mainWindow.loadURL('http://localhost:5173/')
    mainWindow.webContents.openDevTools({ mode: 'detach' })
  } else {
    mainWindow.loadFile(path.join(__dirname, '..', 'dist', 'index.html'))
  }
}

// ----------------------------------------------------------------------
// App lifecycle
// ----------------------------------------------------------------------
app.whenReady().then(() => {
  startKernel()
  createWindow()
  startPolling()
})

app.on('before-quit', () => {
  stopPolling()
  stopKernel()
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit()
})
