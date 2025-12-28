//! Template command arguments.

use clap::{Args, Subcommand};
use std::path::PathBuf;

/// Template utilities (Jinja2-style templating)
#[derive(Args, Debug)]
pub struct TemplateArgs {
    #[command(subcommand)]
    pub command: TemplateCommand,
}

/// Template subcommands
#[derive(Subcommand, Debug)]
pub enum TemplateCommand {
    /// Render a template with data
    Render {
        /// Template file
        template: PathBuf,

        /// JSON data file (or use --json for inline)
        #[arg(short, long)]
        data: Option<PathBuf>,

        /// Inline JSON data
        #[arg(long)]
        json: Option<String>,
    },

    /// Validate template syntax
    Validate {
        /// Template file
        template: PathBuf,
    },
}
