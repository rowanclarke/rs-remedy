use crate::{tokens::Tokenizer, workspace::Workspace};

use anyhow::{anyhow, Error, Result};
use dashmap::DashMap;
use pest::error::LineColLocation;
use pest_meta::{optimizer::OptimizedRule, parse_and_optimize};
use pest_vm::Vm;
use serde::Deserialize;
use std::{fs::File, io::Read, path::PathBuf};
use tower_lsp::lsp_types::*;

#[derive(Deserialize, Debug)]
struct Configuration {
    profile: Vec<Profile>,
}

#[derive(Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
struct Profile {
    name: String,
    grammar: PathBuf,
    rule: String,
}

pub struct ProfileTokenizer {
    profiles: DashMap<PathBuf, Profile>,
    parsers: DashMap<PathBuf, Vec<OptimizedRule>>,
}

impl ProfileTokenizer {
    pub fn new() -> Self {
        Self {
            profiles: DashMap::new(),
            parsers: DashMap::new(),
        }
    }

    fn get_profile(&self, workspace: &Workspace, document: PathBuf) -> Result<Profile> {
        if !self.profiles.contains_key(&document) {
            let config = workspace.get_meta_deserialized::<Configuration, _>("config.toml")?;
            self.profiles
                .insert(document.clone(), config.profile[0].clone());
        }
        Ok(self.profiles.get(&document).unwrap().clone())
    }

    fn get_parser(&self, grammar: PathBuf) -> Result<Vec<OptimizedRule>> {
        if !self.parsers.contains_key(&grammar) {
            let text = self.get_text(grammar.clone())?;
            let (_, rules) =
                parse_and_optimize(&text).map_err(|err| anyhow!(err[0].to_string()))?;
            self.parsers.insert(grammar.clone(), rules);
        }
        Ok(self.parsers.get(&grammar).unwrap().clone())
    }
}

impl Tokenizer for ProfileTokenizer {
    type Output = ();
    type Error = Error;

    fn get_text(&self, document: PathBuf) -> Result<String> {
        let mut file = File::open(document)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        Ok(text)
    }

    fn parse_text(
        &self,
        workspace: PathBuf,
        document: PathBuf,
        text: String,
    ) -> Result<Result<Self::Output, Vec<Diagnostic>>, Self::Error> {
        let workspace = Workspace::new(workspace);
        let profile = self.get_profile(&workspace, document)?;
        let grammar = workspace.get_meta(&profile.grammar);
        let vm = Vm::new(self.get_parser(grammar)?);
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
