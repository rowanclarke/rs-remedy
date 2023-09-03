mod profile;
mod tokens;
mod workspace;

use profile::ProfileTokenizer;
use std::{
    future::{ready, Future},
    marker::PhantomData,
    path::PathBuf,
    pin::Pin,
};
use tokens::Tokenizer;
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

    async fn workspace_folder(&self) -> Result<Option<WorkspaceFolder>> {
        match self.client.workspace_folders().await {
            Ok(Some(vec)) if vec.len() > 0 => Ok(Some(vec[0].clone())),
            Ok(_) => Ok(None),
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
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                // diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                //     DiagnosticOptions {
                //         identifier: None,
                //         inter_file_dependencies: false,
                //         workspace_diagnostics: false,
                //         work_done_progress_options: WorkDoneProgressOptions {
                //             work_done_progress: Some(false),
                //         },
                //     },
                // )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    // async fn diagnostic(
    //     &self,
    //     params: DocumentDiagnosticParams,
    // ) -> Result<DocumentDiagnosticReportResult> {
    //     todo!()
    // }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(workspace) = self
            .workspace_folder()
            .await
            .ok()
            .and_then(|w| w)
            .and_then(|w| w.uri.to_file_path().ok())
        {
            match T::parse(workspace, params.text.expect("include text").clone()) {
                Ok(_) => (),
                Err(err) => self.send_root_diagnostic(uri, err.to_string()).await,
            };
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
