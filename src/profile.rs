use crate::{tokens::Tokenizer, workspace::Workspace};

use anyhow::{anyhow, Error};
use pest_meta::parse_and_optimize;
use pest_vm::Vm;
use serde::Deserialize;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Deserialize, Debug)]
struct Configuration {
    profile: Vec<Profile>,
}

#[derive(Deserialize, Debug)]
struct Profile {
    grammar: PathBuf,
    rule: String,
}

pub struct ProfileTokenizer;

impl Tokenizer for ProfileTokenizer {
    type Tokens = ();
    type Error = Error;

    fn parse(workspace: std::path::PathBuf, document: String) -> Result<Self::Tokens, Self::Error> {
        let workspace = Workspace::new(workspace);
        let config = workspace.get_meta_deserialized::<Configuration, _>("config.toml")?;
        let profile = &config.profile[0];
        let mut grammar = File::open(workspace.get_meta(&profile.grammar))?;
        let mut buffer = String::new();
        grammar.read_to_string(&mut buffer).unwrap();

        let (_, rules) = parse_and_optimize(&buffer).map_err(|vec| vec[0].clone())?;
        let vm = Vm::new(rules);

        let pairs = vm
            .parse(&profile.rule, &document)
            .map_err(|err| anyhow!("{}", err.variant))?;
        Ok(())
    }
}
