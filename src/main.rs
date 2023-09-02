mod tokens;

use std::{
    future::{ready, Future},
    marker::PhantomData,
    pin::Pin,
};
use tokens::{ProfileTokenizer, Tokenizer};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend<T, C> {
    client: Client,
    cache: C,
    phantom: PhantomData<T>,
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

impl<T, C> Backend<T, C> {
    async fn send_root_diagnostic(&self, uri: Url, message: String) {
        self.client
            .publish_diagnostics(
                uri,
                vec![Diagnostic::new_with_code_number(
                    START,
                    DiagnosticSeverity::ERROR,
                    0,
                    None,
                    message,
                )],
                None,
            )
            .await;
    }

    async fn diagnose_workspace_folders(&self, uri: Url) -> Result<Option<WorkspaceFolder>> {
        match self.client.workspace_folders().await {
            Ok(Some(vec)) if vec.len() > 0 => Ok(Some(vec[0].clone())),
            Ok(_) => {
                self.send_root_diagnostic(uri, "Not in workspace".to_string())
                    .await;
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}

#[tower_lsp::async_trait]
impl<T: Tokenizer, C: Sync + Send + 'static> LanguageServer for Backend<T, C> {
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
        let uri = params.text_document.uri;
        if let Some(workspace) = self.diagnose_workspace_folders(uri.clone()).await.unwrap() {
            T::parse(
                workspace
                    .uri
                    .to_file_path()
                    .expect("Workspace is not in file scheme"),
                params.content_changes[0].text.clone(),
            )
            .map_or_else::<Pin<Box<dyn Future<Output = ()> + Send>>, _, _>(
                |err| Box::pin(self.send_root_diagnostic(uri, format!("{:#?}", err))),
                |_| Box::pin(ready(())),
            )
            .await;
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::<ProfileTokenizer, Option<()>> {
        client,
        cache: None,
        phantom: PhantomData,
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
