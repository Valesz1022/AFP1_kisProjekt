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
    },

    load_new_post: () => {
        ipcRenderer.send('load_new_post')
    }
}

const SERVER = 'http://127.0.0.1:3000'

declare global {
    interface Window {
        api: typeof api
    }
    let SERVER: String
    var globalUsername: string;
}

contextBridge.exposeInMainWorld('api', api)
contextBridge.exposeInMainWorld('SERVER', SERVER)
