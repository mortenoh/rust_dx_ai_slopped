//! YAML command - YAML utilities.

use crate::cli::commands::yaml::{YamlArgs, YamlCommand};
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// Run the yaml command
pub fn run(args: YamlArgs) -> Result<()> {
    match args.command {
        YamlCommand::Format { input } => cmd_format(input),
        YamlCommand::Validate { input, quiet } => cmd_validate(input, quiet),
        YamlCommand::ToJson { input, pretty } => cmd_to_json(input, pretty),
        YamlCommand::FromJson { input } => cmd_from_json(input),
    }
}

fn read_input(input: Option<PathBuf>) -> Result<String> {
    match input {
        Some(path) if path.to_string_lossy() == "-" => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
        Some(path) => fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path.display())),
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn cmd_format(input: Option<PathBuf>) -> Result<()> {
    let content = read_input(input)?;

    // Parse and re-serialize to format (using serde_json::Value as intermediate)
    let value: serde_json::Value =
        serde_saphyr::from_str(&content).context("Failed to parse YAML")?;

    let output = serde_saphyr::to_string(&value).context("Failed to serialize YAML")?;
    print!("{}", output);
    Ok(())
}

fn cmd_validate(input: Option<PathBuf>, quiet: bool) -> Result<()> {
    let content = read_input(input)?;

    match serde_saphyr::from_str::<serde_json::Value>(&content) {
        Ok(_) => {
            if !quiet {
                println!("{}", "Valid YAML".green());
            }
            Ok(())
        }
        Err(e) => {
            if !quiet {
                eprintln!("{}: {}", "Invalid YAML".red(), e);
            }
            anyhow::bail!("Invalid YAML syntax")
        }
    }
}

fn cmd_to_json(input: Option<PathBuf>, pretty: bool) -> Result<()> {
    let content = read_input(input)?;

    // Parse YAML
    let value: serde_json::Value =
        serde_saphyr::from_str(&content).context("Failed to parse YAML")?;

    // Output as JSON
    let output = if pretty {
        serde_json::to_string_pretty(&value)?
    } else {
        serde_json::to_string(&value)?
    };

    println!("{}", output);
    Ok(())
}

fn cmd_from_json(input: Option<PathBuf>) -> Result<()> {
    let content = read_input(input)?;

    // Parse JSON
    let value: serde_json::Value =
        serde_json::from_str(&content).context("Failed to parse JSON")?;

    // Output as YAML
    let output = serde_saphyr::to_string(&value).context("Failed to serialize YAML")?;
    print!("{}", output);
    Ok(())
}
