//! Markdown command arguments.

use clap::{Args, Subcommand};
use std::path::PathBuf;

/// Markdown utilities
#[derive(Args, Debug)]
pub struct MarkdownArgs {
    #[command(subcommand)]
    pub command: MarkdownCommand,
}

/// Markdown subcommands
#[derive(Subcommand, Debug)]
pub enum MarkdownCommand {
    /// Render markdown to HTML
    Render {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,
    },

    /// Extract table of contents
    Toc {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Maximum heading depth (1-6)
        #[arg(short, long, default_value = "3")]
        depth: u8,
    },
}
