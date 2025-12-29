//! Polars command - DataFrame operations and data analysis demos.

use crate::cli::commands::polars::{PolarsArgs, PolarsCommand, PolarsOutputFormat};
use anyhow::{Context, Result};
use colored::Colorize;
use dx_datagen::{
    animals, astrology, categories, color, commerce, education, entertainment, file, food,
    generators, geo, government, hacker, healthcare, network, numeric, password, personal, science,
    sports, text, travel, uuid, vehicle, weather,
};
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

            // === UUID / ULID ===
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
            "ulid" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                uuid::ulid_with_rng(r)
            }),

            // === Color ===
            "hex_color" | "hexcolor" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    color::hex_color(r)
                })
            }
            "color_name" | "colorname" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    color::color_name(r).to_string()
                })
            }
            "rgb" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                let (red, g, b) = color::rgb(r);
                format!("rgb({}, {}, {})", red, g, b)
            }),
            "hsl" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                color::css_hsl(r)
            }),

            // === File / System ===
            "mime" | "mime_type" | "mimetype" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    file::mime_type(r).to_string()
                })
            }
            "file_ext" | "extension" | "ext" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    file::file_extension(r).to_string()
                })
            }
            "file_name" | "filename" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    file::file_name(r)
                })
            }
            "file_path" | "filepath" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    file::file_path(r)
                })
            }
            "semver" | "version" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    file::semver(r)
                })
            }
            "user_agent" | "useragent" | "ua" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    file::user_agent(r).to_string()
                })
            }

            // === Commerce ===
            "company" | "company_name" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    commerce::company_name(r)
                })
            }
            "product" | "product_name" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    commerce::product_name(r)
                })
            }
            "job" | "job_title" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    commerce::job_title(r)
                })
            }
            "industry" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    commerce::industry(r).to_string()
                })
            }
            "currency" | "currency_code" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    commerce::currency_code(r).to_string()
                })
            }
            "price" => {
                // Support price[min;max]
                let min: f64 = params.first().and_then(|s| s.parse().ok()).unwrap_or(1.0);
                let max: f64 = params.get(1).and_then(|s| s.parse().ok()).unwrap_or(1000.0);
                let values: Vec<Option<f64>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(commerce::price(&mut *rng, min, max))
                        }
                    })
                    .collect();
                Series::new(name.into(), values)
            }

            // === Vehicle ===
            "vehicle" | "vehicle_make" | "car" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    vehicle::vehicle_make(r).to_string()
                })
            }
            "vehicle_model" | "model" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    vehicle::vehicle_model(r).to_string()
                })
            }
            "vehicle_full" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    vehicle::vehicle_full(r)
                })
            }
            "vin" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                vehicle::vin(r)
            }),
            "license_plate" | "plate" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    vehicle::license_plate(r)
                })
            }

            // === Finance / Crypto ===
            "btc" | "bitcoin" | "btc_address" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    numeric::finance::bitcoin_address(r)
                })
            }
            "eth" | "ethereum" | "eth_address" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    numeric::finance::ethereum_address(r)
                })
            }
            "swift" | "swift_code" | "bic" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    numeric::finance::swift_code(r)
                })
            }
            "routing" | "routing_number" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    numeric::finance::routing_number(r)
                })
            }
            "account" | "account_number" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    numeric::finance::account_number(r)
                })
            }

            // === Science ===
            "element" | "chemical_element" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    science::chemical_element(r).to_string()
                })
            }
            "element_symbol" | "symbol" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    science::chemical_symbol(r).to_string()
                })
            }
            "unit" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                science::unit(r).to_string()
            }),

            // === Timestamp ===
            "timestamp" | "timestamp_s" | "unix_timestamp" => {
                // Support timestamp[min;max] in seconds since epoch
                let min: i64 = params
                    .first()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(946684800); // 2000-01-01
                let max: i64 = params
                    .get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1924991999); // 2030-12-31
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
            "timestamp_ms" | "unix_timestamp_ms" => {
                // Support timestamp_ms[min;max] in milliseconds since epoch
                let min: i64 = params
                    .first()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(946684800000); // 2000-01-01
                let max: i64 = params
                    .get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1924991999000); // 2030-12-31
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

            // === Entertainment ===
            "book_title" | "book" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::book_title(r)
                })
            }
            "book_author" | "author" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::book_author(r)
                })
            }
            "book_genre" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::book_genre(r).to_string()
                })
            }
            "movie_title" | "movie" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::movie_title(r)
                })
            }
            "movie_director" | "director" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::movie_director(r)
                })
            }
            "movie_genre" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::movie_genre(r).to_string()
                })
            }
            "music_artist" | "artist" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::music_artist(r)
                })
            }
            "music_album" | "album" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::music_album(r)
                })
            }
            "music_song" | "song" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::music_song(r)
                })
            }
            "music_genre" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::music_genre(r).to_string()
                })
            }
            "instrument" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::music_instrument(r).to_string()
                })
            }
            "tv_show" | "show" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::tv_show(r)
                })
            }
            "game_title" | "game" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::game_title(r)
                })
            }
            "game_platform" | "platform" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    entertainment::game_platform(r).to_string()
                })
            }

            // === Food ===
            "dish" | "food" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    food::dish(r).to_string()
                })
            }
            "cuisine" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    food::cuisine(r).to_string()
                })
            }
            "ingredient" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    food::ingredient(r).to_string()
                })
            }
            "spice" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    food::spice(r).to_string()
                })
            }
            "vegetable" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    food::vegetable(r).to_string()
                })
            }
            "beverage" | "drink" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    food::beverage(r).to_string()
                })
            }
            "coffee" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    food::coffee_drink(r).to_string()
                })
            }
            "beer" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                food::beer_style(r).to_string()
            }),
            "wine" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                food::wine_variety(r).to_string()
            }),
            "restaurant" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    food::restaurant_name(r)
                })
            }

            // === Animals ===
            "animal" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    animals::animal(r).to_string()
                })
            }
            "dog" | "dog_breed" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    animals::dog_breed(r).to_string()
                })
            }
            "cat_breed" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    animals::cat_breed(r).to_string()
                })
            }
            "bird" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                animals::bird(r).to_string()
            }),
            "fish" => generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                animals::fish(r).to_string()
            }),
            "pet_name" | "pet" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    animals::pet_name(r).to_string()
                })
            }

            // === Travel ===
            "airline" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    travel::airline(r).to_string()
                })
            }
            "flight" | "flight_number" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    travel::flight_number(r)
                })
            }
            "airport" | "airport_code" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    travel::airport_code(r).to_string()
                })
            }
            "aircraft" | "aircraft_type" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    travel::aircraft_type(r).to_string()
                })
            }
            "seat" | "seat_number" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    travel::seat(r)
                })
            }
            "hotel" | "hotel_chain" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    travel::hotel_chain(r).to_string()
                })
            }
            "room_type" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    travel::room_type(r).to_string()
                })
            }
            "landmark" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    travel::landmark(r).to_string()
                })
            }
            "destination" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    travel::destination(r).to_string()
                })
            }

            // === Healthcare ===
            "condition" | "medical_condition" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    healthcare::condition(r).to_string()
                })
            }
            "medication" | "medicine" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    healthcare::medication(r).to_string()
                })
            }
            "blood_type" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    healthcare::blood_type(r).to_string()
                })
            }
            "hospital" | "hospital_name" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    healthcare::hospital_name(r)
                })
            }
            "specialty" | "medical_specialty" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    healthcare::specialty(r).to_string()
                })
            }

            // === Sports ===
            "sport" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    sports::sport(r).to_string()
                })
            }
            "team" | "team_name" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    sports::team_name(r)
                })
            }
            "league" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    sports::league(r).to_string()
                })
            }
            "position" | "player_position" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    sports::position(r).to_string()
                })
            }
            "tournament" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    sports::tournament(r).to_string()
                })
            }

            // === Hacker / Developer ===
            "hacker_phrase" | "hacker" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    hacker::hacker_phrase(r)
                })
            }
            "programming_language" | "lang" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    hacker::programming_language(r).to_string()
                })
            }
            "framework" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    hacker::framework(r).to_string()
                })
            }
            "database" | "db" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    hacker::database(r).to_string()
                })
            }
            "git_branch" | "branch" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    hacker::git_branch(r)
                })
            }
            "git_commit" | "commit_message" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    hacker::git_commit_message(r)
                })
            }
            "git_sha" | "sha" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    hacker::git_sha(r)
                })
            }

            // === Education ===
            "university" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    education::university(r).to_string()
                })
            }
            "degree" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    education::degree(r).to_string()
                })
            }
            "major" | "field_of_study" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    education::major(r).to_string()
                })
            }
            "course" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    education::course_name(r)
                })
            }
            "gpa" => {
                let values: Vec<Option<f64>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(education::gpa(&mut *rng) as f64)
                        }
                    })
                    .collect();
                Series::new(name.into(), values)
            }

            // === Government ===
            "government_agency" | "agency" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    government::us_agency(r).to_string()
                })
            }
            "passport" | "passport_number" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    government::passport_number(r)
                })
            }
            "drivers_license" | "license_number" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    government::drivers_license(r)
                })
            }
            "tax_id" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    government::tax_id(r)
                })
            }

            // === Weather ===
            "weather_condition" | "weather" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    weather::condition(r).to_string()
                })
            }
            "temperature" | "temp" => {
                // Support temp[min;max] in Celsius
                let min: i8 = params.first().and_then(|s| s.parse().ok()).unwrap_or(-20);
                let max: i8 = params.get(1).and_then(|s| s.parse().ok()).unwrap_or(40);
                let values: Vec<Option<i64>> = (0..config.rows)
                    .map(|_| {
                        if config.null_prob > 0.0 && rng.random_bool(config.null_prob) {
                            None
                        } else {
                            Some(rng.random_range(min..=max) as i64)
                        }
                    })
                    .collect();
                Series::new(name.into(), values)
            }
            "season" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    weather::season(r).to_string()
                })
            }
            "forecast" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    weather::forecast_summary(r)
                })
            }

            // === Astrology ===
            "zodiac" | "zodiac_sign" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    astrology::zodiac_sign(r).to_string()
                })
            }
            "chinese_zodiac" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    astrology::chinese_zodiac(r).to_string()
                })
            }
            "birthstone" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    astrology::birthstone(r).to_string()
                })
            }
            "horoscope" => {
                generate_string_series(name, config.rows, config.null_prob, &mut *rng, |r| {
                    astrology::horoscope(r)
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
    println!("  {:20} 01ARZ3NDEKTSV4RR... (sortable)", "ulid".white());
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

    println!("{}", "COLOR".yellow().bold());
    println!("  {:20} #FF5733", "hex_color, hexcolor".white());
    println!(
        "  {:20} Red, Blue, Green, ...",
        "color_name, colorname".white()
    );
    println!("  {:20} rgb(255, 87, 51)", "rgb".white());
    println!("  {:20} hsl(210, 50%, 60%)", "hsl".white());
    println!();

    println!("{}", "FILE / SYSTEM".yellow().bold());
    println!(
        "  {:20} application/json, text/html",
        "mime, mime_type".white()
    );
    println!("  {:20} pdf, docx, png, ...", "file_ext, extension".white());
    println!("  {:20} report_123.pdf", "file_name, filename".white());
    println!(
        "  {:20} /home/user/docs/file.txt",
        "file_path, filepath".white()
    );
    println!("  {:20} 1.2.3", "semver, version".white());
    println!("  {:20} Mozilla/5.0...", "user_agent, ua".white());
    println!();

    println!("{}", "COMMERCE".yellow().bold());
    println!("  {:20} Acme Corporation", "company, company_name".white());
    println!(
        "  {:20} Rustic Steel Chair",
        "product, product_name".white()
    );
    println!("  {:20} Senior Web Developer", "job, job_title".white());
    println!("  {:20} Technology, Healthcare", "industry".white());
    println!("  {:20} USD, EUR, GBP", "currency, currency_code".white());
    println!("  {:20} 99.99 (default: 1-1000)", "price[min;max]".white());
    println!();

    println!("{}", "VEHICLE".yellow().bold());
    println!("  {:20} Toyota, Ford, BMW", "vehicle, vehicle_make".white());
    println!(
        "  {:20} Camry, F-150, 3 Series",
        "vehicle_model, model".white()
    );
    println!("  {:20} 2023 Toyota Camry", "vehicle_full".white());
    println!("  {:20} 1HGBH41JXMN109186", "vin".white());
    println!("  {:20} ABC-1234", "license_plate, plate".white());
    println!();

    println!("{}", "FINANCE / CRYPTO".yellow().bold());
    println!("  {:20} 1A1zP1eP5QGefi2DM...", "btc, bitcoin".white());
    println!("  {:20} 0x742d35Cc6634C0...", "eth, ethereum".white());
    println!("  {:20} CHASUS33XXX", "swift, bic".white());
    println!(
        "  {:20} 021000021 (9 digits)",
        "routing, routing_number".white()
    );
    println!("  {:20} 1234567890123", "account, account_number".white());
    println!();

    println!("{}", "SCIENCE".yellow().bold());
    println!(
        "  {:20} Hydrogen, Carbon, Gold",
        "element, chemical_element".white()
    );
    println!("  {:20} H, C, Au", "element_symbol, symbol".white());
    println!("  {:20} meter, kilogram, second", "unit".white());
    println!();

    println!("{}", "TIMESTAMP".yellow().bold());
    println!("  {:20} 1735300000 (seconds)", "timestamp[min;max]".white());
    println!(
        "  {:20} 1735300000000 (ms)",
        "timestamp_ms[min;max]".white()
    );
    println!();

    println!("{}", "ENTERTAINMENT".yellow().bold());
    println!(
        "  {:20} The Great Adventure, ...",
        "book_title, book".white()
    );
    println!("  {:20} Famous Author Name", "book_author, author".white());
    println!("  {:20} Mystery, Fantasy, Sci-Fi", "book_genre".white());
    println!("  {:20} The Big Picture, ...", "movie_title, movie".white());
    println!(
        "  {:20} Famous Director Name",
        "movie_director, director".white()
    );
    println!("  {:20} Action, Comedy, Drama", "movie_genre".white());
    println!("  {:20} Artist Band Name", "music_artist, artist".white());
    println!("  {:20} Greatest Hits Album", "music_album, album".white());
    println!("  {:20} Song Title Here", "music_song, song".white());
    println!("  {:20} Rock, Pop, Jazz, ...", "music_genre".white());
    println!("  {:20} Guitar, Piano, Drums", "instrument".white());
    println!("  {:20} Popular Show Name", "tv_show, show".white());
    println!("  {:20} Epic Game Title", "game_title, game".white());
    println!(
        "  {:20} PlayStation, Xbox, PC",
        "game_platform, platform".white()
    );
    println!();

    println!("{}", "FOOD".yellow().bold());
    println!("  {:20} Pasta, Sushi, Tacos", "dish, food".white());
    println!("  {:20} Italian, Japanese, Mexican", "cuisine".white());
    println!("  {:20} Tomato, Cheese, Rice", "ingredient".white());
    println!("  {:20} Oregano, Cumin, Basil", "spice".white());
    println!("  {:20} Carrot, Broccoli, Spinach", "vegetable".white());
    println!("  {:20} Water, Juice, Soda", "beverage, drink".white());
    println!("  {:20} Espresso, Latte, Mocha", "coffee".white());
    println!("  {:20} IPA, Stout, Pilsner", "beer".white());
    println!("  {:20} Merlot, Chardonnay, Pinot", "wine".white());
    println!("  {:20} Golden Dragon, ...", "restaurant".white());
    println!();

    println!("{}", "ANIMALS".yellow().bold());
    println!("  {:20} Dog, Cat, Elephant, ...", "animal".white());
    println!("  {:20} Labrador, Poodle, ...", "dog, dog_breed".white());
    println!("  {:20} Persian, Siamese, ...", "cat_breed".white());
    println!("  {:20} Eagle, Sparrow, Owl", "bird".white());
    println!("  {:20} Salmon, Tuna, Goldfish", "fish".white());
    println!("  {:20} Max, Bella, Charlie", "pet_name, pet".white());
    println!();

    println!("{}", "TRAVEL".yellow().bold());
    println!("  {:20} Delta, United, Emirates", "airline".white());
    println!("  {:20} AA1234, UA567", "flight, flight_number".white());
    println!("  {:20} JFK, LAX, LHR", "airport, airport_code".white());
    println!(
        "  {:20} Boeing 737, Airbus A320",
        "aircraft, aircraft_type".white()
    );
    println!("  {:20} 12A, 24F", "seat, seat_number".white());
    println!("  {:20} Hilton, Marriott", "hotel, hotel_chain".white());
    println!("  {:20} Suite, Standard, Deluxe", "room_type".white());
    println!("  {:20} Eiffel Tower, Big Ben", "landmark".white());
    println!("  {:20} Paris, Tokyo, New York", "destination".white());
    println!();

    println!("{}", "HEALTHCARE".yellow().bold());
    println!(
        "  {:20} Hypertension, Diabetes",
        "condition, medical_condition".white()
    );
    println!("  {:20} Aspirin, Ibuprofen", "medication, medicine".white());
    println!("  {:20} A+, B-, O+, AB-", "blood_type".white());
    println!(
        "  {:20} City General Hospital",
        "hospital, hospital_name".white()
    );
    println!(
        "  {:20} Cardiology, Neurology",
        "specialty, medical_specialty".white()
    );
    println!();

    println!("{}", "SPORTS".yellow().bold());
    println!("  {:20} Football, Basketball, Tennis", "sport".white());
    println!("  {:20} City Lions, ...", "team, team_name".white());
    println!("  {:20} NFL, NBA, Premier League", "league".white());
    println!(
        "  {:20} Quarterback, Forward, ...",
        "position, player_position".white()
    );
    println!("  {:20} World Cup, Super Bowl", "tournament".white());
    println!();

    println!("{}", "HACKER / DEVELOPER".yellow().bold());
    println!(
        "  {:20} We need to hack the...",
        "hacker_phrase, hacker".white()
    );
    println!(
        "  {:20} Rust, Python, TypeScript",
        "programming_language, lang".white()
    );
    println!("  {:20} React, Django, Rails", "framework".white());
    println!("  {:20} PostgreSQL, MongoDB", "database, db".white());
    println!("  {:20} feature/add-login", "git_branch, branch".white());
    println!(
        "  {:20} feat: add user auth",
        "git_commit, commit_message".white()
    );
    println!("  {:20} a1b2c3d4e5f6...", "git_sha, sha".white());
    println!();

    println!("{}", "EDUCATION".yellow().bold());
    println!("  {:20} Harvard, MIT, Stanford", "university".white());
    println!("  {:20} B.S., M.A., Ph.D.", "degree".white());
    println!(
        "  {:20} Computer Science, Biology",
        "major, field_of_study".white()
    );
    println!("  {:20} CS 101: Intro to...", "course".white());
    println!("  {:20} 3.85 (0.0-4.0)", "gpa".white());
    println!();

    println!("{}", "GOVERNMENT".yellow().bold());
    println!(
        "  {:20} FBI, IRS, NASA",
        "government_agency, agency".white()
    );
    println!("  {:20} AB1234567", "passport, passport_number".white());
    println!(
        "  {:20} D12345678",
        "drivers_license, license_number".white()
    );
    println!("  {:20} 123-45-6789", "tax_id".white());
    println!();

    println!("{}", "WEATHER".yellow().bold());
    println!(
        "  {:20} Sunny, Cloudy, Rainy",
        "weather_condition, weather".white()
    );
    println!(
        "  {:20} 23.5 (Celsius, default -20 to 40)",
        "temperature[min;max]".white()
    );
    println!("  {:20} Spring, Summer, Fall, Winter", "season".white());
    println!("  {:20} Partly cloudy with...", "forecast".white());
    println!();

    println!("{}", "ASTROLOGY".yellow().bold());
    println!(
        "  {:20} Aries, Taurus, Gemini, ...",
        "zodiac, zodiac_sign".white()
    );
    println!("  {:20} Rat, Ox, Tiger, ...", "chinese_zodiac".white());
    println!("  {:20} Diamond, Ruby, Emerald", "birthstone".white());
    println!("  {:20} Today you will find...", "horoscope".white());
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
