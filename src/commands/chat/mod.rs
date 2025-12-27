//! # Chat Command
//!
//! Real-time chat using gRPC (tonic).
//!
//! ## Examples
//! ```bash
//! # Start a server
//! dx chat server --port 50051
//!
//! # Connect clients
//! dx chat client Alice
//! dx chat client Bob --server http://localhost:50051
//! ```

mod client;
mod server;

use crate::cli::commands::chat::{ChatArgs, ChatCommand};
use anyhow::Result;

/// Include the generated protobuf code
pub mod proto {
    tonic::include_proto!("chat");
}

/// Run the chat command (async entry point)
pub async fn run(args: ChatArgs) -> Result<()> {
    match args.command {
        ChatCommand::Server { port } => server::run(port).await,
        ChatCommand::Client { name, server } => client::run(&name, &server).await,
    }
}
