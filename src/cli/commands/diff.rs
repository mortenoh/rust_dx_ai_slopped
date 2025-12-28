//! Diff command arguments.

use clap::{Args, ValueEnum};
use std::path::PathBuf;

/// Text diffing utilities
#[derive(Args, Debug)]
pub struct DiffArgs {
    /// First file to compare
    pub file1: PathBuf,

    /// Second file to compare
    pub file2: PathBuf,

    /// Output format
    #[arg(short, long, default_value = "unified")]
    pub format: DiffFormat,

    /// Number of context lines
    #[arg(short = 'C', long, default_value = "3")]
    pub context: usize,
}

/// Diff output format
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum DiffFormat {
    /// Unified diff format
    #[default]
    Unified,
    /// Inline diff with change markers
    Inline,
    /// Compact diff (changes only)
    Compact,
}
