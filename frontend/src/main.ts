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
})

ipcMain.on('load_login_page', () => {
    if (window) {
        window.loadFile(path.join(app.getAppPath(), 'html/index.html'));
    }
});
