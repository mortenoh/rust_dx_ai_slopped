//! CSV command arguments.

use clap::{Args, Subcommand};
use std::path::PathBuf;

/// CSV utilities (format, convert, query)
#[derive(Args, Debug)]
pub struct CsvArgs {
    #[command(subcommand)]
    pub command: CsvCommand,
}

/// CSV subcommands
#[derive(Subcommand, Debug)]
pub enum CsvCommand {
    /// Pretty-print CSV as a table
    Format {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Delimiter character
        #[arg(short, long, default_value = ",")]
        delimiter: char,

        /// No header row
        #[arg(long)]
        no_header: bool,
    },

    /// Convert CSV to JSON array
    #[command(name = "to-json")]
    ToJson {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Delimiter character
        #[arg(short, long, default_value = ",")]
        delimiter: char,

        /// Pretty-print output
        #[arg(short, long)]
        pretty: bool,
    },

    /// Convert JSON array to CSV
    #[command(name = "from-json")]
    FromJson {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,
    },

    /// Select specific columns from CSV
    Query {
        /// Input file (use - for stdin)
        #[arg(value_name = "FILE")]
        input: Option<PathBuf>,

        /// Columns to select (comma-separated names or indices)
        #[arg(short, long, value_delimiter = ',')]
        columns: Vec<String>,

        /// Delimiter character
        #[arg(short, long, default_value = ",")]
        delimiter: char,
    },
}
