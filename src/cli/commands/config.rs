//! Config command arguments.

use clap::{Args, Subcommand};

/// Manage application configuration
#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

/// Config subcommands
#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// Show current configuration
    Show {
        /// Show specific key only
        key: Option<String>,

        /// Output format
        #[arg(short, long, default_value = "toml")]
        format: ConfigFormat,
    },

    /// Get a configuration value
    Get {
        /// Configuration key (dot-separated for nested)
        key: String,
    },

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,

        /// Value to set
        value: String,
    },

    /// Remove a configuration value
    Unset {
        /// Configuration key
        key: String,
    },

    /// List all configuration keys
    List {
        /// Show values too
        #[arg(long)]
        values: bool,
    },

    /// Show config file path
    Path,

    /// Edit config file in editor
    Edit,

    /// Reset configuration to defaults
    Reset {
        /// Don't ask for confirmation
        #[arg(short, long)]
        force: bool,
    },
}

/// Config output format
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum ConfigFormat {
    /// TOML format
    #[default]
    Toml,
    /// JSON format
    Json,
}
