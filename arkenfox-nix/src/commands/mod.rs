mod extract;
mod generate;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Extract preferences from user.js files
    Extract(extract::ExtractCommand),
    /// Generate default.nix from arkenfox versions
    Generate(generate::GenerateCommand),
}

impl Commands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            Commands::Extract(cmd) => cmd.execute().await,
            Commands::Generate(cmd) => cmd.execute().await,
        }
    }
}

pub trait Command {
    async fn execute(&self) -> Result<()>;
}
