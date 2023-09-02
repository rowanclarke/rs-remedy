use std::{fmt::Debug, path::PathBuf};

pub trait Tokenizer: Sync + Send + 'static {
    type Tokens: Send;
    type Error: Debug + Send;

    fn parse(workspace: PathBuf, document: String) -> Result<Self::Tokens, Self::Error>;
}

pub struct ProfileTokenizer;

impl Tokenizer for ProfileTokenizer {
    type Tokens = ();
    type Error = String;

    fn parse(workspace: PathBuf, document: String) -> Result<Self::Tokens, Self::Error> {
        Err("Testing".to_string())
    }
}
