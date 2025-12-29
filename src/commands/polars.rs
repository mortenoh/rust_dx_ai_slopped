//! Polars command - DataFrame operations and data analysis demos.

use crate::cli::commands::polars::{PolarsArgs, PolarsCommand};
use anyhow::{Context, Result};
use colored::Colorize;
use polars::prelude::*;
use std::path::Path;
use std::time::Instant;

/// File format enum
#[derive(Debug, Clone, Copy)]
enum Format {
    Csv,
    Parquet,
}

/// Configuration for random data generation
struct RandomConfig<'a> {
    output: &'a Path,
    rows: usize,
    columns: &'a [String],
    categories: usize,
    string_len: usize,
    min: i64,
    max: i64,
    null_prob: f64,
    seed: Option<u64>,
}

/// Detect file format from extension
fn detect_format(path: &Path) -> Format {
    match path.extension().and_then(|e| e.to_str()) {
        Some("parquet" | "pq") => Format::Parquet,
        _ => Format::Csv,
    }
}

/// Run the polars command
pub fn run(args: PolarsArgs) -> Result<()> {
    match args.command {
        PolarsCommand::Random {
            file,
            rows,
            columns,
            categories,
            string_len,
            min,
            max,
            null_prob,
            seed,
        } => cmd_random(RandomConfig {
            output: &file,
            rows,
            columns: &columns,
            categories,
            string_len,
            min,
            max,
            null_prob,
            seed,
        }),
    }
}

/// Generate random data file
fn cmd_random(config: RandomConfig) -> Result<()> {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    let start = Instant::now();

    // Initialize RNG with optional seed
    let mut rng: Box<dyn rand::RngCore> = if let Some(s) = config.seed {
        Box::new(StdRng::seed_from_u64(s))
    } else {
        Box::new(rand::rng())
    };

    // Default columns if none specified
    let col_defs: Vec<(&str, &str)> = if config.columns.is_empty() {
        vec![
            ("id", "int"),
            ("name", "string"),
            ("value", "float"),
            ("category", "category"),
            ("active", "bool"),
            ("created_at", "date"),
        ]
    } else {
        config
            .columns
            .iter()
            .map(|c| {
                let parts: Vec<&str> = c.split(':').collect();
                if parts.len() == 2 {
                    (parts[0], parts[1])
                } else {
                    (c.as_str(), "string")
                }
            })
            .collect()
    };

    // Generate category values
    let category_values: Vec<String> = (0..config.categories)
        .map(|i| format!("cat_{}", i))
        .collect();

    // Generate columns
    let mut series_vec: Vec<Series> = Vec::new();

    for (name, col_type) in &col_defs {
        let series = match *col_type {
            "int" | "integer" | "i64" => {
                let values: Vec<Option<i64>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(rng.random_range(config.min..=config.max))
                        }
                    })
                    .collect();
                Series::new((*name).into(), values)
            }
            "float" | "f64" | "double" => {
                let values: Vec<Option<f64>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(rng.random_range(config.min as f64..=config.max as f64))
                        }
                    })
                    .collect();
                Series::new((*name).into(), values)
            }
            "string" | "str" | "text" => {
                let values: Vec<Option<String>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(generate_random_string(&mut *rng, config.string_len))
                        }
                    })
                    .collect();
                Series::new((*name).into(), values)
            }
            "bool" | "boolean" => {
                let values: Vec<Option<bool>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(rng.random_bool(0.5))
                        }
                    })
                    .collect();
                Series::new((*name).into(), values)
            }
            "category" | "cat" | "enum" => {
                let values: Vec<Option<&str>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(
                                category_values[rng.random_range(0..category_values.len())]
                                    .as_str(),
                            )
                        }
                    })
                    .collect();
                Series::new((*name).into(), values)
            }
            "date" => {
                // Generate dates in 2020-2025 range
                let base_date = 18262i32; // 2020-01-01 as days since epoch
                let values: Vec<Option<i32>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(base_date + rng.random_range(0..1826)) // ~5 years
                        }
                    })
                    .collect();
                Series::new((*name).into(), values).cast(&DataType::Date)?
            }
            _ => {
                // Default to string
                let values: Vec<Option<String>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(generate_random_string(&mut *rng, config.string_len))
                        }
                    })
                    .collect();
                Series::new((*name).into(), values)
            }
        };
        series_vec.push(series);
    }

    // Convert Vec<Series> to Vec<Column>
    let columns_vec: Vec<Column> = series_vec.into_iter().map(|s| s.into()).collect();
    let df = DataFrame::new(columns_vec)?;
    let gen_time = start.elapsed();

    // Write to file
    let write_start = Instant::now();
    let format = detect_format(config.output);
    match format {
        Format::Parquet => {
            let file = std::fs::File::create(config.output)
                .with_context(|| format!("Failed to create file: {}", config.output.display()))?;
            ParquetWriter::new(file)
                .finish(&mut df.clone())
                .with_context(|| "Failed to write Parquet")?;
        }
        Format::Csv => {
            let file = std::fs::File::create(config.output)
                .with_context(|| format!("Failed to create file: {}", config.output.display()))?;
            CsvWriter::new(file)
                .finish(&mut df.clone())
                .with_context(|| "Failed to write CSV")?;
        }
    }
    let write_time = write_start.elapsed();

    // Get file size
    let file_size = std::fs::metadata(config.output)
        .map(|m| m.len())
        .unwrap_or(0);

    // Print summary
    println!("{}", "Random Data Generated".green().bold());
    println!("{}", "═".repeat(50));
    println!();
    println!("{}: {}", "Output".yellow(), config.output.display());
    println!("{}: {:?}", "Format".yellow(), format);
    println!("{}: {}", "Rows".yellow(), format_number(config.rows));
    println!("{}: {}", "Columns".yellow(), df.width());
    println!("{}: {}", "File size".yellow(), format_bytes(file_size));
    println!();

    println!("{}", "Schema:".cyan().bold());
    for col in df.get_columns() {
        println!(
            "  {} : {}",
            col.name().to_string().white(),
            format!("{:?}", col.dtype()).dimmed()
        );
    }
    println!();

    println!("{}: {:.2?}", "Generation time".dimmed(), gen_time);
    println!("{}: {:.2?}", "Write time".dimmed(), write_time);

    Ok(())
}

/// Generate a random alphanumeric string
fn generate_random_string(rng: &mut dyn rand::RngCore, len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    (0..len)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Format a number with thousand separators
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// Format bytes in human readable form
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
