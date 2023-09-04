use crate::{tokens::Tokenizer, workspace::Workspace};

use anyhow::{anyhow, Error};
use pest::error::LineColLocation;
use pest_meta::parse_and_optimize;
use pest_vm::Vm;
use serde::Deserialize;
use std::{fs::File, io::Read, path::PathBuf};
use tower_lsp::lsp_types::*;

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

    fn get_text(document: PathBuf) -> Result<String, Self::Error> {
        let mut file = File::open(document)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        Ok(text)
    }

    fn parse_text(workspace: PathBuf, text: String) -> Result<Result<(), Vec<Diagnostic>>, Error> {
        let workspace = Workspace::new(workspace);
        let config = workspace.get_meta_deserialized::<Configuration, _>("config.toml")?;
        let profile = &config.profile[0];

        let grammar = Self::get_text(workspace.get_meta(&profile.grammar))?;
        let (_, rules) = parse_and_optimize(&grammar).map_err(|err| anyhow!(err[0].to_string()))?;
        let vm = Vm::new(rules);

        let pairs = vm.parse(&profile.rule, &text);
        Ok(match pairs {
            Ok(_) => Ok(()),
            Err(err) => Err(vec![Diagnostic {
                range: match err.line_col {
                    LineColLocation::Pos((a, b)) => {
                        let (a, b) = (a as u32 - 1, b as u32 - 1);
                        Range::new(Position::new(a, b), Position::new(a, b + 1))
                    }
                    LineColLocation::Span((a, b), (c, d)) => {
                        let (a, b, c, d) = (a as u32 - 1, b as u32 - 1, c as u32 - 1, d as u32 - 1);
                        Range::new(Position::new(a, b), Position::new(c, d))
                    }
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: err.variant.to_string(),
                ..Default::default()
            }]),
        })
    }
}
