//! Encrypt command arguments.

use clap::{Args, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Encryption utilities
#[derive(Args, Debug)]
pub struct EncryptArgs {
    #[command(subcommand)]
    pub command: EncryptCommand,
}

/// Encryption algorithm
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum EncryptAlgorithm {
    /// ChaCha20-Poly1305 (recommended, fast in software)
    #[default]
    Chacha20,
    /// AES-256-GCM
    AesGcm,
}

/// Encrypt subcommands
#[derive(Subcommand, Debug)]
pub enum EncryptCommand {
    /// Encrypt data
    Encrypt {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// String to encrypt (alternative to file)
        #[arg(short, long)]
        string: Option<String>,

        /// Output file (default: stdout)
        #[arg(long, name = "out")]
        out_file: Option<PathBuf>,

        /// Password/key for encryption
        #[arg(short, long)]
        password: String,

        /// Algorithm to use
        #[arg(short, long, default_value = "chacha20")]
        algorithm: EncryptAlgorithm,
    },

    /// Decrypt data
    Decrypt {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// String to decrypt (base64 encoded)
        #[arg(short, long)]
        string: Option<String>,

        /// Output file (default: stdout)
        #[arg(long, name = "out")]
        out_file: Option<PathBuf>,

        /// Password/key for decryption
        #[arg(short, long)]
        password: String,

        /// Algorithm to use
        #[arg(short, long, default_value = "chacha20")]
        algorithm: EncryptAlgorithm,
    },
}
