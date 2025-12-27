//! HTTP command - make HTTP requests.

use crate::cli::commands::http::{HttpArgs, HttpCommand, OutputFormat};
use anyhow::{Context, Result};
use colored::Colorize;
use serde_json::{json, Value};
use std::fs;
use std::time::Duration;
use ureq::Agent;

/// Run the http command
pub fn run(args: HttpArgs) -> Result<()> {
    match args.command {
        HttpCommand::Get {
            url,
            headers,
            format,
            follow: _,
            timeout,
        } => cmd_get(&url, &headers, format, timeout),

        HttpCommand::Post {
            url,
            data,
            file,
            headers,
            content_type,
            format,
            follow: _,
            timeout,
        } => cmd_post(&url, data, file, &headers, &content_type, format, timeout),

        HttpCommand::Put {
            url,
            data,
            headers,
            content_type,
            format,
            timeout,
        } => cmd_put(&url, data, &headers, &content_type, format, timeout),

        HttpCommand::Delete {
            url,
            headers,
            format,
            timeout,
        } => cmd_delete(&url, &headers, format, timeout),

        HttpCommand::Head {
            url,
            headers,
            timeout,
        } => cmd_head(&url, &headers, timeout),
    }
}

fn create_agent(timeout: u64) -> Agent {
    Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(timeout)))
        .build()
        .into()
}

fn parse_header(header: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = header.splitn(2, ':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid header format: '{}'. Expected 'Key: Value'", header);
    }
    Ok((parts[0].trim().to_string(), parts[1].trim().to_string()))
}

fn print_status(status: u16, status_text: &str) {
    let status_str = format!("HTTP {} {}", status, status_text);
    if (200..300).contains(&status) {
        println!("{}", status_str.green());
    } else if status >= 400 {
        println!("{}", status_str.red());
    } else {
        println!("{}", status_str.yellow());
    }
}

fn print_body(body: &str) {
    // Try to pretty-print JSON
    if let Ok(json) = serde_json::from_str::<Value>(body) {
        println!(
            "{}",
            serde_json::to_string_pretty(&json).unwrap_or_else(|_| body.to_string())
        );
    } else {
        println!("{}", body);
    }
}

fn cmd_get(url: &str, headers: &[String], format: OutputFormat, timeout: u64) -> Result<()> {
    let agent = create_agent(timeout);
    let mut request = agent.get(url);

    for header in headers {
        let (key, value) = parse_header(header)?;
        request = request.header(&key, &value);
    }

    let response = request.call().context("Failed to send GET request")?;

    let status = response.status().as_u16();
    let status_text = response
        .status()
        .canonical_reason()
        .unwrap_or("Unknown");

    match format {
        OutputFormat::Headers => {
            print_status(status, status_text);
            for name in response.headers().keys() {
                if let Some(value) = response.headers().get(name) {
                    println!("{}: {}", name.as_str().cyan(), value.to_str().unwrap_or(""));
                }
            }
        }
        OutputFormat::Full => {
            print_status(status, status_text);
            for name in response.headers().keys() {
                if let Some(value) = response.headers().get(name) {
                    println!("{}: {}", name.as_str().cyan(), value.to_str().unwrap_or(""));
                }
            }
            println!();
            let body = response.into_body().read_to_string()?;
            print_body(&body);
        }
        OutputFormat::Body => {
            let body = response.into_body().read_to_string()?;
            print_body(&body);
        }
        OutputFormat::Json => {
            let body = response.into_body().read_to_string()?;
            let output = json!({
                "status": status,
                "status_text": status_text,
                "body": body,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }
    Ok(())
}

fn cmd_post(
    url: &str,
    data: Option<String>,
    file: Option<String>,
    headers: &[String],
    content_type: &str,
    format: OutputFormat,
    timeout: u64,
) -> Result<()> {
    let agent = create_agent(timeout);
    let mut request = agent.post(url).header("Content-Type", content_type);

    for header in headers {
        let (key, value) = parse_header(header)?;
        request = request.header(&key, &value);
    }

    let body = if let Some(data) = data {
        data
    } else if let Some(file_path) = file {
        fs::read_to_string(&file_path).context(format!("Failed to read file: {}", file_path))?
    } else {
        String::new()
    };

    let response = request
        .send(&body)
        .context("Failed to send POST request")?;

    let status = response.status().as_u16();
    let status_text = response.status().canonical_reason().unwrap_or("Unknown");

    match format {
        OutputFormat::Headers => {
            print_status(status, status_text);
            for name in response.headers().keys() {
                if let Some(value) = response.headers().get(name) {
                    println!("{}: {}", name.as_str().cyan(), value.to_str().unwrap_or(""));
                }
            }
        }
        OutputFormat::Full | OutputFormat::Body => {
            if matches!(format, OutputFormat::Full) {
                print_status(status, status_text);
                for name in response.headers().keys() {
                    if let Some(value) = response.headers().get(name) {
                        println!("{}: {}", name.as_str().cyan(), value.to_str().unwrap_or(""));
                    }
                }
                println!();
            }
            let body = response.into_body().read_to_string()?;
            print_body(&body);
        }
        OutputFormat::Json => {
            let body = response.into_body().read_to_string()?;
            let output = json!({
                "status": status,
                "status_text": status_text,
                "body": body,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }
    Ok(())
}

fn cmd_put(
    url: &str,
    data: Option<String>,
    headers: &[String],
    content_type: &str,
    format: OutputFormat,
    timeout: u64,
) -> Result<()> {
    let agent = create_agent(timeout);
    let mut request = agent.put(url).header("Content-Type", content_type);

    for header in headers {
        let (key, value) = parse_header(header)?;
        request = request.header(&key, &value);
    }

    let body = data.unwrap_or_default();

    let response = request.send(&body).context("Failed to send PUT request")?;

    let status = response.status().as_u16();
    let status_text = response.status().canonical_reason().unwrap_or("Unknown");

    match format {
        OutputFormat::Headers => {
            print_status(status, status_text);
        }
        OutputFormat::Full | OutputFormat::Body => {
            if matches!(format, OutputFormat::Full) {
                print_status(status, status_text);
                println!();
            }
            let body = response.into_body().read_to_string()?;
            print_body(&body);
        }
        OutputFormat::Json => {
            let body = response.into_body().read_to_string()?;
            let output = json!({
                "status": status,
                "status_text": status_text,
                "body": body,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }
    Ok(())
}

fn cmd_delete(url: &str, headers: &[String], format: OutputFormat, timeout: u64) -> Result<()> {
    let agent = create_agent(timeout);
    let mut request = agent.delete(url);

    for header in headers {
        let (key, value) = parse_header(header)?;
        request = request.header(&key, &value);
    }

    let response = request.call().context("Failed to send DELETE request")?;

    let status = response.status().as_u16();
    let status_text = response.status().canonical_reason().unwrap_or("Unknown");

    match format {
        OutputFormat::Headers => {
            print_status(status, status_text);
        }
        OutputFormat::Full | OutputFormat::Body => {
            if matches!(format, OutputFormat::Full) {
                print_status(status, status_text);
                println!();
            }
            let body = response.into_body().read_to_string()?;
            print_body(&body);
        }
        OutputFormat::Json => {
            let body = response.into_body().read_to_string()?;
            let output = json!({
                "status": status,
                "status_text": status_text,
                "body": body,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }
    Ok(())
}

fn cmd_head(url: &str, headers: &[String], timeout: u64) -> Result<()> {
    let agent = create_agent(timeout);
    let mut request = agent.head(url);

    for header in headers {
        let (key, value) = parse_header(header)?;
        request = request.header(&key, &value);
    }

    let response = request.call().context("Failed to send HEAD request")?;

    let status = response.status().as_u16();
    let status_text = response.status().canonical_reason().unwrap_or("Unknown");
    print_status(status, status_text);

    for name in response.headers().keys() {
        if let Some(value) = response.headers().get(name) {
            println!("{}: {}", name.as_str().cyan(), value.to_str().unwrap_or(""));
        }
    }
    Ok(())
}
