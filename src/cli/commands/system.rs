//! System command arguments.

use clap::{Args, Subcommand};

/// System information and utilities
#[derive(Args, Debug)]
pub struct SystemArgs {
    #[command(subcommand)]
    pub command: SystemCommand,
}

#[derive(Subcommand, Debug)]
pub enum SystemCommand {
    /// Display system information (OS, CPU, memory, disk)
    Info {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}
