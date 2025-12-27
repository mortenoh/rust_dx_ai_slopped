//! UUID command arguments.

use clap::{Args, ValueEnum};

/// Generate UUIDs
#[derive(Args, Debug)]
pub struct UuidArgs {
    /// UUID version to generate (v4 = random, v7 = time-ordered)
    #[arg(
        short = 'T',
        long = "type",
        value_name = "VERSION",
        default_value = "v4"
    )]
    pub uuid_version: UuidVersion,

    /// Number of UUIDs to generate
    #[arg(short, long, default_value = "1")]
    pub count: usize,

    /// Output format
    #[arg(short, long, default_value = "standard")]
    pub format: UuidFormat,

    /// Uppercase output
    #[arg(short = 'U', long)]
    pub uppercase: bool,
}

/// UUID version
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum UuidVersion {
    /// Random UUID (version 4)
    #[default]
    V4,
    /// Time-ordered UUID (version 7)
    V7,
}

/// UUID output format
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum UuidFormat {
    /// Standard format: 8-4-4-4-12
    #[default]
    Standard,
    /// No dashes: 32 hex chars
    Simple,
    /// URN format: urn:uuid:...
    Urn,
    /// Braced format: {uuid}
    Braced,
}
