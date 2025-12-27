//! Time command arguments.

use clap::{Args, Subcommand, ValueEnum};

/// Convert and format timestamps
#[derive(Args, Debug)]
pub struct TimeArgs {
    #[command(subcommand)]
    pub command: TimeCommand,
}

/// Time subcommands
#[derive(Subcommand, Debug)]
pub enum TimeCommand {
    /// Show current time in various formats
    Now {
        /// Output format
        #[arg(short, long, default_value = "iso")]
        format: TimeFormat,

        /// Timezone (e.g., UTC, America/New_York)
        #[arg(short, long, default_value = "local")]
        timezone: String,
    },

    /// Parse a timestamp and show in different formats
    Parse {
        /// Timestamp to parse
        timestamp: String,

        /// Input format hint
        #[arg(short, long)]
        input_format: Option<String>,
    },

    /// Convert between timestamp formats
    Convert {
        /// Input timestamp
        input: String,

        /// Output format
        #[arg(short, long, default_value = "iso")]
        format: TimeFormat,
    },

    /// Calculate duration between two timestamps
    Diff {
        /// Start timestamp
        start: String,

        /// End timestamp (defaults to now)
        end: Option<String>,
    },
}

/// Time output formats
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum TimeFormat {
    /// ISO 8601 format
    #[default]
    Iso,
    /// Unix timestamp (seconds)
    Unix,
    /// Unix timestamp (milliseconds)
    UnixMs,
    /// RFC 2822 format
    Rfc2822,
    /// RFC 3339 format
    Rfc3339,
    /// Human readable
    Human,
}
