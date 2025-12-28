//! Hash command arguments.

use clap::{Args, ValueEnum};
use std::path::PathBuf;

/// Compute file or string hashes
#[derive(Args, Debug)]
pub struct HashArgs {
    /// Input file to hash (use - for stdin)
    #[arg(value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Hash a string instead of a file
    #[arg(short, long, conflicts_with = "input")]
    pub string: Option<String>,

    /// Hash algorithm to use
    #[arg(short, long, default_value = "sha256")]
    pub algorithm: Algorithm,

    /// Output only the hash value (no filename)
    #[arg(short = 'q', long)]
    pub quiet: bool,

    /// Verify hash against expected value
    #[arg(long, value_name = "HASH")]
    pub verify: Option<String>,

    /// Cost factor for bcrypt (4-31) or Argon2 iterations
    #[arg(long, default_value = "12")]
    pub cost: u32,
}

/// Supported hash algorithms
#[derive(Debug, Clone, Copy, Default, ValueEnum, PartialEq, Eq)]
pub enum Algorithm {
    /// MD5 (128-bit, fast but not secure)
    Md5,
    /// SHA-256 (256-bit, recommended)
    #[default]
    Sha256,
    /// SHA-512 (512-bit, more secure)
    Sha512,
    /// Bcrypt password hash (includes salt, configurable cost)
    Bcrypt,
    /// Argon2id password hash (includes salt, memory-hard)
    Argon2,
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::Md5 => write!(f, "MD5"),
            Algorithm::Sha256 => write!(f, "SHA256"),
            Algorithm::Sha512 => write!(f, "SHA512"),
            Algorithm::Bcrypt => write!(f, "BCRYPT"),
            Algorithm::Argon2 => write!(f, "ARGON2"),
        }
    }
}
