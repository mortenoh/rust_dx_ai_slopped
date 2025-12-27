//! Environment command arguments.

use clap::{Args, Subcommand};

/// Manage environment variables
#[derive(Args, Debug)]
pub struct EnvArgs {
    #[command(subcommand)]
    pub command: EnvCommand,
}

/// Environment subcommands
#[derive(Subcommand, Debug)]
pub enum EnvCommand {
    /// List all environment variables
    List {
        /// Filter by prefix
        #[arg(short, long)]
        prefix: Option<String>,

        /// Show values (may contain secrets)
        #[arg(long)]
        show_values: bool,

        /// Sort alphabetically
        #[arg(short, long)]
        sort: bool,
    },

    /// Get a specific environment variable
    Get {
        /// Variable name
        name: String,

        /// Default value if not set
        #[arg(short, long)]
        default: Option<String>,
    },

    /// Check if environment variables are set
    Check {
        /// Variable names to check
        names: Vec<String>,

        /// Exit with error if any are missing
        #[arg(short, long)]
        strict: bool,
    },

    /// Export variables to different formats
    Export {
        /// Output format
        #[arg(short, long, default_value = "shell")]
        format: ExportFormat,

        /// Filter by prefix
        #[arg(short, long)]
        prefix: Option<String>,
    },
}

/// Export format
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum ExportFormat {
    /// Shell export format (export KEY=VALUE)
    #[default]
    Shell,
    /// Docker format (KEY=VALUE per line)
    Docker,
    /// JSON object
    Json,
    /// TOML table
    Toml,
}
