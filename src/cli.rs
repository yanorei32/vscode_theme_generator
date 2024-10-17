use clap::{Parser, Subcommand};
use generate::GenerateArgs;

pub mod generate;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Generate(GenerateArgs),
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Generate(args) => Self::generate(args),
        }
    }
}
