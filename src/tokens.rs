use std::{fmt::Display, fs::File, path::PathBuf};

use tower_lsp::lsp_types::*;

pub trait Tokenizer: Sync + Send + 'static {
    type Tokens: Send;
    type Error: Display + Sync + Send;

    fn get_text(document: PathBuf) -> Result<String, Self::Error>;

    fn parse_document(
        workspace: PathBuf,
        document: PathBuf,
    ) -> Result<Result<Self::Tokens, Vec<Diagnostic>>, Self::Error> {
        Self::parse_text(workspace, Self::get_text(document)?)
    }

    fn parse_text(
        workspace: PathBuf,
        document: String,
    ) -> Result<Result<Self::Tokens, Vec<Diagnostic>>, Self::Error>;
}
