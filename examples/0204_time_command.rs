//! # Time Command Implementation
//!
//! This example shows how to implement timestamp parsing and formatting.
//!
//! Run with: `cargo run --example 0204_time_command`

#![allow(dead_code)]

use chrono::{DateTime, Local, TimeZone, Utc};

// =========================================================================
// TIME FORMATS
// =========================================================================

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Iso,
    Unix,
    UnixMs,
    Rfc2822,
    Rfc3339,
    Human,
}

// =========================================================================
// FORMATTING
// =========================================================================

pub fn format_time<Tz: TimeZone>(dt: &DateTime<Tz>, format: Format) -> String
where
    Tz::Offset: std::fmt::Display,
{
    match format {
        Format::Iso => dt.format("%Y-%m-%dT%H:%M:%S%:z").to_string(),
        Format::Unix => dt.timestamp().to_string(),
        Format::UnixMs => dt.timestamp_millis().to_string(),
        Format::Rfc2822 => dt.format("%a, %d %b %Y %H:%M:%S %z").to_string(),
        Format::Rfc3339 => dt.to_rfc3339(),
        Format::Human => dt.format("%B %d, %Y at %I:%M:%S %p").to_string(),
    }
}

// =========================================================================
// PARSING
// =========================================================================

pub fn parse_timestamp(s: &str) -> Result<DateTime<Utc>, String> {
    // Try RFC 3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Try RFC 2822
    if let Ok(dt) = DateTime::parse_from_rfc2822(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Try Unix timestamp
    if let Ok(ts) = s.parse::<i64>() {
        let dt = if ts > 10_000_000_000 {
            // Milliseconds
            Utc.timestamp_millis_opt(ts)
                .single()
                .ok_or("Invalid timestamp")?
        } else {
            // Seconds
            Utc.timestamp_opt(ts, 0)
                .single()
                .ok_or("Invalid timestamp")?
        };
        return Ok(dt);
    }

    Err(format!("Could not parse: {}", s))
}

fn main() {
    println!("=== Time Command Implementation ===\n");

    // =========================================================================
    // CURRENT TIME
    // =========================================================================

    println!("--- Current Time ---");
    let now_local = Local::now();
    let now_utc = Utc::now();

    println!("  Local: {}", format_time(&now_local, Format::Iso));
    println!("  UTC:   {}", format_time(&now_utc, Format::Iso));

    println!();

    // =========================================================================
    // FORMAT OPTIONS
    // =========================================================================

    println!("--- Format Options ---");
    let now = Utc::now();

    println!("  ISO 8601:  {}", format_time(&now, Format::Iso));
    println!("  RFC 3339:  {}", format_time(&now, Format::Rfc3339));
    println!("  RFC 2822:  {}", format_time(&now, Format::Rfc2822));
    println!("  Unix:      {}", format_time(&now, Format::Unix));
    println!("  Unix (ms): {}", format_time(&now, Format::UnixMs));
    println!("  Human:     {}", format_time(&now, Format::Human));

    println!();

    // =========================================================================
    // PARSING
    // =========================================================================

    println!("--- Parsing Timestamps ---");

    let inputs = [
        "2024-01-15T10:30:00Z",
        "2024-01-15T10:30:00+05:30",
        "1705315800",
        "1705315800000",
    ];

    for input in inputs {
        match parse_timestamp(input) {
            Ok(dt) => println!("  {} -> {}", input, format_time(&dt, Format::Iso)),
            Err(e) => println!("  {} -> Error: {}", input, e),
        }
    }

    println!();

    // =========================================================================
    // DURATION CALCULATION
    // =========================================================================

    println!("--- Duration Calculation ---");
    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc::now();
    let duration = end.signed_duration_since(start);

    println!("  From: {}", format_time(&start, Format::Iso));
    println!("  To:   {}", format_time(&end, Format::Iso));
    println!("  Duration:");
    println!("    Days:    {}", duration.num_days());
    println!("    Hours:   {}", duration.num_hours());
    println!("    Minutes: {}", duration.num_minutes());
    println!("    Seconds: {}", duration.num_seconds());

    println!();

    // =========================================================================
    // CLI USAGE
    // =========================================================================

    println!("--- CLI Usage ---");
    println!(
        r#"
# Current time
dx time now
dx time now -f unix
dx time now -f human

# Parse and show formats
dx time parse "2024-01-15T10:30:00Z"
dx time parse "1705315800"

# Convert between formats
dx time convert "1705315800" -f iso
dx time convert "2024-01-15T10:30:00Z" -f unix

# Calculate duration
dx time diff "2024-01-01T00:00:00Z"
dx time diff "2024-01-01" "2024-12-31"
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Time command features:");
    println!("  1. Multiple formats: ISO, RFC, Unix, Human");
    println!("  2. Flexible parsing (auto-detect format)");
    println!("  3. Timezone handling (UTC, local)");
    println!("  4. Duration calculation");
    println!("  5. Format conversion");
}
