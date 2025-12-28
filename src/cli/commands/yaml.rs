//! YAML command arguments.

use clap::{Args, Subcommand};
use std::path::PathBuf;

/// YAML utilities (format, convert, validate)
#[derive(Args, Debug)]
pub struct YamlArgs {
    #[command(subcommand)]
    pub command: YamlCommand,
}

/// YAML subcommands
#[derive(Subcommand, Debug)]
pub enum YamlCommand {
    /// Pretty-print YAML
    Format {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,
    },

    /// Validate YAML syntax
    Validate {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Quiet mode (exit code only)
        #[arg(short, long)]
        quiet: bool,
    },

    /// Convert YAML to JSON
    #[command(name = "to-json")]
    ToJson {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Pretty-print output
        #[arg(short, long)]
        pretty: bool,
    },

    /// Convert JSON to YAML
    #[command(name = "from-json")]
    FromJson {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,
    },
}
