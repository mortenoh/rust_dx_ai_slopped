//! JWT command arguments.

use clap::{Args, Subcommand, ValueEnum};

/// JWT utilities (decode, encode, verify)
#[derive(Args, Debug)]
pub struct JwtArgs {
    #[command(subcommand)]
    pub command: JwtCommand,
}

/// JWT algorithm
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum JwtAlgorithm {
    /// HMAC SHA-256
    #[default]
    Hs256,
    /// HMAC SHA-384
    Hs384,
    /// HMAC SHA-512
    Hs512,
}

/// JWT subcommands
#[derive(Subcommand, Debug)]
pub enum JwtCommand {
    /// Decode a JWT token (without verification)
    Decode {
        /// JWT token to decode
        token: String,

        /// Output format
        #[arg(short, long, default_value = "pretty")]
        format: DecodeFormat,
    },

    /// Create a JWT token
    Encode {
        /// Secret key for signing
        #[arg(short, long)]
        secret: String,

        /// JSON payload (claims)
        #[arg(short, long)]
        payload: String,

        /// Algorithm to use
        #[arg(short, long, default_value = "hs256")]
        algorithm: JwtAlgorithm,

        /// Expiration time in seconds from now
        #[arg(long)]
        exp: Option<i64>,

        /// Subject claim
        #[arg(long)]
        sub: Option<String>,

        /// Issuer claim
        #[arg(long)]
        iss: Option<String>,
    },

    /// Verify a JWT token
    Verify {
        /// JWT token to verify
        token: String,

        /// Secret key for verification
        #[arg(short, long)]
        secret: String,

        /// Algorithm to use
        #[arg(short, long, default_value = "hs256")]
        algorithm: JwtAlgorithm,
    },
}

/// Output format for decode
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum DecodeFormat {
    /// Pretty-printed JSON
    #[default]
    Pretty,
    /// Compact JSON
    Json,
    /// Raw base64 parts
    Raw,
}
