//! # rust_cli_complete
//!
//! A comprehensive CLI toolkit library demonstrating production-ready patterns.
//!
//! This library provides:
//! - CLI argument parsing with clap
//! - File hashing (MD5, SHA256, SHA512)
//! - Base64/hex encoding and decoding
//! - UUID generation (v4, v7)
//! - Timestamp conversions
//! - JSON formatting and validation
//! - Environment variable management
//! - Configuration management
//! - Expression evaluation
//!
//! ## Example
//!
//! ```rust,ignore
//! use rust_cli_complete::commands::hash;
//!
//! // Hash a file with SHA256
//! let result = hash::hash_file("path/to/file", hash::Algorithm::Sha256)?;
//! println!("SHA256: {}", result);
//! ```
//!
//! ```
//! use rust_cli_complete::expr;
//!
//! // Evaluate arithmetic expressions
//! let result = expr::eval("2 + 3 * 4").unwrap();
//! assert_eq!(result, 14.0);
//! ```

pub mod cli;
pub mod commands;
pub mod config;
pub mod expr;
pub mod utils;

/// Re-export common types
pub use cli::{Cli, Commands};
