mod commands;
mod extractor;

use anyhow::Result;
use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(name = "arkenfox-nix")]
#[command(about = "CLI for arkenfox-nix")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.execute().await
}
