//! # Network Utilities Command
//!
//! Network-related utilities for developers.
//!
//! ## Examples
//! ```bash
//! dx net ip                  # Show local IP addresses
//! dx net ip --public         # Show public IP
//! dx net url "https://example.com:8080/path?q=1"
//! dx net port 8080           # Check if port is in use
//! dx net lookup google.com   # DNS lookup
//! ```

use crate::cli::commands::net::{NetArgs, NetCommand};
use anyhow::{Context, Result};
use colored::Colorize;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn run(args: NetArgs) -> Result<()> {
    match args.command {
        NetCommand::Ip { public } => cmd_ip(public),
        NetCommand::Url { url } => cmd_url(&url),
        NetCommand::Port { port, host } => cmd_port(&host, port),
        NetCommand::Lookup { domain } => cmd_lookup(&domain),
    }
}

/// Show IP addresses
fn cmd_ip(public: bool) -> Result<()> {
    if public {
        // Get public IP via external service
        // We'll use a simple approach without async
        println!("{}: fetching...", "public".cyan());
        match get_public_ip() {
            Ok(ip) => println!("{}: {}", "public".cyan(), ip),
            Err(e) => println!("{}: {} ({})", "public".cyan(), "unavailable".yellow(), e),
        }
    } else {
        // Get local IPs
        match local_ip_address::local_ip() {
            Ok(ip) => println!("{}: {}", "local".cyan(), ip),
            Err(e) => println!("{}: {} ({})", "local".cyan(), "unavailable".yellow(), e),
        }

        // Also try to get all local IPs
        if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
            for (name, ip) in interfaces {
                if !ip.is_loopback() {
                    println!("{}: {}", name.cyan(), ip);
                }
            }
        }
    }
    Ok(())
}

/// Get public IP from external service
fn get_public_ip() -> Result<String> {
    // Use a simple HTTP request without reqwest
    // We'll connect to a simple IP echo service
    let addrs = "api.ipify.org:80"
        .to_socket_addrs()?
        .next()
        .context("Could not resolve api.ipify.org")?;

    let mut stream = TcpStream::connect_timeout(&addrs, Duration::from_secs(5))?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    use std::io::{Read, Write};
    stream.write_all(b"GET / HTTP/1.1\r\nHost: api.ipify.org\r\nConnection: close\r\n\r\n")?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Extract body (after \r\n\r\n)
    if let Some(body_start) = response.find("\r\n\r\n") {
        let ip = response[body_start + 4..].trim().to_string();
        Ok(ip)
    } else {
        anyhow::bail!("Invalid response")
    }
}

/// Parse and display URL components
fn cmd_url(url_str: &str) -> Result<()> {
    let parsed = url::Url::parse(url_str).context("Invalid URL")?;

    println!("{}: {}", "scheme".cyan(), parsed.scheme());

    if let Some(host) = parsed.host_str() {
        println!("{}: {}", "host".cyan(), host);
    }

    if let Some(port) = parsed.port() {
        println!("{}: {}", "port".cyan(), port);
    } else {
        // Show default port for known schemes
        let default_port = match parsed.scheme() {
            "http" => Some(80),
            "https" => Some(443),
            "ftp" => Some(21),
            "ssh" => Some(22),
            _ => None,
        };
        if let Some(p) = default_port {
            println!("{}: {} (default)", "port".cyan(), p);
        }
    }

    println!("{}: {}", "path".cyan(), parsed.path());

    if let Some(query) = parsed.query() {
        println!("{}: {}", "query".cyan(), query);

        // Parse query parameters
        for (key, value) in parsed.query_pairs() {
            println!("  {}: {}", key.dimmed(), value);
        }
    }

    if let Some(fragment) = parsed.fragment() {
        println!("{}: {}", "fragment".cyan(), fragment);
    }

    if !parsed.username().is_empty() {
        println!("{}: {}", "username".cyan(), parsed.username());
    }

    Ok(())
}

/// Check if a port is in use
fn cmd_port(host: &str, port: u16) -> Result<()> {
    let addr = format!("{}:{}", host, port);

    match TcpStream::connect_timeout(
        &addr.to_socket_addrs()?.next().context("Invalid address")?,
        Duration::from_secs(2),
    ) {
        Ok(_) => {
            println!(
                "{} Port {} is {} on {}",
                "✓".green().bold(),
                port,
                "open".green(),
                host
            );
        }
        Err(_) => {
            println!(
                "{} Port {} is {} on {}",
                "✗".red().bold(),
                port,
                "closed".red(),
                host
            );
        }
    }

    Ok(())
}

/// DNS lookup
fn cmd_lookup(domain: &str) -> Result<()> {
    // Simple DNS lookup using std
    let addr = format!("{}:80", domain);

    match addr.to_socket_addrs() {
        Ok(addrs) => {
            println!("{}: {}", "domain".cyan(), domain);
            for addr in addrs {
                println!("  {}", addr.ip());
            }
        }
        Err(e) => {
            println!("{} Could not resolve {}: {}", "✗".red().bold(), domain, e);
        }
    }

    Ok(())
}
