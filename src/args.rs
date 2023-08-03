use clap::{Parser, Subcommand};

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
    Save(DeckSaveAction),
}

#[derive(clap::Args)]
pub struct DeckSaveAction {
    pub path: String,
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
