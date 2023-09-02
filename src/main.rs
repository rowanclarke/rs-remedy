use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

const START: Range = Range {
    start: Position {
        line: 0,
        character: 0,
    },
    end: Position {
        line: 0,
        character: 1,
    },
};

impl Backend {
    async fn diagnose_workspace_folders(&self, uri: Url) -> Result<Option<WorkspaceFolder>> {
        match self.client.workspace_folders().await {
            Ok(Some(vec)) if vec.len() > 0 => Ok(Some(vec[0].clone())),
            Ok(_) => {
                self.client
                    .publish_diagnostics(
                        uri,
                        vec![Diagnostic::new_with_code_number(
                            START,
                            DiagnosticSeverity::ERROR,
                            0,
                            None,
                            "Not in workspace".to_string(),
                        )],
                        None,
                    )
                    .await;
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        change: Some(TextDocumentSyncKind::FULL),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(workspace) = self
            .diagnose_workspace_folders(params.text_document.uri)
            .await
            .unwrap()
        {}
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
