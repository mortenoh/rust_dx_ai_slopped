//! Calculator command arguments.

use clap::{Args, Subcommand};

/// Unit conversions and calculations
#[derive(Args, Debug)]
pub struct CalcArgs {
    #[command(subcommand)]
    pub command: CalcCommand,
}

#[derive(Subcommand, Debug)]
pub enum CalcCommand {
    /// Convert byte sizes
    #[command(visible_alias = "b")]
    Bytes {
        /// Size to convert (e.g., "1.5gb", "1024", "500mb")
        value: String,
    },
    /// Convert/parse durations
    #[command(visible_alias = "t")]
    Time {
        /// Duration to convert (e.g., "3600s", "90m", "1h30m")
        value: String,
    },
    /// Calculate percentage (value of total)
    #[command(visible_alias = "p")]
    Percent {
        /// Value
        value: f64,
        /// Total
        total: f64,
    },
    /// Convert between number bases
    Base {
        /// Number to convert
        number: String,
        /// Source base (2-36)
        from: u32,
        /// Target base (2-36)
        to: u32,
    },
}
