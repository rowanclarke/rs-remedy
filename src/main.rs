mod profile;
mod tokens;
mod workspace;

use dashmap::DashMap;
use profile::ProfileTokenizer;
use std::{marker::PhantomData, sync::Mutex};
use tokens::Tokenizer;
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend<T, C> {
    client: Client,
    cache: C,
    errors: DashMap<Url, Vec<Diagnostic>>,
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
    async fn workspace_folder(&self) -> Result<Option<WorkspaceFolder>> {
        match self.client.workspace_folders().await {
            Ok(Some(vec)) if vec.len() > 0 => Ok(Some(vec[0].clone())),
            Ok(_) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn publish_diagnostics(&self, uri: Url) {
        let diagnostics = self.errors.get(&uri).map_or(vec![], |r| r.clone());
        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
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
                        will_save: Some(true),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, params: InitializedParams) {
        self.client
            .show_message(MessageType::ERROR, "Initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(workspace) = self
            .workspace_folder()
            .await
            .ok()
            .and_then(|w| w)
            .and_then(|w| w.uri.to_file_path().ok())
        {
            match T::parse_text(workspace, params.content_changes[0].text.clone()) {
                Ok(Ok(_)) => drop(self.errors.remove(&uri)),
                Ok(Err(diagnostics)) => drop(self.errors.insert(uri.clone(), diagnostics)),
                Err(err) => drop(self.errors.insert(
                    uri.clone(),
                    vec![Diagnostic {
                        range: START,
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: err.to_string(),
                        ..Default::default()
                    }],
                )),
            };
        }
        self.publish_diagnostics(uri).await;
    }

    async fn will_save(&self, params: WillSaveTextDocumentParams) {}
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::<ProfileTokenizer, Option<()>> {
        client,
        cache: None,
        errors: DashMap::new(),
        phantom: PhantomData,
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
