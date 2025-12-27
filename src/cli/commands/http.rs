//! HTTP command arguments.

use clap::{Args, Subcommand, ValueEnum};

/// Make HTTP requests
#[derive(Args, Debug)]
pub struct HttpArgs {
    #[command(subcommand)]
    pub command: HttpCommand,
}

#[derive(Subcommand, Debug)]
pub enum HttpCommand {
    /// Send GET request
    Get {
        /// URL to request
        url: String,

        /// Request headers (can be repeated)
        #[arg(short = 'H', long = "header", value_name = "KEY:VALUE")]
        headers: Vec<String>,

        /// Output format
        #[arg(short, long, default_value = "body")]
        format: OutputFormat,

        /// Follow redirects
        #[arg(short = 'L', long)]
        follow: bool,

        /// Request timeout in seconds
        #[arg(short, long, default_value = "30")]
        timeout: u64,
    },

    /// Send POST request
    Post {
        /// URL to request
        url: String,

        /// Request body (JSON or raw text)
        #[arg(short, long)]
        data: Option<String>,

        /// Read body from file
        #[arg(short = 'F', long, conflicts_with = "data")]
        file: Option<String>,

        /// Request headers (can be repeated)
        #[arg(short = 'H', long = "header", value_name = "KEY:VALUE")]
        headers: Vec<String>,

        /// Content type
        #[arg(long, default_value = "application/json")]
        content_type: String,

        /// Output format
        #[arg(short, long, default_value = "body")]
        format: OutputFormat,

        /// Follow redirects
        #[arg(short = 'L', long)]
        follow: bool,

        /// Request timeout in seconds
        #[arg(short, long, default_value = "30")]
        timeout: u64,
    },

    /// Send PUT request
    Put {
        /// URL to request
        url: String,

        /// Request body
        #[arg(short, long)]
        data: Option<String>,

        /// Request headers
        #[arg(short = 'H', long = "header", value_name = "KEY:VALUE")]
        headers: Vec<String>,

        /// Content type
        #[arg(long, default_value = "application/json")]
        content_type: String,

        /// Output format
        #[arg(short, long, default_value = "body")]
        format: OutputFormat,

        /// Request timeout in seconds
        #[arg(short, long, default_value = "30")]
        timeout: u64,
    },

    /// Send DELETE request
    Delete {
        /// URL to request
        url: String,

        /// Request headers
        #[arg(short = 'H', long = "header", value_name = "KEY:VALUE")]
        headers: Vec<String>,

        /// Output format
        #[arg(short, long, default_value = "body")]
        format: OutputFormat,

        /// Request timeout in seconds
        #[arg(short, long, default_value = "30")]
        timeout: u64,
    },

    /// Send HEAD request (headers only)
    Head {
        /// URL to request
        url: String,

        /// Request headers
        #[arg(short = 'H', long = "header", value_name = "KEY:VALUE")]
        headers: Vec<String>,

        /// Request timeout in seconds
        #[arg(short, long, default_value = "30")]
        timeout: u64,
    },
}

/// Output format for HTTP responses
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    /// Response body only
    #[default]
    Body,
    /// Headers and body
    Full,
    /// Headers only
    Headers,
    /// JSON formatted response info
    Json,
}
