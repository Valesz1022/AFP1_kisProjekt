import { ipcRenderer, contextBridge } from 'electron';

const api = {
    load_reg_page: () => {
        ipcRenderer.send('load_reg_page')
    },

    load_login_page: () => {
        ipcRenderer.send('load_login_page')
    }
}

declare global {
    interface Window {
        api: typeof api
    }
}

contextBridge.exposeInMainWorld('api', api)
