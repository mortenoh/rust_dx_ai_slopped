//! Chat client implementation using tonic gRPC.

use super::proto::chat_client::ChatClient;
use super::proto::{ChatMessage, SubscribeRequest};
use anyhow::{Context, Result};
use colored::Colorize;
use tokio::io::{AsyncBufReadExt, BufReader};

/// Connect to chat server and run the client
pub async fn run(name: &str, server: &str) -> Result<()> {
    // Connect to the server
    let mut client = ChatClient::connect(server.to_string())
        .await
        .context("Failed to connect to chat server")?;

    println!(
        "{} Connected to {}",
        "✓".green().bold(),
        server.cyan()
    );
    println!(
        "{}",
        "Type messages and press Enter to send. Ctrl+C to quit.".dimmed()
    );
    println!();

    // Clone client for the sender task
    let mut sender_client = client.clone();
    let sender_name = name.to_string();

    // Subscribe to receive messages
    let request = SubscribeRequest {
        name: name.to_string(),
    };

    let response = client.subscribe(request).await?;
    let mut stream = response.into_inner();

    // Spawn task to handle incoming messages
    let receiver_name = name.to_string();
    let receiver_handle = tokio::spawn(async move {
        while let Some(result) = tokio_stream::StreamExt::next(&mut stream).await {
            match result {
                Ok(msg) => {
                    let timestamp = chrono::Utc::now().format("%H:%M:%S");

                    // Don't echo our own messages (server already showed them)
                    if msg.sender == receiver_name {
                        continue;
                    }

                    // Format based on sender
                    if msg.sender == "server" {
                        println!(
                            "{} {}",
                            format!("[{}]", timestamp).dimmed(),
                            msg.content.yellow()
                        );
                    } else {
                        println!(
                            "{} {}: {}",
                            format!("[{}]", timestamp).dimmed(),
                            msg.sender.cyan(),
                            msg.content
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{} Stream error: {}", "✗".red(), e);
                    break;
                }
            }
        }
    });

    // Read stdin and send messages
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let content = line.trim().to_string();
        if content.is_empty() {
            continue;
        }

        let msg = ChatMessage {
            sender: sender_name.clone(),
            content,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        if let Err(e) = sender_client.send_message(msg).await {
            eprintln!("{} Failed to send: {}", "✗".red(), e);
        }
    }

    // Clean up
    receiver_handle.abort();

    Ok(())
}
