use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Parser, Point};

// 假设你有 tree_sitter_bsv，这里做一个 mock，实际应用中替换为真正的 BSV language
// extern "C" { fn tree_sitter_bsv() -> tree_sitter::Language; }

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<Url, String>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // 声明支持文档同步（全量同步，简单起见）
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                // 声明支持“跳转到定义”
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "BSV Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    // 当文件打开时，将内容存入 map
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, format!("File opened: {}", params.text_document.uri))
            .await;
        self.document_map
            .insert(params.text_document.uri, params.text_document.text);
    }

    // 当文件修改时，更新 map 中的内容
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().next() {
            self.document_map
                .insert(params.text_document.uri, change.text);
        }
    }

    // 核心逻辑：处理跳转到定义请求
    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let document_text = match self.document_map.get(&uri) {
            Some(text) => text.clone(),
            None => return Ok(None),
        };

        // --- Tree-sitter 解析流程 ---
        let mut parser = Parser::new();
        // 实际开发中解开这行注释并设置真正的 BSV Language
        // parser.set_language(unsafe { tree_sitter_bsv() }).unwrap();
        
        // 为了演示，这里我们用一个简易的正则/字符串查找机制模拟 Tree-sitter 查找到定义节点的行为
        // 在实际开发中，你会：
        // 1. parser.parse(&document_text, None) 得到 AST Tree
        // 2. tree.root_node().descendant_for_point_range() 找到光标处的标识符 Node
        // 3. 向上遍历作用域，查找该标识符的声明位置 (Declaration Node)
        
        let lines: Vec<&str> = document_text.lines().collect();
        if position.line as usize >= lines.len() {
            return Ok(None);
        }
        
        // 提取光标所在行的单词 (极简模拟)
        let line = lines[position.line as usize];
        let char_idx = position.character as usize;
        let word = extract_word_at_cursor(line, char_idx);

        if word.is_empty() {
            return Ok(None);
        }

        self.client
            .log_message(MessageType::INFO, format!("Looking for definition of: {}", word))
            .await;

        // 模拟：在整个文档中寻找 `module <word>` 或 `interface <word>` 或 `<type> <word> =`
        // 实际应用中这里是基于 Tree-sitter 的 Scope 查询
        for (i, l) in lines.iter().enumerate() {
            if l.contains(&format!("module {}", word)) || l.contains(&format!("interface {}", word)) {
                let start_char = l.find(&word).unwrap_or(0);
                return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position::new(i as u32, start_char as u32),
                        end: Position::new(i as u32, (start_char + word.len()) as u32),
                    },
                })));
            }
        }

        Ok(None)
    }
}

// 辅助函数：提取光标处的单词
fn extract_word_at_cursor(line: &str, cursor_idx: usize) -> String {
    let mut start = cursor_idx;
    let mut end = cursor_idx;
    let chars: Vec<char> = line.chars().collect();

    if chars.is_empty() || cursor_idx >= chars.len() { return String::new(); }

    while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
        start -= 1;
    }
    while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
        end += 1;
    }
    chars[start..end].iter().collect()
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_map: DashMap::new(),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}