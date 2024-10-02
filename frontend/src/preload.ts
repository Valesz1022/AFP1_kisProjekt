import { ipcRenderer, contextBridge } from 'electron';

const api = {
    load_reg_page: () => {
        ipcRenderer.send('load_reg_page')
    },

    load_login_page: () => {
        ipcRenderer.send('load_login_page')
    },

    load_main_page_admin: () => {
        ipcRenderer.send('load_main_page_admin')
    },

    load_main_page_user: () => {
        ipcRenderer.send('load_main_page_user')
    }
}

declare global {
    interface Window {
        api: typeof api
    }
}

contextBridge.exposeInMainWorld('api', api)
