use crate::commands::Command;
use crate::extractor::ArkenfoxExtractor;
use anyhow::{Context, Result};
use clap::Args;
use fancy_regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Args)]
pub struct GenerateCommand {}

#[derive(Deserialize)]
struct GitRef {
    #[serde(rename = "ref")]
    git_ref: String,
}

impl Command for GenerateCommand {
    async fn execute(&self) -> Result<()> {
        let client = reqwest::Client::new();
        let data_dir = PathBuf::from("data");
        let output_file = data_dir.join("default.nix");

        fs::create_dir_all(&data_dir)
            .with_context(|| format!("Failed to create data directory: {}", data_dir.display()))?;

        let response = client
            .get("https://api.github.com/repos/arkenfox/user.js/git/refs/tags")
            .header("User-Agent", "arkenfox-nix-cli")
            .send()
            .await
            .context("Failed to fetch GitHub API tags")?;

        let refs: Vec<GitRef> = response
            .json()
            .await
            .context("Failed to parse GitHub API response")?;

        let mut versions = Vec::new();
        versions.push("master".to_string());

        let version_regex =
            Regex::new(r"^(\d+\.\d+)$").context("Failed to compile version regex")?;

        for git_ref in refs {
            if git_ref.git_ref.starts_with("refs/tags/") {
                let tag = git_ref.git_ref.strip_prefix("refs/tags/").unwrap();
                if let Ok(Some(captures)) = version_regex.captures(tag)
                    && let Some(version_match) = captures.get(1)
                {
                    let version = version_match.as_str();
                    if let Ok(version_num) = version.parse::<f64>()
                        && version_num >= 91.0
                    {
                        versions.push(version.to_string());
                    }
                }
            }
        }

        let mut default_nix = String::from("{\n");

        for version in versions {
            match self.generate_version(&client, &version, &data_dir).await {
                Ok(content) => {
                    default_nix.push_str(&content);
                }
                Err(e) => {
                    eprintln!("Failed to generate version {}: {}", version, e);
                }
            }
        }

        default_nix.push_str("}\n");

        fs::write(&output_file, default_nix)
            .with_context(|| format!("Failed to write output file: {}", output_file.display()))?;

        println!("Successfully generated configurations.");

        Ok(())
    }
}

impl GenerateCommand {
    async fn generate_version(
        &self,
        client: &reqwest::Client,
        version: &str,
        data_dir: &Path,
    ) -> Result<String> {
        if version == "master" {
            println!("Generating {}", version)
        } else {
            println!("Generating v{}", version);
        }

        let url = format!(
            "https://raw.githubusercontent.com/arkenfox/user.js/{}/user.js",
            version
        );

        let response = client
            .get(&url)
            .header("User-Agent", "arkenfox-nix-cli")
            .send()
            .await
            .with_context(|| format!("Failed to fetch {}", url))?;

        let content = response
            .text()
            .await
            .with_context(|| format!("Failed to read content from {}", url))?;

        let mut extractor = ArkenfoxExtractor::new().context("Failed to create extractor")?;
        let result = extractor.extract(&content)?;

        let json_content =
            serde_json::to_string_pretty(&result).context("Failed to serialize to JSON")?;

        let json_file_path = data_dir.join(format!("{}.json", version));
        fs::write(&json_file_path, json_content)
            .with_context(|| format!("Failed to write {}", json_file_path.display()))?;

        Ok(format!(
            "  \"{}\" = builtins.fromJSON (builtins.readFile ./{}.json);\n",
            version, version
        ))
    }
}
