//! Markdown command - Markdown utilities.

use crate::cli::commands::markdown::{MarkdownArgs, MarkdownCommand};
use anyhow::{Context, Result};
use comrak::{markdown_to_html, Options};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// Run the markdown command
pub fn run(args: MarkdownArgs) -> Result<()> {
    match args.command {
        MarkdownCommand::Render { input } => cmd_render(input),
        MarkdownCommand::Toc { input, depth } => cmd_toc(input, depth),
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

fn cmd_render(input: Option<PathBuf>) -> Result<()> {
    let content = read_input(input)?;

    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;

    let html = markdown_to_html(&content, &options);
    print!("{}", html);
    Ok(())
}

fn cmd_toc(input: Option<PathBuf>, max_depth: u8) -> Result<()> {
    let content = read_input(input)?;

    // Simple heading extraction
    for line in content.lines() {
        if line.starts_with('#') {
            let level = line.chars().take_while(|c| *c == '#').count();
            if level <= max_depth as usize {
                let text = line.trim_start_matches('#').trim();
                let indent = "  ".repeat(level.saturating_sub(1));
                let slug = text
                    .to_lowercase()
                    .chars()
                    .map(|c| {
                        if c.is_alphanumeric() || c == '-' {
                            c
                        } else {
                            '-'
                        }
                    })
                    .collect::<String>()
                    .replace("--", "-")
                    .trim_matches('-')
                    .to_string();
                println!("{}- [{}](#{})", indent, text, slug);
            }
        }
    }
    Ok(())
}
