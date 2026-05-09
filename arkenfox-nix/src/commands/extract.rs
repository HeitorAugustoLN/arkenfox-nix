use crate::commands::Command;
use crate::extractor::ArkenfoxExtractor;
use anyhow::{Context, Result};
use clap::Args;
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct ExtractCommand {
    /// Path to the input user.js file
    pub file_path: PathBuf,
}

impl Command for ExtractCommand {
    async fn execute(&self) -> Result<()> {
        let content = fs::read_to_string(&self.file_path)
            .with_context(|| format!("Failed to read file: {}", self.file_path.display()))?;

        let mut extractor = ArkenfoxExtractor::new().context("Failed to initialize extractor")?;
        let result = extractor.extract(&content)?;

        let json_output =
            serde_json::to_string_pretty(&result).context("Failed to serialize result to JSON")?;

        println!("{}", json_output);
        Ok(())
    }
}
