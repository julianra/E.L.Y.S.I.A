const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('path');
const { spawn } = require('child_process');

let kernelProcess = null;
let kernelStatus = 'stopped';

function getKernelPath() {
  if (app.isPackaged) {
    return path.join(
      process.resourcesPath,
      'app.asar.unpacked',
      'bin',
      'kernel.exe'
    );
  }
  return path.join(__dirname, '../bin/kernel.exe');
}

function startKernel() {
  const kernelPath = getKernelPath();

  console.log('[KERNEL] Starting:', kernelPath);
  kernelStatus = 'starting';

  try {
    kernelProcess = spawn(kernelPath, [], {
      windowsHide: true,
      cwd: path.dirname(kernelPath)
    });

    kernelProcess.stdout?.on('data', d => {
      console.log('[KERNEL]', d.toString());
    });

    kernelProcess.stderr?.on('data', d => {
      console.error('[KERNEL ERR]', d.toString());
    });

    kernelProcess.on('spawn', () => {
      console.log('[KERNEL] Spawned');
      kernelStatus = 'running';
    });

    kernelProcess.on('exit', (code) => {
      console.log('[KERNEL] Exited with code', code);
      kernelStatus = 'stopped';
      kernelProcess = null;
    });

    kernelProcess.on('error', (err) => {
      console.error('[KERNEL] Spawn error:', err);
      kernelStatus = 'error';
    });

  } catch (e) {
    console.error('[KERNEL] Fatal spawn error:', e);
    kernelStatus = 'error';
  }
}

function createWindow() {
  const win = new BrowserWindow({
    width: 1000,
    height: 700,
    autoHideMenuBar: true,
    webPreferences: {
      preload: path.join(__dirname, 'preload.cjs'),
      contextIsolation: true
    }
  });

  win.loadFile(path.join(__dirname, '../dist/index.html'));
  win.webContents.openDevTools();
}

ipcMain.handle('kernel:getStatus', () => kernelStatus);

app.whenReady().then(() => {
  startKernel();
  createWindow();
});

app.on('window-all-closed', () => {
  if (kernelProcess) kernelProcess.kill();
  if (process.platform !== 'darwin') app.quit();
});
