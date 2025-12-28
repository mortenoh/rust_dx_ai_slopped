//! DHIS2 command arguments.

use clap::{Args, Subcommand};

/// DHIS2 utilities - interact with DHIS2 instances
#[derive(Args, Debug)]
pub struct Dhis2Args {
    /// DHIS2 server URL
    #[arg(
        long,
        env = "DHIS2_SERVER",
        default_value = "https://play.im.dhis2.org/stable-2-42-3-1",
        global = true
    )]
    pub server: String,

    /// DHIS2 username
    #[arg(long, env = "DHIS2_USER", default_value = "admin", global = true)]
    pub user: String,

    /// DHIS2 password
    #[arg(
        long,
        env = "DHIS2_PASSWORD",
        default_value = "district",
        global = true
    )]
    pub password: String,

    #[command(subcommand)]
    pub command: Dhis2Command,
}

/// DHIS2 subcommands
#[derive(Subcommand, Debug)]
pub enum Dhis2Command {
    /// Generate DHIS2 UIDs (11-character alphanumeric identifiers)
    Uid {
        /// Number of UIDs to generate
        #[arg(default_value = "1")]
        count: usize,

        /// Validate a UID instead of generating
        #[arg(long)]
        validate: Option<String>,

        /// Output as JSON
        #[arg(short, long)]
        json: bool,

        /// Output one UID per line (no extra formatting)
        #[arg(short, long)]
        plain: bool,
    },

    /// Show DHIS2 server information and version
    Info,

    /// List organisation units
    OrgUnits {
        /// Filter by level (1-4)
        #[arg(long)]
        level: Option<i32>,

        /// Maximum number of results
        #[arg(long, default_value = "25")]
        limit: usize,

        /// Include geometry in output
        #[arg(long)]
        geometry: bool,

        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },

    /// List data elements
    #[command(visible_alias = "de")]
    DataElements {
        /// Maximum number of results
        #[arg(long, default_value = "25")]
        limit: usize,

        /// Filter by value type (e.g., NUMBER, TEXT, BOOLEAN)
        #[arg(long)]
        value_type: Option<String>,

        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },

    /// List data sets
    #[command(visible_alias = "ds")]
    DataSets {
        /// Maximum number of results
        #[arg(long, default_value = "25")]
        limit: usize,

        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },

    /// List organisation unit groups
    #[command(visible_alias = "oug")]
    OrgUnitGroups {
        /// Maximum number of results
        #[arg(long, default_value = "25")]
        limit: usize,

        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },

    /// List organisation unit group sets
    #[command(visible_alias = "ougs")]
    OrgUnitGroupSets {
        /// Maximum number of results
        #[arg(long, default_value = "25")]
        limit: usize,

        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },

    /// Interactive TUI browser for organisation units
    #[cfg(feature = "ui")]
    Tui,
}
