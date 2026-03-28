import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
    Executable
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    // 指向你编译好的 rust 二进制文件路径
    // 如果你在远程开发，确保相对路径正确。这里的 .. 指向 workspace 根目录
    const command = context.asAbsolutePath(
        path.join('..', 'server', 'target', 'release', 'bsv-ls')
    );

    const run: Executable = {
        command,
        transport: TransportKind.stdio,
    };

    const serverOptions: ServerOptions = {
        run,
        debug: run
    };

    // 控制客户端的选项
    const clientOptions: LanguageClientOptions = {
        // 注册到 bs 语言
        documentSelector: [{ scheme: 'file', language: 'bsv' }],
        synchronize: {
            // 当工作区内的文件改变时通知 server
            fileEvents: workspace.createFileSystemWatcher('**/*.bsv')
        }
    };

    // 创建并启动客户端
    client = new LanguageClient(
        'bsvLanguageServer',
        'BSV Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}