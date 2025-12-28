//! Compress command arguments.

use clap::{Args, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Compression utilities
#[derive(Args, Debug)]
pub struct CompressArgs {
    #[command(subcommand)]
    pub command: CompressCommand,
}

/// Compression format
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum CompressionFormat {
    /// Gzip compression
    #[default]
    Gzip,
    /// Zstandard compression
    Zstd,
}

/// Compress subcommands
#[derive(Subcommand, Debug)]
pub enum CompressCommand {
    /// Compress a file
    Compress {
        /// Input file
        input: PathBuf,

        /// Output file (default: input.gz or input.zst)
        #[arg(short = 'O', long, name = "out")]
        out_file: Option<PathBuf>,

        /// Compression format
        #[arg(short, long, default_value = "gzip")]
        format: CompressionFormat,

        /// Compression level (1-9 for gzip, 1-22 for zstd)
        #[arg(short, long, default_value = "6")]
        level: u32,
    },

    /// Decompress a file
    Decompress {
        /// Input file (.gz or .zst)
        input: PathBuf,

        /// Output file (default: input without extension)
        #[arg(short = 'O', long, name = "out")]
        out_file: Option<PathBuf>,
    },
}
