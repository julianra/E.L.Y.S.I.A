/**
 * ELYSIA Kernel UI
 * Preload script
 * - Exposes kernel status to renderer
 */

const { contextBridge, ipcRenderer } = require('electron')

contextBridge.exposeInMainWorld('elysia', {
  onKernelStatus: (callback) => {
    ipcRenderer.on('kernel-status', (_, status) => callback(status))
  }
})
