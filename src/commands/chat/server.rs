//! Chat server implementation using tonic gRPC.

use super::proto::chat_server::{Chat, ChatServer};
use super::proto::{ChatMessage, Empty, SubscribeRequest};
use anyhow::Result;
use colored::Colorize;
use std::net::SocketAddr;
use std::pin::Pin;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};

/// Capacity of the broadcast channel
const CHANNEL_CAPACITY: usize = 100;

/// Chat service state
pub struct ChatService {
    /// Broadcast channel for messages
    tx: broadcast::Sender<ChatMessage>,
}

impl ChatService {
    fn new() -> Self {
        let (tx, _) = broadcast::channel(CHANNEL_CAPACITY);
        Self { tx }
    }
}

#[tonic::async_trait]
impl Chat for ChatService {
    /// Handle incoming messages from clients
    async fn send_message(
        &self,
        request: Request<ChatMessage>,
    ) -> Result<Response<Empty>, Status> {
        let msg = request.into_inner();

        // Log the message on server
        let timestamp = chrono::Utc::now().format("%H:%M:%S");
        println!(
            "{} {}: {}",
            format!("[{}]", timestamp).dimmed(),
            msg.sender.cyan(),
            msg.content
        );

        // Broadcast to all subscribers
        // Ignore errors (no receivers is ok)
        let _ = self.tx.send(msg);

        Ok(Response::new(Empty {}))
    }

    /// Server-streaming response type
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;

    /// Handle subscription requests - returns a stream of messages
    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let name = request.into_inner().name;
        let timestamp = chrono::Utc::now().format("%H:%M:%S");

        println!(
            "{} {} {}",
            format!("[{}]", timestamp).dimmed(),
            name.green(),
            "joined".green()
        );

        // Create a new receiver for this subscriber
        let rx = self.tx.subscribe();

        // Announce the join
        let join_msg = ChatMessage {
            sender: "server".to_string(),
            content: format!("{} joined the chat", name),
            timestamp: chrono::Utc::now().timestamp_millis(),
        };
        let _ = self.tx.send(join_msg);

        // Convert broadcast receiver to a stream
        let stream = BroadcastStream::new(rx);

        // Map the stream to handle lagged receivers gracefully
        let output_stream = tokio_stream::StreamExt::filter_map(stream, |result| {
            match result {
                Ok(msg) => Some(Ok(msg)),
                Err(_) => {
                    // Client fell behind or channel closed, skip
                    None
                }
            }
        });

        Ok(Response::new(Box::pin(output_stream)))
    }
}

/// Start the chat server
pub async fn run(port: u16) -> Result<()> {
    let addr: SocketAddr = format!("[::1]:{}", port).parse()?;

    let service = ChatService::new();

    println!(
        "{} Chat server listening on {}",
        "✓".green().bold(),
        addr.to_string().cyan()
    );
    println!("{}", "Press Ctrl+C to stop".dimmed());
    println!();

    tonic::transport::Server::builder()
        .add_service(ChatServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
