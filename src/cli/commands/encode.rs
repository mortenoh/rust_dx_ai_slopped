//! Encode command arguments.

use clap::{Args, ValueEnum};
use std::path::PathBuf;

/// Encode or decode data
#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// Input file (use - for stdin)
    #[arg(value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Encode/decode a string instead of a file
    #[arg(short, long, conflicts_with = "input")]
    pub string: Option<String>,

    /// Encoding format
    #[arg(short, long, default_value = "base64")]
    pub format: EncodingFormat,

    /// Decode instead of encode
    #[arg(short, long)]
    pub decode: bool,

    /// URL-safe encoding (for base64)
    #[arg(long)]
    pub url_safe: bool,

    /// Don't add padding (for base64)
    #[arg(long)]
    pub no_padding: bool,
}

/// Supported encoding formats
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum EncodingFormat {
    /// Base64 encoding
    #[default]
    Base64,
    /// Hexadecimal encoding
    Hex,
}
