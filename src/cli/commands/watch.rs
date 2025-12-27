//! Watch command arguments.

use clap::Args;
use std::path::PathBuf;

/// Watch files for changes and run commands
#[derive(Args, Debug)]
pub struct WatchArgs {
    /// Paths to watch (files or directories)
    #[arg(value_name = "PATH", required = true)]
    pub paths: Vec<PathBuf>,

    /// Command to run on change (everything after --)
    #[arg(last = true, required = true)]
    pub command: Vec<String>,

    /// Debounce delay in milliseconds
    #[arg(short, long, default_value = "500")]
    pub debounce: u64,

    /// Clear screen before running command
    #[arg(short, long)]
    pub clear: bool,

    /// Run command immediately on start
    #[arg(short, long)]
    pub initial: bool,

    /// File patterns to include (e.g., "*.rs")
    #[arg(long, value_name = "GLOB")]
    pub include: Option<String>,

    /// File patterns to exclude
    #[arg(long, value_name = "GLOB")]
    pub exclude: Option<String>,

    /// Watch recursively (default: true)
    #[arg(long, default_value = "true")]
    pub recursive: bool,
}
