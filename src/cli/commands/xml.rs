//! XML command arguments.

use clap::{Args, Subcommand};
use std::path::PathBuf;

/// XML utilities (format, validate, convert)
#[derive(Args, Debug)]
pub struct XmlArgs {
    #[command(subcommand)]
    pub command: XmlCommand,
}

/// XML subcommands
#[derive(Subcommand, Debug)]
pub enum XmlCommand {
    /// Pretty-print XML
    Format {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Indentation (spaces)
        #[arg(short, long, default_value = "2")]
        indent: usize,
    },

    /// Validate XML syntax
    Validate {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Quiet mode (exit code only)
        #[arg(short, long)]
        quiet: bool,
    },

    /// Convert XML to JSON
    #[command(name = "to-json")]
    ToJson {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Pretty-print output
        #[arg(short, long)]
        pretty: bool,
    },
}
