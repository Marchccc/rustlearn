var rust = require('../native');
// module.exports = rust; // 导出模块到本js

const electron = require('electron');
const {app} = electron;
const {BrowserWindow} = electron;
const path = require('path');
const url = require('url');

let mainWindow;

function createWindow() {

    // 创建浏览器窗口
    mainWindow = new BrowserWindow({
        width: 600,
        height: 600,
        webPreferences: {
            nodeIntegration: true,
            // nodeIntegrationInWorker: true,
            // webSecurity: false,
            // allowRunningInsecureContent: true,
            // experimentalFeatures: true,
        }
    });

    mainWindow.loadURL(
        url.format({
          pathname: path.join(__dirname, 'index.html'),
          protocol: 'file:',
          slashes: true
        })
    );

    // mainWindow.webContents.on('did-finish-load', function(){
    //     mainWindow.webContents.send('data', '123456');
    // });
    
    mainWindow.on('closed', () => {
        mainWindow = null;
    });

    // 打开开发者工具
    mainWindow.webContents.openDevTools();

    // console.log(rust.threadCount());
}

// Electron会在初始化完成后调用这个方法
app.on('ready', createWindow);

//当所有窗口都被关闭后退出
app.on('window-all-closed', () => {

    // 在 macOS 上，除非用户用 Cmd + Q 确定地退出，
    // 否则绝大部分应用及其菜单栏会保持激活。
    if (process.platform !== 'darwin') {
        app.quit();
    }
})

app.on('active', () => {

    // 在macOS上，当单击dock图标并且没有其他窗口打开时，
    // 通常在应用程序中重新创建一个窗口。
    if (mainWindow === null) {
        createWindow();
    }
})

// console.log(addon.hello());

