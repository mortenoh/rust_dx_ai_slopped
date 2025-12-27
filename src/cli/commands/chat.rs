//! # Chat Command CLI Arguments
//!
//! Defines the CLI structure for the gRPC-based chat command.
//!
//! ## Usage
//! ```bash
//! dx chat server [--port 50051]     # Start chat server
//! dx chat client <NAME> [--server]  # Connect as client
//! ```

use clap::{Args, Subcommand};

/// Arguments for the chat command
#[derive(Args, Debug)]
pub struct ChatArgs {
    /// The chat subcommand to run
    #[command(subcommand)]
    pub command: ChatCommand,
}

/// Available chat subcommands
#[derive(Subcommand, Debug)]
pub enum ChatCommand {
    /// Start a chat server
    ///
    /// Listens for client connections and broadcasts messages
    /// to all connected clients.
    Server {
        /// Port to listen on
        #[arg(short, long, default_value = "50051")]
        port: u16,
    },

    /// Connect to a chat server as a client
    ///
    /// Allows you to send and receive messages in real-time.
    Client {
        /// Your display name in the chat
        name: String,

        /// Server address to connect to
        #[arg(short, long, default_value = "http://[::1]:50051")]
        server: String,
    },
}
