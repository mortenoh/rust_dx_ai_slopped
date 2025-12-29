//! Polars command - DataFrame operations and data analysis demos.

use crate::cli::commands::polars::{PolarsArgs, PolarsCommand, PolarsOutputFormat};
use anyhow::{Context, Result};
use colored::Colorize;
use dx_datagen::{categories, generators, geo, network, numeric, password, personal, text, uuid};
use polars::prelude::*;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};
use std::path::Path;
use std::time::Instant;

/// File format enum
#[derive(Debug, Clone, Copy)]
enum Format {
    Csv,
    Parquet,
}

/// Configuration for viewing data
struct ViewConfig<'a> {
    file: &'a Path,
    rows: usize,
    tail: bool,
    columns: &'a [String],
    schema_only: bool,
    stats: bool,
    json: bool,
}

/// Configuration for random data generation
struct RandomConfig<'a> {
    output: Option<&'a Path>,
    rows: usize,
    columns: &'a [String],
    categories: usize,
    string_len: usize,
    min: i64,
    max: i64,
    null_prob: f64,
    seed: Option<u64>,
    format: PolarsOutputFormat,
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
        PolarsCommand::View {
            file,
            rows,
            tail,
            columns,
            schema,
            stats,
            json,
        } => cmd_view(ViewConfig {
            file: &file,
            rows,
            tail,
            columns: &columns,
            schema_only: schema,
            stats,
            json,
        }),
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
            format,
        } => cmd_random(RandomConfig {
            output: file.as_deref(),
            rows,
            columns: &columns,
            categories,
            string_len,
            min,
            max,
            null_prob,
            seed,
            format,
        }),
    }
}

/// Read a DataFrame from file
fn read_df(path: &Path) -> Result<DataFrame> {
    let format = detect_format(path);
    match format {
        Format::Csv => CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()
            .with_context(|| format!("Failed to read CSV: {}", path.display())),
        Format::Parquet => ParquetReader::new(std::fs::File::open(path)?)
            .finish()
            .with_context(|| format!("Failed to read Parquet: {}", path.display())),
    }
}

/// View data from file
fn cmd_view(config: ViewConfig) -> Result<()> {
    let start = Instant::now();
    let df = read_df(config.file)?;
    let read_time = start.elapsed();

    // Get file metadata
    let file_size = std::fs::metadata(config.file).map(|m| m.len()).unwrap_or(0);
    let format = detect_format(config.file);

    // Select columns if specified
    let df = if config.columns.is_empty() {
        df
    } else {
        df.select(config.columns.iter().map(|s| s.as_str()))
            .with_context(|| "Failed to select columns")?
    };

    // Schema only mode
    if config.schema_only {
        if config.json {
            let schema_json: Vec<serde_json::Value> = df
                .get_columns()
                .iter()
                .map(|col| {
                    serde_json::json!({
                        "name": col.name().to_string(),
                        "dtype": format!("{:?}", col.dtype()),
                        "null_count": col.null_count(),
                    })
                })
                .collect();
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "file": config.file.display().to_string(),
                    "format": format!("{:?}", format),
                    "rows": df.height(),
                    "columns": df.width(),
                    "file_size": file_size,
                    "schema": schema_json,
                }))?
            );
        } else {
            println!("{}", "Schema".cyan().bold());
            println!("{}", "═".repeat(50));
            println!();
            println!("{}: {}", "File".yellow(), config.file.display());
            println!("{}: {:?}", "Format".yellow(), format);
            println!("{}: {}", "Rows".yellow(), format_number(df.height()));
            println!("{}: {}", "Columns".yellow(), df.width());
            println!("{}: {}", "Size".yellow(), format_bytes(file_size));
            println!();
            println!("{}", "Columns:".white().bold());
            for col in df.get_columns() {
                let null_count = col.null_count();
                let null_pct = if df.height() > 0 {
                    (null_count as f64 / df.height() as f64) * 100.0
                } else {
                    0.0
                };
                println!(
                    "  {:20} {:15} {} nulls ({:.1}%)",
                    col.name().to_string().white(),
                    format!("{:?}", col.dtype()).dimmed(),
                    null_count,
                    null_pct
                );
            }
        }
        return Ok(());
    }

    // Stats mode
    if config.stats {
        return cmd_view_stats(&df, config.file, format, file_size, config.json);
    }

    // Get rows to display
    let display_df = if config.tail {
        df.tail(Some(config.rows))
    } else {
        df.head(Some(config.rows))
    };

    // JSON output
    if config.json {
        let mut rows_json: Vec<serde_json::Value> = Vec::new();
        for i in 0..display_df.height() {
            let mut row: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
            for col in display_df.get_columns() {
                let series = col.as_materialized_series();
                let value = format_value_json(series, i);
                row.insert(col.name().to_string(), value);
            }
            rows_json.push(serde_json::Value::Object(row));
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "file": config.file.display().to_string(),
                "total_rows": df.height(),
                "displayed_rows": display_df.height(),
                "columns": df.width(),
                "data": rows_json,
            }))?
        );
        return Ok(());
    }

    // Use ratatui for table rendering
    let title = config.file.display().to_string();
    render_dataframe_table(&display_df, display_df.height(), &title)?;

    if df.height() > config.rows {
        let showing = if config.tail { "last" } else { "first" };
        println!(
            "{}",
            format!(
                "Showing {} {} of {} rows",
                showing,
                config.rows,
                format_number(df.height())
            )
            .dimmed()
        );
    }
    println!("{}", format!("Read time: {:.2?}", read_time).dimmed());

    Ok(())
}

/// Show statistics for the dataframe
fn cmd_view_stats(
    df: &DataFrame,
    file: &Path,
    format: Format,
    file_size: u64,
    json_output: bool,
) -> Result<()> {
    if json_output {
        let stats: Vec<serde_json::Value> = df
            .get_columns()
            .iter()
            .map(|col| {
                let series = col.as_materialized_series();
                let mut stat = serde_json::json!({
                    "name": col.name().to_string(),
                    "dtype": format!("{:?}", col.dtype()),
                    "count": series.len(),
                    "null_count": series.null_count(),
                });
                if series.dtype().is_numeric() {
                    if let Some(mean) = series.mean() {
                        stat["mean"] = serde_json::json!(mean);
                    }
                    if let Some(min) = series.min::<f64>().ok().flatten() {
                        stat["min"] = serde_json::json!(min);
                    }
                    if let Some(max) = series.max::<f64>().ok().flatten() {
                        stat["max"] = serde_json::json!(max);
                    }
                }
                stat
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "file": file.display().to_string(),
                "format": format!("{:?}", format),
                "rows": df.height(),
                "columns": df.width(),
                "file_size": file_size,
                "statistics": stats,
            }))?
        );
        return Ok(());
    }

    println!("{}", "Statistics".cyan().bold());
    println!("{}", "═".repeat(70));
    println!();
    println!("{}: {}", "File".yellow(), file.display());
    println!("{}: {:?}", "Format".yellow(), format);
    println!("{}: {}", "Rows".yellow(), format_number(df.height()));
    println!("{}: {}", "Columns".yellow(), df.width());
    println!("{}: {}", "Size".yellow(), format_bytes(file_size));
    println!();

    // Print stats table header
    println!(
        "{:20} {:10} {:>10} {:>12} {:>12} {:>12}",
        "Column".white().bold(),
        "Type".white().bold(),
        "Nulls".white().bold(),
        "Mean".white().bold(),
        "Min".white().bold(),
        "Max".white().bold()
    );
    println!("{}", "─".repeat(70));

    for col in df.get_columns() {
        let series = col.as_materialized_series();
        let dtype = format!("{:?}", col.dtype());
        let nulls = series.null_count();

        let (mean, min, max) = if series.dtype().is_numeric() {
            (
                series.mean().map(|v| format!("{:.2}", v)),
                series
                    .min::<f64>()
                    .ok()
                    .flatten()
                    .map(|v| format!("{:.2}", v)),
                series
                    .max::<f64>()
                    .ok()
                    .flatten()
                    .map(|v| format!("{:.2}", v)),
            )
        } else {
            (None, None, None)
        };

        println!(
            "{:20} {:10} {:>10} {:>12} {:>12} {:>12}",
            col.name().to_string().truncate_str(20),
            dtype.truncate_str(10).dimmed(),
            nulls,
            mean.unwrap_or_else(|| "-".to_string()),
            min.unwrap_or_else(|| "-".to_string()),
            max.unwrap_or_else(|| "-".to_string()),
        );
    }

    Ok(())
}

/// Format a value for JSON output
fn format_value_json(series: &Series, idx: usize) -> serde_json::Value {
    let value = series.get(idx);
    if value.is_err() || matches!(value.as_ref().ok(), Some(AnyValue::Null)) {
        return serde_json::Value::Null;
    }

    match series.dtype() {
        DataType::Float64 => series
            .f64()
            .ok()
            .and_then(|v| v.get(idx))
            .map(|v| serde_json::json!(v))
            .unwrap_or(serde_json::Value::Null),
        DataType::Float32 => series
            .f32()
            .ok()
            .and_then(|v| v.get(idx))
            .map(|v| serde_json::json!(v))
            .unwrap_or(serde_json::Value::Null),
        DataType::Int64 => series
            .i64()
            .ok()
            .and_then(|v| v.get(idx))
            .map(|v| serde_json::json!(v))
            .unwrap_or(serde_json::Value::Null),
        DataType::Int32 => series
            .i32()
            .ok()
            .and_then(|v| v.get(idx))
            .map(|v| serde_json::json!(v))
            .unwrap_or(serde_json::Value::Null),
        DataType::Boolean => series
            .bool()
            .ok()
            .and_then(|v| v.get(idx))
            .map(|v| serde_json::json!(v))
            .unwrap_or(serde_json::Value::Null),
        DataType::String => series
            .str()
            .ok()
            .and_then(|v| v.get(idx))
            .map(|v| {
                // Try to parse as JSON if it looks like JSON (starts with { or [)
                if (v.starts_with('{') && v.ends_with('}'))
                    || (v.starts_with('[') && v.ends_with(']'))
                {
                    serde_json::from_str(v).unwrap_or_else(|_| serde_json::json!(v))
                } else {
                    serde_json::json!(v)
                }
            })
            .unwrap_or(serde_json::Value::Null),
        _ => series
            .get(idx)
            .map(|v| serde_json::json!(format!("{}", v)))
            .unwrap_or(serde_json::Value::Null),
    }
}

/// Trait for truncating strings with ellipsis
trait TruncateStr {
    fn truncate_str(&self, max_len: usize) -> String;
}

impl TruncateStr for str {
    fn truncate_str(&self, max_len: usize) -> String {
        if self.len() <= max_len {
            self.to_string()
        } else if max_len > 1 {
            format!("{}…", &self[..max_len - 1])
        } else {
            "…".to_string()
        }
    }
}

impl TruncateStr for String {
    fn truncate_str(&self, max_len: usize) -> String {
        self.as_str().truncate_str(max_len)
    }
}

/// Generate random data file
fn cmd_random(config: RandomConfig) -> Result<()> {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    // Check if user wants to list available generators
    if config.columns.len() == 1 {
        let first = config.columns[0].to_lowercase();
        if first == "list" || first == "help" || first == "?" {
            print_available_generators();
            return Ok(());
        }
    }

    let start = Instant::now();

    // Initialize RNG with optional seed
    let mut rng: Box<dyn rand::RngCore> = if let Some(s) = config.seed {
        Box::new(StdRng::seed_from_u64(s))
    } else {
        Box::new(rand::rng())
    };

    // Column definition with optional parameters
    struct ColDef<'a> {
        name: &'a str,
        col_type: &'a str,
        params: Vec<&'a str>,
    }

    // Parse column type and extract parameters: "int[0;100]" -> ("int", ["0", "100"])
    fn parse_col_type(type_str: &str) -> (&str, Vec<&str>) {
        if let Some(bracket_start) = type_str.find('[') {
            if let Some(bracket_end) = type_str.find(']') {
                let base_type = &type_str[..bracket_start];
                let params_str = &type_str[bracket_start + 1..bracket_end];
                // Use semicolon as separator to avoid conflict with name:type colon
                let params: Vec<&str> = params_str.split(';').map(|s| s.trim()).collect();
                return (base_type, params);
            }
        }
        (type_str, vec![])
    }

    // Default columns if none specified
    let col_defs: Vec<ColDef> = if config.columns.is_empty() {
        vec![
            ColDef {
                name: "id",
                col_type: "id",
                params: vec![],
            },
            ColDef {
                name: "store",
                col_type: "city",
                params: vec![],
            },
            ColDef {
                name: "item",
                col_type: "fruit",
                params: vec![],
            },
            ColDef {
                name: "value",
                col_type: "int",
                params: vec![],
            },
        ]
    } else {
        config
            .columns
            .iter()
            .map(|c| {
                // Split only on the first colon to handle name:type[params]
                if let Some(colon_pos) = c.find(':') {
                    let name = &c[..colon_pos];
                    let type_str = &c[colon_pos + 1..];
                    let (col_type, params) = parse_col_type(type_str);
                    ColDef {
                        name,
                        col_type,
                        params,
                    }
                } else {
                    ColDef {
                        name: c.as_str(),
                        col_type: "string",
                        params: vec![],
                    }
                }
            })
            .collect()
    };

    // Generate category values
    let category_values: Vec<String> = (0..config.categories)
        .map(|i| format!("cat_{}", i))
        .collect();

    // Track ID columns for sorting
    let mut id_columns: Vec<&str> = Vec::new();

    // Generate columns
    let mut series_vec: Vec<Series> = Vec::new();

    for col_def in &col_defs {
        let name = col_def.name;
        let col_type = col_def.col_type;
        let params = &col_def.params;

        let series = match col_type {
            "id" => {
                // Sequential ID column (1, 2, 3, ..., n) - never null
                id_columns.push(name);
                // Support id(start) or id(start, step)
                let start: i64 = params.first().and_then(|s| s.parse().ok()).unwrap_or(1);
                let step: i64 = params.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
                let values: Vec<i64> = (0..config.rows as i64).map(|i| start + i * step).collect();
                Series::new(name.into(), values)
            }
            "int" | "integer" | "i64" => {
                // Support int(min, max)
                let min: i64 = params
                    .first()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(config.min);
                let max: i64 = params
                    .get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(config.max);
                let values: Vec<Option<i64>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(rng.random_range(min..=max))
                        }
                    })
                    .collect();
                Series::new(name.into(), values)
            }
            "float" | "f64" | "double" => {
                // Support float(min, max)
                let min: f64 = params
                    .first()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(config.min as f64);
                let max: f64 = params
                    .get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(config.max as f64);
                let values: Vec<Option<f64>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(rng.random_range(min..=max))
                        }
                    })
                    .collect();
                Series::new(name.into(), values)
            }
            "string" | "str" | "text" => {
                // Support string(length)
                let len: usize = params
                    .first()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(config.string_len);
                let values: Vec<Option<String>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(generators::alphanumeric(&mut *rng, len))
                        }
                    })
                    .collect();
                Series::new(name.into(), values)
            }
            "bool" | "boolean" => {
                // Support bool(probability) where probability is chance of true (0.0-1.0)
                let prob: f64 = params.first().and_then(|s| s.parse().ok()).unwrap_or(0.5);
                let values: Vec<Option<bool>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(rng.random_bool(prob))
                        }
                    })
                    .collect();
                Series::new((*name).into(), values)
            }
            "category" | "cat" | "enum" => {
                // Generic category using --categories count
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
            "fruit" => generate_category_series(
                name,
                categories::FRUITS,
                config.rows,
                config.null_prob,
                &mut *rng,
            ),
            "color" => generate_category_series(
                name,
                categories::COLORS,
                config.rows,
                config.null_prob,
                &mut *rng,
            ),
            "city" => generate_category_series(
                name,
                categories::CITIES,
                config.rows,
                config.null_prob,
                &mut *rng,
            ),
            "country" => generate_category_series(
                name,
                categories::COUNTRIES,
                config.rows,
                config.null_prob,
                &mut *rng,
            ),
            "status" => generate_category_series(
                name,
                categories::STATUSES,
                config.rows,
                config.null_prob,
                &mut *rng,
            ),
            "priority" => generate_category_series(
                name,
                categories::PRIORITIES,
                config.rows,
                config.null_prob,
                &mut *rng,
            ),
            "department" | "dept" => generate_category_series(
                name,
                categories::DEPARTMENTS,
                config.rows,
                config.null_prob,
                &mut *rng,
            ),
            "day" => generate_category_series(
                name,
                categories::DAYS,
                config.rows,
                config.null_prob,
                &mut *rng,
            ),
            "size" => generate_category_series(
                name,
                categories::SIZES,
                config.rows,
                config.null_prob,
                &mut *rng,
            ),
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

            // === Personal data ===
            "first_name" | "firstname" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    personal::names::first_name(r).to_string()
                })
            }
            "last_name" | "lastname" | "surname" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    personal::names::last_name(r).to_string()
                })
            }
            "full_name" | "fullname" | "name" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    personal::names::full_name(r)
                })
            }
            "email" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    personal::email::email(r)
                })
            }
            "username" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    personal::username::username(r)
                })
            }
            "phone" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    personal::phone::phone(r)
                })
            }
            "address" | "street_address" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    personal::address::street_address(r)
                })
            }
            "zip" | "zip_code" | "postal_code" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    personal::address::zip_code(r)
                })
            }

            // === Network data ===
            "ipv4" | "ip" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    network::ip::ipv4(r).to_string()
                })
            }
            "ipv6" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                network::ip::ipv6(r).to_string()
            }),
            "mac" | "mac_address" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    network::mac::mac_address(r)
                })
            }
            "domain" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    network::domain::domain(r)
                })
            }
            "url" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                network::url::url(r)
            }),

            // === Numeric identifiers ===
            "credit_card" | "cc" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    numeric::credit_card::credit_card(r)
                })
            }
            "isbn" | "isbn13" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    numeric::isbn::isbn13(r)
                })
            }
            "isbn10" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    numeric::isbn::isbn10(r)
                })
            }
            "iban" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                numeric::iban::iban(r)
            }),
            "ssn" | "ssn_us" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    numeric::ssn::ssn_us(r)
                })
            }
            "ssn_no" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    numeric::ssn::ssn_no(r)
                })
            }

            // === Text data ===
            "word" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                text::words::word(r).to_string()
            }),
            "sentence" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    text::lorem::sentence(r)
                })
            }
            "paragraph" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    text::lorem::paragraph(r)
                })
            }

            // === UUID ===
            "uuid" | "uuid4" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |_| {
                    uuid::v4().to_string()
                })
            }
            "uuid7" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |_| {
                    uuid::v7().to_string()
                })
            }

            // === Other ===
            "password" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    password::password(r, 16, true)
                })
            }
            "hex" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                generators::hex_string(r, 16)
            }),

            // === Geographic data ===
            "lat" | "latitude" => {
                // Support lat[min;max]
                let min: f64 = params.first().and_then(|s| s.parse().ok()).unwrap_or(-90.0);
                let max: f64 = params.get(1).and_then(|s| s.parse().ok()).unwrap_or(90.0);
                let values: Vec<Option<f64>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(geo::latitude_in_range(&mut *rng, min, max))
                        }
                    })
                    .collect();
                Series::new(name.into(), values)
            }
            "lon" | "longitude" => {
                // Support lon[min;max]
                let min: f64 = params
                    .first()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(-180.0);
                let max: f64 = params.get(1).and_then(|s| s.parse().ok()).unwrap_or(180.0);
                let values: Vec<Option<f64>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(geo::longitude_in_range(&mut *rng, min, max))
                        }
                    })
                    .collect();
                Series::new(name.into(), values)
            }
            "coords" | "coordinates" => {
                // Support coords[minLon;minLat;maxLon;maxLat]
                let bbox: [f64; 4] = [
                    params
                        .first()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(-180.0),
                    params.get(1).and_then(|s| s.parse().ok()).unwrap_or(-90.0),
                    params.get(2).and_then(|s| s.parse().ok()).unwrap_or(180.0),
                    params.get(3).and_then(|s| s.parse().ok()).unwrap_or(90.0),
                ];
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    geo::coordinates_in_bounds_string(r, bbox)
                })
            }
            "point" | "geojson_point" | "geojson" => {
                // Support point[minLon;minLat;maxLon;maxLat]
                let bbox: [f64; 4] = [
                    params
                        .first()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(-180.0),
                    params.get(1).and_then(|s| s.parse().ok()).unwrap_or(-90.0),
                    params.get(2).and_then(|s| s.parse().ok()).unwrap_or(180.0),
                    params.get(3).and_then(|s| s.parse().ok()).unwrap_or(90.0),
                ];
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    geo::geojson_point_in_bounds_string(r, bbox)
                })
            }

            _ => {
                // Default to string
                let values: Vec<Option<String>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(generators::alphanumeric(&mut *rng, config.string_len))
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
    let mut df = DataFrame::new(columns_vec)?;
    let gen_time = start.elapsed();

    // Sort by ID columns if any exist
    if !id_columns.is_empty() {
        df = df.sort(id_columns.iter().copied(), SortMultipleOptions::default())?;
    }

    // Either write to file or display on screen
    match config.output {
        Some(output_path) => {
            // Write to file
            let write_start = Instant::now();
            let format = detect_format(output_path);
            match format {
                Format::Parquet => {
                    let file = std::fs::File::create(output_path).with_context(|| {
                        format!("Failed to create file: {}", output_path.display())
                    })?;
                    ParquetWriter::new(file)
                        .finish(&mut df)
                        .with_context(|| "Failed to write Parquet")?;
                }
                Format::Csv => {
                    let file = std::fs::File::create(output_path).with_context(|| {
                        format!("Failed to create file: {}", output_path.display())
                    })?;
                    CsvWriter::new(file)
                        .finish(&mut df)
                        .with_context(|| "Failed to write CSV")?;
                }
            }
            let write_time = write_start.elapsed();

            // Get file size
            let file_size = std::fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);

            // Print summary
            println!("{}", "Random Data Generated".green().bold());
            println!("{}", "═".repeat(50));
            println!();
            println!("{}: {}", "Output".yellow(), output_path.display());
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
        }
        None => {
            // Display on screen based on format
            match config.format {
                PolarsOutputFormat::Csv => {
                    // Output as CSV to stdout
                    let mut stdout = std::io::stdout();
                    CsvWriter::new(&mut stdout)
                        .finish(&mut df)
                        .with_context(|| "Failed to write CSV to stdout")?;
                }
                PolarsOutputFormat::Json => {
                    // Output as JSON array of objects
                    let rows_json: Vec<serde_json::Value> = (0..df.height())
                        .map(|i| {
                            let mut row: serde_json::Map<String, serde_json::Value> =
                                serde_json::Map::new();
                            for col in df.get_columns() {
                                let series = col.as_materialized_series();
                                let value = format_value_json(series, i);
                                row.insert(col.name().to_string(), value);
                            }
                            serde_json::Value::Object(row)
                        })
                        .collect();
                    println!("{}", serde_json::to_string_pretty(&rows_json)?);
                }
                PolarsOutputFormat::Jsonl => {
                    // Output as JSON Lines (one object per line)
                    for i in 0..df.height() {
                        let mut row: serde_json::Map<String, serde_json::Value> =
                            serde_json::Map::new();
                        for col in df.get_columns() {
                            let series = col.as_materialized_series();
                            let value = format_value_json(series, i);
                            row.insert(col.name().to_string(), value);
                        }
                        println!(
                            "{}",
                            serde_json::to_string(&serde_json::Value::Object(row))?
                        );
                    }
                }
                PolarsOutputFormat::Table => {
                    // Display as table (default)
                    display_dataframe(&df, config.rows)?;
                    println!();
                    println!("{}: {:.2?}", "Generation time".dimmed(), gen_time);
                }
            }
        }
    }

    Ok(())
}

/// Display a DataFrame on screen using ratatui Table
fn display_dataframe(df: &DataFrame, rows: usize) -> Result<()> {
    render_dataframe_table(df, rows, "Generated Data")
}

/// Render a DataFrame as a ratatui table
fn render_dataframe_table(df: &DataFrame, rows: usize, title: &str) -> Result<()> {
    let display_df = df.head(Some(rows));

    // Build header row
    let header_cells: Vec<Cell> = display_df
        .get_columns()
        .iter()
        .map(|col| {
            Cell::from(col.name().to_string()).style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
        })
        .collect();
    let header = Row::new(header_cells).style(Style::default()).height(1);

    // Build data rows
    let data_rows: Vec<Row> = (0..display_df.height())
        .map(|i| {
            let cells: Vec<Cell> = display_df
                .get_columns()
                .iter()
                .map(|col| {
                    let val = format_value_plain(col.as_materialized_series(), i);
                    let style = get_value_style(col.as_materialized_series(), i);
                    Cell::from(val).style(style)
                })
                .collect();
            Row::new(cells)
        })
        .collect();

    // Calculate column widths based on terminal width
    let col_count = display_df.width();
    let (term_width, _) = crossterm::terminal::size().unwrap_or((120, 24));
    let available_width = term_width.saturating_sub(4) as usize; // Account for borders

    // First pass: calculate ideal widths for each column
    let ideal_widths: Vec<usize> = (0..col_count)
        .map(|i| {
            let col = &display_df.get_columns()[i];
            let header_width = col.name().len();
            let max_value_width = (0..display_df.height())
                .map(|j| format_value_plain(col.as_materialized_series(), j).len())
                .max()
                .unwrap_or(0);
            header_width.max(max_value_width) + 2
        })
        .collect();

    let total_ideal: usize = ideal_widths.iter().sum();

    // Second pass: distribute available width proportionally
    let widths: Vec<Constraint> = if total_ideal <= available_width {
        // All columns fit - use ideal widths
        ideal_widths
            .iter()
            .map(|&w| Constraint::Length(w as u16))
            .collect()
    } else {
        // Need to shrink - distribute proportionally with minimum of 10 chars
        ideal_widths
            .iter()
            .map(|&ideal| {
                let proportion = ideal as f64 / total_ideal as f64;
                let width = (proportion * available_width as f64) as usize;
                Constraint::Length(width.max(10) as u16)
            })
            .collect()
    };

    // Build the table widget
    let block_title = format!(
        "{} ({} rows, {} cols)",
        title,
        format_number(df.height()),
        df.width()
    );
    let table = Table::new(data_rows, &widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(block_title)
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .row_highlight_style(Style::default().add_modifier(Modifier::BOLD));

    // Render to terminal using inline viewport
    let table_height = (display_df.height() + 3) as u16; // +3 for header, borders
    render_widget_inline(table, table_height)?;

    if df.height() > rows {
        println!(
            "{}",
            format!(
                "Showing first {} of {} rows",
                rows,
                format_number(df.height())
            )
            .dimmed()
        );
    }

    Ok(())
}

/// Render a ratatui widget inline (without taking over the screen)
fn render_widget_inline<W: ratatui::widgets::Widget>(widget: W, height: u16) -> Result<()> {
    use ratatui::layout::Rect;

    // Get terminal width
    let (width, _) = crossterm::terminal::size().unwrap_or((80, 24));

    // Create a buffer and render the widget to it
    let area = Rect::new(0, 0, width, height);
    let mut buffer = ratatui::buffer::Buffer::empty(area);
    widget.render(area, &mut buffer);

    // Print the buffer contents line by line
    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            let cell = buffer.cell((x, y)).unwrap();
            // Apply styling using crossterm
            let mut styled = cell.symbol().to_string();
            if cell.fg != Color::Reset {
                styled = apply_color(&styled, cell.fg);
            }
            if cell.modifier.contains(Modifier::BOLD) {
                styled = styled.bold().to_string();
            }
            if cell.modifier.contains(Modifier::DIM) {
                styled = styled.dimmed().to_string();
            }
            line.push_str(&styled);
        }
        println!("{}", line.trim_end());
    }

    Ok(())
}

/// Apply ratatui Color to a string using colored crate
fn apply_color(s: &str, color: Color) -> String {
    match color {
        Color::Cyan => s.cyan().to_string(),
        Color::Green => s.green().to_string(),
        Color::Red => s.red().to_string(),
        Color::Yellow => s.yellow().to_string(),
        Color::Blue => s.blue().to_string(),
        Color::Magenta => s.magenta().to_string(),
        Color::White => s.white().to_string(),
        Color::DarkGray => s.dimmed().to_string(),
        Color::Gray => s.dimmed().to_string(),
        _ => s.to_string(),
    }
}

/// Get plain value string (no color codes)
fn format_value_plain(series: &Series, idx: usize) -> String {
    let value = series.get(idx);
    if value.is_err() || matches!(value.as_ref().ok(), Some(AnyValue::Null)) {
        return "null".to_string();
    }

    match series.dtype() {
        DataType::Float64 | DataType::Float32 => {
            if let Ok(val) = series.f64() {
                if let Some(v) = val.get(idx) {
                    return format!("{:.4}", v);
                }
            }
            if let Ok(val) = series.f32() {
                if let Some(v) = val.get(idx) {
                    return format!("{:.4}", v);
                }
            }
            "-".to_string()
        }
        DataType::Boolean => {
            if let Ok(val) = series.bool() {
                if let Some(v) = val.get(idx) {
                    return if v { "true" } else { "false" }.to_string();
                }
            }
            "-".to_string()
        }
        DataType::String => {
            let val = series.str().ok().and_then(|s| s.get(idx));
            match val {
                Some(s) => s.to_string(),
                None => "-".to_string(),
            }
        }
        _ => series
            .get(idx)
            .map(|v| format!("{}", v))
            .unwrap_or_default(),
    }
}

/// Get style for a value based on its type
fn get_value_style(series: &Series, idx: usize) -> Style {
    let value = series.get(idx);
    if value.is_err() || matches!(value.as_ref().ok(), Some(AnyValue::Null)) {
        return Style::default().fg(Color::DarkGray);
    }

    match series.dtype() {
        DataType::Boolean => {
            if let Ok(val) = series.bool() {
                if let Some(v) = val.get(idx) {
                    return if v {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Red)
                    };
                }
            }
            Style::default()
        }
        _ => Style::default(),
    }
}

/// Generate a Series from a predefined category list
fn generate_category_series(
    name: &str,
    values: &[&str],
    rows: usize,
    null_prob: f64,
    rng: &mut dyn rand::RngCore,
) -> Series {
    use rand::Rng;
    let data: Vec<Option<&str>> = (0..rows)
        .map(|_| {
            if null_prob > 0.0 && rng.random_bool(null_prob) {
                None
            } else {
                Some(values[rng.random_range(0..values.len())])
            }
        })
        .collect();
    Series::new(name.into(), data)
}

/// Generate a Series using a generator function
fn generate_string_series<F>(
    name: &str,
    rows: usize,
    null_prob: f64,
    rng: &mut dyn rand::RngCore,
    generator: F,
) -> Series
where
    F: Fn(&mut dyn rand::RngCore) -> String,
{
    use rand::Rng;
    let data: Vec<Option<String>> = (0..rows)
        .map(|_| {
            if null_prob > 0.0 && rng.random_bool(null_prob) {
                None
            } else {
                Some(generator(rng))
            }
        })
        .collect();
    Series::new(name.into(), data)
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

/// Print available generators
fn print_available_generators() {
    println!("{}", "Available Column Generators".cyan().bold());
    println!("{}", "═".repeat(60));
    println!();

    println!(
        "{}",
        "PRIMITIVES (with optional parameters)".yellow().bold()
    );
    println!(
        "  {:20} Sequential integers (default: 1, 1)",
        "id[start;step]".white()
    );
    println!(
        "  {:20} Random integers (default: 0, 1000)",
        "int[min;max]".white()
    );
    println!(
        "  {:20} Random floats (default: 0.0, 1000.0)",
        "float[min;max]".white()
    );
    println!(
        "  {:20} Random alphanumeric (default: 10)",
        "string[len]".white()
    );
    println!(
        "  {:20} Random true/false (default: 0.5)",
        "bool[prob]".white()
    );
    println!("  {:20} Random dates (2020-2025)", "date".white());
    println!();

    println!("{}", "CATEGORIES".yellow().bold());
    println!(
        "  {:20} Generic cat_0..cat_N (--categories)",
        "category, cat, enum".white()
    );
    println!("  {:20} apple, banana, orange, ...", "fruit".white());
    println!("  {:20} red, blue, green, ...", "color".white());
    println!("  {:20} New York, London, Tokyo, ...", "city".white());
    println!("  {:20} USA, Germany, Japan, ...", "country".white());
    println!("  {:20} pending, active, completed, ...", "status".white());
    println!("  {:20} low, medium, high, critical", "priority".white());
    println!(
        "  {:20} Engineering, Sales, HR, ...",
        "department, dept".white()
    );
    println!("  {:20} Monday, Tuesday, ...", "day".white());
    println!("  {:20} XS, S, M, L, XL, XXL", "size".white());
    println!();

    println!("{}", "PERSONAL".yellow().bold());
    println!("  {:20} James, Mary, ...", "first_name, firstname".white());
    println!("  {:20} Smith, Johnson, ...", "last_name, lastname".white());
    println!(
        "  {:20} James Smith, Mary Johnson, ...",
        "full_name, name".white()
    );
    println!("  {:20} user@example.com", "email".white());
    println!("  {:20} cool_user42", "username".white());
    println!("  {:20} (555) 123-4567", "phone".white());
    println!("  {:20} 123 Main Street", "address, street_address".white());
    println!("  {:20} 12345", "zip, zip_code, postal_code".white());
    println!();

    println!("{}", "NETWORK".yellow().bold());
    println!("  {:20} 192.168.1.100", "ipv4, ip".white());
    println!("  {:20} 2001:db8::1", "ipv6".white());
    println!("  {:20} 00:1A:2B:3C:4D:5E", "mac, mac_address".white());
    println!("  {:20} example.com", "domain".white());
    println!("  {:20} https://example.com/path", "url".white());
    println!();

    println!("{}", "IDENTIFIERS".yellow().bold());
    println!("  {:20} 550e8400-e29b-41d4-...", "uuid, uuid4".white());
    println!("  {:20} 018f6b1c-... (time-based)", "uuid7".white());
    println!(
        "  {:20} 4532015112830366 (Luhn valid)",
        "credit_card, cc".white()
    );
    println!("  {:20} DE89370400440532013000", "iban".white());
    println!("  {:20} 9780306406157", "isbn, isbn13".white());
    println!("  {:20} 0306406152", "isbn10".white());
    println!("  {:20} 123-45-6789 (US format)", "ssn, ssn_us".white());
    println!("  {:20} 12345678901 (Norwegian)", "ssn_no".white());
    println!();

    println!("{}", "TEXT".yellow().bold());
    println!("  {:20} random word", "word".white());
    println!("  {:20} Lorem ipsum dolor sit amet...", "sentence".white());
    println!("  {:20} Full paragraph of lorem ipsum", "paragraph".white());
    println!();

    println!("{}", "GEO".yellow().bold());
    println!(
        "  {:20} Random latitude (-90 to 90)",
        "lat[min;max]".white()
    );
    println!(
        "  {:20} Random longitude (-180 to 180)",
        "lon[min;max]".white()
    );
    println!(
        "  {:20} \"(lon, lat)\" coordinate string",
        "coords[bbox]".white()
    );
    println!("  {:20} GeoJSON Point geometry", "point[bbox]".white());
    println!("  {:20} (bbox = minLon;minLat;maxLon;maxLat)", "".dimmed());
    println!();

    println!("{}", "OTHER".yellow().bold());
    println!("  {:20} Secure random password", "password".white());
    println!("  {:20} Random hex string", "hex".white());
    println!();

    println!("{}", "USAGE".cyan().bold());
    println!("  dx polars random -c \"id:id,name:full_name,email:email\"");
    println!("  dx polars random -c \"id:id,card:credit_card,ip:ipv4\" -n 100");
    println!("  dx polars random -c \"id:id,book:isbn,price:float\" -f csv");
    println!();

    println!("{}", "PARAMETER EXAMPLES".cyan().bold());
    println!("  dx polars random -c \"id:id[100;1],small:int[0;10],big:int[1000;9999]\"");
    println!("  dx polars random -c \"pct:float[0;100],code:string[5],active:bool[0.8]\"");
    println!("  dx polars random -c \"id:id,loc:point[4;57;31;71]\" # Norway bbox");
}
