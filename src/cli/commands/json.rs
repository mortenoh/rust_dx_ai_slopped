//! JSON command arguments.

use clap::{Args, Subcommand};
use std::path::PathBuf;

/// Format and validate JSON
#[derive(Args, Debug)]
pub struct JsonArgs {
    #[command(subcommand)]
    pub command: JsonCommand,
}

/// JSON subcommands
#[derive(Subcommand, Debug)]
pub enum JsonCommand {
    /// Pretty-print JSON
    #[command(visible_alias = "pp")]
    Format {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Indentation (spaces)
        #[arg(short, long, default_value = "2")]
        indent: usize,

        /// Use tabs instead of spaces
        #[arg(long)]
        tabs: bool,

        /// Sort object keys
        #[arg(short, long)]
        sort_keys: bool,

        /// Compact output (no whitespace)
        #[arg(short, long)]
        compact: bool,
    },

    /// Validate JSON syntax
    Validate {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Quiet mode (exit code only)
        #[arg(short, long)]
        quiet: bool,
    },

    /// Minify JSON (remove whitespace)
    Minify {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,
    },

    /// Query JSON with a path expression
    Query {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// JSON path query (e.g., ".foo.bar[0]")
        #[arg(short, long)]
        path: String,
    },
}
