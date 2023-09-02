use std::{fmt::Display, path::PathBuf};

pub trait Tokenizer: Sync + Send + 'static {
    type Tokens: Send;
    type Error: Display + Sync + Send;

    fn parse(workspace: PathBuf, document: String) -> Result<Self::Tokens, Self::Error>;
}
