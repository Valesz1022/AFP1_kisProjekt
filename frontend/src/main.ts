import { app, BrowserWindow } from 'electron';
import path from 'path';

let window: BrowserWindow;

app.on('ready', () => {
    window = new BrowserWindow({
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
