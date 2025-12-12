const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('kernel', {
  getStatus: async () => {
    try {
      return await ipcRenderer.invoke('kernel:getStatus');
    } catch {
      return 'ipc-error';
    }
  }
});
