use std::{fmt::Display, fs::File, path::PathBuf};

use tower_lsp::lsp_types::*;

pub trait Tokens: Send {}

impl Tokens for () {}

pub trait Tokenizer: Sync + Send + 'static {
    type Output: Tokens;
    type Error: Display + Sync + Send;

    fn get_text(&self, document: PathBuf) -> Result<String, Self::Error>;

    fn parse_document(
        &self,
        workspace: PathBuf,
        document: PathBuf,
    ) -> Result<Result<Self::Output, Vec<Diagnostic>>, Self::Error> {
        self.parse_text(workspace, document.clone(), self.get_text(document)?)
    }

    fn parse_text(
        &self,
        workspace: PathBuf,
        document: PathBuf,
        text: String,
    ) -> Result<Result<Self::Output, Vec<Diagnostic>>, Self::Error>;
}
