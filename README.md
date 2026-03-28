# BSV Language Server (BSV-LS) - MVP

这是一个基于 **Rust (`tower-lsp`)** 和 **Tree-sitter** 实现的简易 Bluespec SystemVerilog (BSV) 语言服务器。它通过 VS Code 插件提供基础的语言支持功能。

## 🚀 当前支持的功能 (MVP 阶段)

* **文档同步**：支持实时跟踪 `.bsv` 文件的打开和内容修改。
* **跳转到定义 (Go to Definition)**：支持 `module` 和 `interface` 的基础符号跳转（目前基于字符串匹配与基础解析模拟，后续对接 Tree-sitter）。
* **日志系统**：在 VS Code 的输出面板中实时显示 LSP 交互日志。

---

## 🏗️ 实验环境准备

* **硬件**：MacBook (Client) + x86 Linux Server (Remote Host)
* **连接方式**：VS Code **Remote - SSH** 插件。
* **后端环境**：
  * Rust (1.65+)
  * Cargo
* **前端环境**：
  * Node.js (v16+)
  * npm

---

## 📂 项目结构说明

```text
bsv-lsp-workspace/
├── client/                 # TypeScript 实现的 VS Code 插件
│   ├── src/extension.ts    # 插件入口，负责启动并连接 Rust Server
│   ├── package.json        # 插件定义与配置
│   └── tsconfig.json       # TypeScript 编译配置
└── server/                 # Rust 实现的 LSP 服务端
    ├── src/main.rs         # LSP 核心逻辑 (tower-lsp)
    └── Cargo.toml          # Rust 依赖管理
```

---

## 🛠️ 编译与安装步骤

### 1. 编译 Rust 服务端

在远程服务器上执行：

```bash
cd bsv-lsp-workspace/server
cargo build --release
```

编译完成后，二进制文件位于 `server/target/release/bsv-ls`。

### 2. 编译 VS Code 插件

在远程服务器上执行：

```bash
cd ../client
npm install
npm run compile
```

---

## 🔍 调试与验证

### 1. 启动开发模式

1. 在远程 VS Code 中，打开 `client` 文件夹。
2. 按下 `F5` 或点击 **“运行和调试”** 面板中的 **"Run Extension"**。
3. 这会弹出一个新的 VS Code 窗口（[Extension Development Host]）。

### 2. 功能测试

在弹出的新窗口中，创建一个 `test.bsv` 文件：

```verilog
// 定义一个模块
module mkCalculator();
    // ...
endmodule

// 调用模块
module mkMain();
    mkCalculator calc(); // 在这里尝试对 mkCalculator 按 F12
endmodule
```

* **验证跳转**：将光标放在 `mkMain` 里的 `mkCalculator` 上，按下 `F12`。如果配置正确，光标会自动跳回顶部的模块定义处。
* **查看日志**：在 VS Code 底部面板切换到 **Output (输出)** 标签页，在下拉菜单中选择 **"BSV Language Server"**，你可以看到 LSP 握手和跳转请求的详细日志。

---

## 📈 后续路线图 (Roadmap)

1. **Tree-sitter 集成**：
   * 引入 `tree-sitter-bsv` 语法解析库。
   * 替换目前的简单字符串查找逻辑，实现基于 AST 的精确跳转。
2. **符号搜索 (Document Symbol)**：
   * 实现 `textDocument/documentSymbol` 以支持大纲视图 (Outline)。
3. **语法高亮**：
   * 通过 Tree-sitter 提供比正则更精准的语义高亮。
4. **bsc Testsuite 验证**：
   * 从 [bsc repository](https://github.com/B-Lang-org/bsc/tree/main/testsuite) 引入测试用例。
   * 编写自动化脚本，确保 LSP 能正确解析官方所有的测试样本。

---

## 📝 故障排除

* **报错 `Cannot find type definition file for 'node'`**：
  在 `client` 目录下运行 `npm i --save-dev @types/node`。
* **插件启动失败，提示找不到 server 路径**：
  请检查 `client/src/extension.ts` 中的二进制文件路径是否与 `server/target/release/bsv-ls` 匹配。
* **跳转无效**：
  确保当前文件的 Language Mode 已识别为 `bsv`（查看右下角状态栏）。
