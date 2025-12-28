//! CSV command - CSV utilities.

use crate::cli::commands::csv::{CsvArgs, CsvCommand};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// Run the csv command
pub fn run(args: CsvArgs) -> Result<()> {
    match args.command {
        CsvCommand::Format {
            input,
            delimiter,
            no_header,
        } => cmd_format(input, delimiter, no_header),
        CsvCommand::ToJson {
            input,
            delimiter,
            pretty,
        } => cmd_to_json(input, delimiter, pretty),
        CsvCommand::FromJson { input } => cmd_from_json(input),
        CsvCommand::Query {
            input,
            columns,
            delimiter,
        } => cmd_query(input, columns, delimiter),
    }
}

fn read_input(input: Option<PathBuf>) -> Result<String> {
    match input {
        Some(path) if path.to_string_lossy() == "-" => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
        Some(path) => fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path.display())),
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn cmd_format(input: Option<PathBuf>, delimiter: char, no_header: bool) -> Result<()> {
    let content = read_input(input)?;

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(!no_header)
        .from_reader(content.as_bytes());

    // Collect all records to determine column widths
    let headers: Vec<String> = if !no_header {
        rdr.headers()
            .context("Failed to read CSV headers")?
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        vec![]
    };

    let records: Vec<Vec<String>> = rdr
        .records()
        .map(|r| {
            r.map(|rec| rec.iter().map(|s| s.to_string()).collect())
                .context("Failed to read CSV record")
        })
        .collect::<Result<Vec<_>>>()?;

    // Calculate column widths
    let num_cols = if !headers.is_empty() {
        headers.len()
    } else if let Some(first) = records.first() {
        first.len()
    } else {
        return Ok(());
    };

    let mut widths = vec![0usize; num_cols];

    if !headers.is_empty() {
        for (i, h) in headers.iter().enumerate() {
            widths[i] = widths[i].max(h.len());
        }
    }

    for record in &records {
        for (i, field) in record.iter().enumerate() {
            if i < widths.len() {
                widths[i] = widths[i].max(field.len());
            }
        }
    }

    // Print formatted table
    if !headers.is_empty() {
        let header_line: Vec<String> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:width$}", h, width = widths[i]))
            .collect();
        println!("{}", header_line.join(" | "));

        // Print separator
        let sep: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
        println!("{}", sep.join("-+-"));
    }

    for record in &records {
        let line: Vec<String> = record
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let width = widths.get(i).copied().unwrap_or(0);
                format!("{:width$}", f, width = width)
            })
            .collect();
        println!("{}", line.join(" | "));
    }

    Ok(())
}

fn cmd_to_json(input: Option<PathBuf>, delimiter: char, pretty: bool) -> Result<()> {
    let content = read_input(input)?;

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(true)
        .from_reader(content.as_bytes());

    let headers: Vec<String> = rdr
        .headers()
        .context("Failed to read CSV headers")?
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut result: Vec<HashMap<String, serde_json::Value>> = Vec::new();

    for record in rdr.records() {
        let record = record.context("Failed to read CSV record")?;
        let mut obj: HashMap<String, serde_json::Value> = HashMap::new();

        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                // Try to parse as number, otherwise use string
                let value = if let Ok(n) = field.parse::<i64>() {
                    serde_json::Value::Number(n.into())
                } else if let Ok(n) = field.parse::<f64>() {
                    serde_json::Number::from_f64(n)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::String(field.to_string()))
                } else if field == "true" {
                    serde_json::Value::Bool(true)
                } else if field == "false" {
                    serde_json::Value::Bool(false)
                } else {
                    serde_json::Value::String(field.to_string())
                };
                obj.insert(header.clone(), value);
            }
        }
        result.push(obj);
    }

    let output = if pretty {
        serde_json::to_string_pretty(&result)?
    } else {
        serde_json::to_string(&result)?
    };

    println!("{}", output);
    Ok(())
}

fn cmd_from_json(input: Option<PathBuf>) -> Result<()> {
    let content = read_input(input)?;

    let data: Vec<HashMap<String, serde_json::Value>> =
        serde_json::from_str(&content).context("Failed to parse JSON array")?;

    if data.is_empty() {
        return Ok(());
    }

    // Collect all headers from all objects
    let mut headers: Vec<String> = Vec::new();
    for obj in &data {
        for key in obj.keys() {
            if !headers.contains(key) {
                headers.push(key.clone());
            }
        }
    }
    headers.sort();

    let mut wtr = csv::Writer::from_writer(io::stdout());

    // Write header
    wtr.write_record(&headers)?;

    // Write records
    for obj in &data {
        let record: Vec<String> = headers
            .iter()
            .map(|h| {
                obj.get(h)
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Null => String::new(),
                        other => other.to_string(),
                    })
                    .unwrap_or_default()
            })
            .collect();
        wtr.write_record(&record)?;
    }

    wtr.flush()?;
    Ok(())
}

fn cmd_query(input: Option<PathBuf>, columns: Vec<String>, delimiter: char) -> Result<()> {
    let content = read_input(input)?;

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(true)
        .from_reader(content.as_bytes());

    let headers: Vec<String> = rdr
        .headers()
        .context("Failed to read CSV headers")?
        .iter()
        .map(|s| s.to_string())
        .collect();

    // Resolve column names/indices to indices
    let column_indices: Vec<usize> = columns
        .iter()
        .filter_map(|c| {
            // Try as index first
            if let Ok(idx) = c.parse::<usize>() {
                if idx < headers.len() {
                    return Some(idx);
                }
            }
            // Try as column name
            headers.iter().position(|h| h == c)
        })
        .collect();

    if column_indices.is_empty() {
        anyhow::bail!("No valid columns found");
    }

    let mut wtr = csv::Writer::from_writer(io::stdout());

    // Write selected headers
    let selected_headers: Vec<&str> = column_indices
        .iter()
        .map(|&i| headers[i].as_str())
        .collect();
    wtr.write_record(&selected_headers)?;

    // Write selected columns from each record
    for record in rdr.records() {
        let record = record.context("Failed to read CSV record")?;
        let selected: Vec<&str> = column_indices
            .iter()
            .filter_map(|&i| record.get(i))
            .collect();
        wtr.write_record(&selected)?;
    }

    wtr.flush()?;
    Ok(())
}
