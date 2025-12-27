//! Random generation command arguments.

use clap::{Args, Subcommand};

/// Random data generation
#[derive(Args, Debug)]
pub struct RandArgs {
    #[command(subcommand)]
    pub command: RandCommand,
}

#[derive(Subcommand, Debug)]
pub enum RandCommand {
    /// Generate random integer
    Int {
        /// Minimum value (inclusive)
        #[arg(default_value = "1")]
        min: i64,
        /// Maximum value (inclusive)
        #[arg(default_value = "100")]
        max: i64,
        /// Number of values to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    /// Generate random float
    Float {
        /// Minimum value
        #[arg(default_value = "0.0")]
        min: f64,
        /// Maximum value
        #[arg(default_value = "1.0")]
        max: f64,
        /// Number of values to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    /// Generate random alphanumeric string
    String {
        /// Length of string
        #[arg(default_value = "16")]
        length: usize,
        /// Number of strings to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    /// Generate random hex string
    Hex {
        /// Length in bytes (output is 2x this)
        #[arg(default_value = "16")]
        bytes: usize,
        /// Number of strings to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    /// Generate secure password
    Password {
        /// Length of password
        #[arg(default_value = "16")]
        length: usize,
        /// Exclude symbols
        #[arg(long)]
        no_symbols: bool,
        /// Number of passwords to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    /// Pick random item from list
    Choice {
        /// Items to choose from
        #[arg(required = true, num_args = 1..)]
        items: Vec<String>,
    },
    /// Shuffle items
    Shuffle {
        /// Items to shuffle
        #[arg(required = true, num_args = 1..)]
        items: Vec<String>,
    },
    /// Flip a coin
    Coin {
        /// Number of flips
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
    /// Roll dice
    Dice {
        /// Number of sides
        #[arg(default_value = "6")]
        sides: u32,
        /// Number of dice to roll
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
}
