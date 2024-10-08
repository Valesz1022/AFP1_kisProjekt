import { app, BrowserWindow, ipcMain } from 'electron';
import path from 'path';

let window: BrowserWindow;

app.on('ready', () => {
    window = new BrowserWindow({
        webPreferences: {
            preload: path.join(app.getAppPath(), 'dist/preload.js')
        },
        autoHideMenuBar: true
    });
    window.maximize();
    window.loadFile(path.join(app.getAppPath(), 'html/index.html'));
    window.webContents.openDevTools();
});

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit();
    }
});

ipcMain.on('load_reg_page', () => {
    if (window) {
        window.loadFile(path.join(app.getAppPath(), 'html/regisztracio.html'));
    }
    window.webContents.openDevTools();
})

ipcMain.on('load_login_page', () => {
    if (window) {
        window.loadFile(path.join(app.getAppPath(), 'html/index.html'));
    }
    window.webContents.openDevTools();
});

ipcMain.on('load_main_page_admin', () => {
    if (window) {
        window.loadFile(path.join(app.getAppPath(), 'html/main_page_admin.html'));
    }
    window.webContents.openDevTools();
    console.log(globalThis.globalUsername);
})

ipcMain.on('load_main_page_user', () => {
    if (window) {
        window.loadFile(path.join(app.getAppPath(), 'html/main_page_user.html'));
    }
    window.webContents.openDevTools();
    console.log(globalThis.globalUsername);
})

ipcMain.on('load_new_post', () => {
    if (window) {
        window.loadFile(path.join(app.getAppPath(), 'html/new_post.html'));
    }
    window.webContents.openDevTools();
    console.log(globalThis.globalUsername);
})