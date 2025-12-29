//! Polars command arguments - DataFrame operations and data analysis.

use clap::{Args, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Output format for screen display.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum PolarsOutputFormat {
    /// Table format (default)
    #[default]
    Table,
    /// CSV format
    Csv,
    /// JSON array of objects
    Json,
    /// JSON Lines (one object per line)
    Jsonl,
}

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
        #[arg(short = 'n', long, default_value = "20")]
        rows: usize,

        /// Column definitions as name:type pairs (comma-separated)
        ///
        /// PRIMITIVES (with optional parameters):
        ///   id[start;step]  - Sequential integers (default: 1, 1)
        ///   int[min;max]    - Random integers (default: 0, 1000)
        ///   float[min;max]  - Random floats (default: 0.0, 1000.0)
        ///   string[len]     - Random alphanumeric (default: 10)
        ///   bool[prob]      - Random true/false (default: 0.5)
        ///   date            - Random dates (2020-2025)
        ///
        /// CATEGORIES:
        ///   category  - Generic cat_0..cat_N (--categories)
        ///   fruit, color, city, country, status
        ///   priority, department, day, size
        ///
        /// PERSONAL:
        ///   first_name, last_name, full_name (alias: name)
        ///   email, username, phone
        ///   address, zip_code
        ///
        /// NETWORK:
        ///   ipv4 (alias: ip), ipv6, mac_address
        ///   domain, url
        ///
        /// IDENTIFIERS:
        ///   uuid (alias: uuid4), uuid7
        ///   credit_card (alias: cc), iban
        ///   isbn (alias: isbn13), isbn10
        ///   ssn (alias: ssn_us), ssn_no
        ///
        /// TEXT:
        ///   word, sentence, paragraph
        ///
        /// GEO:
        ///   lat[min;max], lon[min;max]  - Coordinates
        ///   coords[bbox], point[bbox]   - GeoJSON Point
        ///   (bbox = minLon;minLat;maxLon;maxLat)
        ///
        /// OTHER:
        ///   password, hex
        ///
        /// Example: -c "id:id,user:email,card:credit_card"
        /// Example: -c "id:id,loc:point[4;57;31;71]"
        #[arg(short, long, value_delimiter = ',', verbatim_doc_comment)]
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

        /// Output format (table, json, jsonl)
        #[arg(short, long, default_value = "table")]
        format: PolarsOutputFormat,
    },
}
