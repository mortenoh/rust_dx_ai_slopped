//! Polars command arguments - DataFrame operations and data analysis.

use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct PolarsArgs {
    #[command(subcommand)]
    pub command: PolarsCommand,
}

#[derive(Subcommand, Debug)]
pub enum PolarsCommand {
    /// View data from CSV or Parquet file
    View {
        /// Input file (CSV or Parquet)
        file: PathBuf,

        /// Number of rows to display
        #[arg(short = 'n', long, default_value = "20")]
        rows: usize,

        /// Show last N rows instead of first
        #[arg(long)]
        tail: bool,

        /// Select specific columns (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        columns: Vec<String>,

        /// Show schema only (no data)
        #[arg(long)]
        schema: bool,

        /// Show summary statistics
        #[arg(long)]
        stats: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Generate random data (output to file or screen)
    Random {
        /// Output file path (format determined by extension: .csv, .parquet, .pq)
        /// If not specified, outputs to screen
        file: Option<PathBuf>,

        /// Number of rows to generate
        #[arg(short = 'n', long, default_value = "10000")]
        rows: usize,

        /// Column definitions: name:type (e.g., "id:id,city:city,score:float")
        /// Types: id, int, float, string, bool, date
        /// Categories: category, fruit, color, city, country, status, priority, department, day, size
        #[arg(short, long, value_delimiter = ',')]
        columns: Vec<String>,

        /// Number of categories for category columns
        #[arg(long, default_value = "10")]
        categories: usize,

        /// String length for string columns
        #[arg(long, default_value = "10")]
        string_len: usize,

        /// Min value for numeric columns
        #[arg(long, default_value = "0")]
        min: i64,

        /// Max value for numeric columns
        #[arg(long, default_value = "1000")]
        max: i64,

        /// Null probability (0.0 - 1.0)
        #[arg(long, default_value = "0.0")]
        null_prob: f64,

        /// Random seed for reproducibility
        #[arg(long)]
        seed: Option<u64>,
    },
}
