"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const path = require("path");
const vscode_1 = require("vscode");
const node_1 = require("vscode-languageclient/node");
let client;
function activate(context) {
    // 指向你编译好的 rust 二进制文件路径
    // 如果你在远程开发，确保相对路径正确。这里的 .. 指向 workspace 根目录
    const command = context.asAbsolutePath(path.join('..', 'server', 'target', 'release', 'bsv-ls'));
    const run = {
        command,
        transport: node_1.TransportKind.stdio,
    };
    const serverOptions = {
        run,
        debug: run
    };
    // 控制客户端的选项
    const clientOptions = {
        // 注册到 bs 语言
        documentSelector: [{ scheme: 'file', language: 'bsv' }],
        synchronize: {
            // 当工作区内的文件改变时通知 server
            fileEvents: vscode_1.workspace.createFileSystemWatcher('**/*.bsv')
        }
    };
    // 创建并启动客户端
    client = new node_1.LanguageClient('bsvLanguageServer', 'BSV Language Server', serverOptions, clientOptions);
    client.start();
}
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
//# sourceMappingURL=extension.js.map