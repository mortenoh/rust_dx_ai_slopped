//! Network utilities command arguments.

use clap::{Args, Subcommand};

/// Network utilities
#[derive(Args, Debug)]
pub struct NetArgs {
    #[command(subcommand)]
    pub command: NetCommand,
}

#[derive(Subcommand, Debug)]
pub enum NetCommand {
    /// Show IP addresses
    Ip {
        /// Show public IP (via external API)
        #[arg(long)]
        public: bool,
    },
    /// Parse and analyze URL
    Url {
        /// URL to parse
        url: String,
    },
    /// Check if a port is in use
    Port {
        /// Port number to check
        port: u16,
        /// Host to check (default: localhost)
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,
    },
    /// DNS lookup
    Lookup {
        /// Domain name to lookup
        domain: String,
    },
}
