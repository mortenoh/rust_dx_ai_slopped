//! Grep command arguments.

use clap::Args;
use std::path::PathBuf;

/// Search for patterns in files
#[derive(Args, Debug)]
pub struct GrepArgs {
    /// Pattern to search for (regex supported)
    #[arg(value_name = "PATTERN")]
    pub pattern: String,

    /// Files or directories to search (default: current directory)
    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,

    /// Case-insensitive search
    #[arg(short, long)]
    pub ignore_case: bool,

    /// Show line numbers
    #[arg(short = 'n', long)]
    pub line_number: bool,

    /// Recursive search in directories
    #[arg(short, long)]
    pub recursive: bool,

    /// Only show filenames with matches
    #[arg(short = 'l', long)]
    pub files_only: bool,

    /// Show count of matches per file
    #[arg(short, long)]
    pub count: bool,

    /// Invert match (show non-matching lines)
    #[arg(long)]
    pub invert: bool,

    /// Show N lines before match
    #[arg(short = 'B', long, value_name = "N")]
    pub before: Option<usize>,

    /// Show N lines after match
    #[arg(short = 'A', long, value_name = "N")]
    pub after: Option<usize>,

    /// Show N lines before and after match
    #[arg(short = 'C', long, value_name = "N")]
    pub context: Option<usize>,

    /// Include hidden files
    #[arg(long)]
    pub hidden: bool,

    /// File pattern to include (e.g., "*.rs")
    #[arg(long, value_name = "GLOB")]
    pub include: Option<String>,

    /// File pattern to exclude
    #[arg(long, value_name = "GLOB")]
    pub exclude: Option<String>,
}
