use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Deck(DeckAction),
    Session(SessionAction),
}

#[derive(clap::Args)]
pub struct DeckAction {
    #[command(subcommand)]
    pub command: DeckCommand,
}

#[derive(Subcommand)]
pub enum DeckCommand {
    Add(DeckAddAction),
}

#[derive(clap::Args)]
pub struct DeckAddAction {
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,
}

#[derive(clap::Args)]
pub struct SessionAction {
    #[command(subcommand)]
    pub command: SessionCommand,
}

#[derive(Subcommand)]
pub enum SessionCommand {
    Initialize,
    Learn,
}
