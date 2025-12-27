//! # Time Command Implementation
//!
//! This module provides date/time utilities for parsing, formatting, and
//! calculating durations between timestamps.
//!
//! ## Key Concepts
//!
//! ### Timezone-Aware vs Naive DateTimes
//! - **Naive**: Just date and time, no timezone (dangerous for comparisons!)
//! - **Aware**: Date, time, AND timezone offset (safe for calculations)
//!
//! We use `DateTime<FixedOffset>` which is always timezone-aware.
//!
//! ### Common Time Formats
//!
//! | Format | Example | Use Case |
//! |--------|---------|----------|
//! | ISO 8601 | 2023-11-14T22:13:20+00:00 | Data interchange |
//! | RFC 2822 | Tue, 14 Nov 2023 22:13:20 +0000 | Email headers |
//! | RFC 3339 | 2023-11-14T22:13:20.000Z | APIs, JSON |
//! | Unix seconds | 1700000000 | Databases, logs |
//! | Unix milliseconds | 1700000000000 | JavaScript, Java |
//!
//! ### Unix Timestamps
//! Unix time is seconds (or milliseconds) since January 1, 1970 00:00:00 UTC.
//! This is called the "Unix epoch". It's timezone-agnostic by definition.
//!
//! ## Example Usage
//! ```bash
//! dx time now                          # Current time in ISO format
//! dx time now --format unix            # Current Unix timestamp
//! dx time parse 1700000000             # Parse Unix timestamp
//! dx time parse "2023-11-14T22:13:20Z" # Parse ISO string
//! dx time convert 1700000000 --format human  # Convert to readable
//! dx time diff 1700000000 1700086400   # Duration between timestamps
//! ```
//!
//! ## External Documentation
//! - Chrono crate: <https://docs.rs/chrono>
//! - ISO 8601: <https://en.wikipedia.org/wiki/ISO_8601>
//! - Unix time: <https://en.wikipedia.org/wiki/Unix_time>

use crate::cli::commands::time::{TimeArgs, TimeCommand, TimeFormat};
use anyhow::{Context, Result, bail};
use chrono::{DateTime, Local, TimeZone, Utc};
use colored::Colorize;

/// Run the time command, dispatching to the appropriate subcommand.
///
/// Time has multiple subcommands:
/// - `now`: Display current time
/// - `parse`: Parse a timestamp and show in all formats
/// - `convert`: Convert a timestamp to a specific format
/// - `diff`: Calculate duration between two timestamps
pub fn run(args: TimeArgs) -> Result<()> {
    match args.command {
        TimeCommand::Now { format, timezone } => cmd_now(format, &timezone),
        TimeCommand::Parse {
            timestamp,
            input_format,
        } => cmd_parse(&timestamp, input_format.as_deref()),
        TimeCommand::Convert { input, format } => cmd_convert(&input, format),
        TimeCommand::Diff { start, end } => cmd_diff(&start, end.as_deref()),
    }
}

/// Show the current time in the requested format and timezone.
///
/// # Timezone Handling
/// - "utc": Use UTC (Coordinated Universal Time)
/// - "local": Use the system's local timezone
/// - Other values: Fall back to local (could be extended to support IANA zones)
///
/// # Why `to_rfc3339()` then parse?
/// This ensures we have a `DateTime<FixedOffset>` which can be formatted
/// consistently. The round-trip through RFC3339 normalizes the representation.
fn cmd_now(format: TimeFormat, timezone: &str) -> Result<()> {
    // Get current time in the specified timezone
    // eq_ignore_ascii_case provides case-insensitive comparison
    let now = if timezone.eq_ignore_ascii_case("utc") {
        // UTC: No daylight saving, no local quirks
        Utc::now().with_timezone(&Utc).to_rfc3339()
    } else if timezone.eq_ignore_ascii_case("local") {
        // Local: Uses system timezone settings
        Local::now().to_rfc3339()
    } else {
        // Fallback to local for unrecognized timezones
        // TODO: Could use chrono-tz crate for IANA timezone names
        Local::now().to_rfc3339()
    };

    // Parse the RFC3339 string back to get a DateTime with fixed offset
    // This gives us a consistent type for formatting
    let dt = DateTime::parse_from_rfc3339(&now)?;
    println!("{}", format_datetime(&dt, format));
    Ok(())
}

/// Parse a timestamp and display it in all supported formats.
///
/// This is useful for debugging timestamps - just paste any format and
/// see what it means in every other format.
///
/// # Parameters
/// - `timestamp`: The input to parse (Unix, ISO, RFC2822, etc.)
/// - `_input_format`: Reserved for future use (explicit format hint)
fn cmd_parse(timestamp: &str, _input_format: Option<&str>) -> Result<()> {
    // Auto-detect format and parse
    let dt = parse_timestamp(timestamp)?;

    // Display the same moment in time in all formats
    // This helps developers understand format differences
    println!(
        "{}: {}",
        "ISO 8601".cyan(),
        format_datetime(&dt, TimeFormat::Iso)
    );
    println!(
        "{}: {}",
        "RFC 2822".cyan(),
        format_datetime(&dt, TimeFormat::Rfc2822)
    );
    println!(
        "{}: {}",
        "RFC 3339".cyan(),
        format_datetime(&dt, TimeFormat::Rfc3339)
    );
    println!(
        "{}: {}",
        "Unix".cyan(),
        format_datetime(&dt, TimeFormat::Unix)
    );
    println!(
        "{}: {}",
        "Unix (ms)".cyan(),
        format_datetime(&dt, TimeFormat::UnixMs)
    );
    println!(
        "{}: {}",
        "Human".cyan(),
        format_datetime(&dt, TimeFormat::Human)
    );

    Ok(())
}

/// Convert a timestamp to a specific format.
///
/// Unlike `parse` which shows all formats, this outputs just one format.
/// Useful for scripting: `dx time convert $TIMESTAMP --format unix`
fn cmd_convert(input: &str, format: TimeFormat) -> Result<()> {
    let dt = parse_timestamp(input)?;
    println!("{}", format_datetime(&dt, format));
    Ok(())
}

/// Calculate and display the duration between two timestamps.
///
/// # Default End Time
/// If no end timestamp is provided, uses the current time.
/// This allows: "how long ago was this timestamp?"
///
/// # Duration Calculation
/// Chrono's `signed_duration_since` returns a `Duration` which can be
/// broken down into days, hours, minutes, seconds. We use modulo (%)
/// to get the remainder after extracting larger units.
fn cmd_diff(start: &str, end: Option<&str>) -> Result<()> {
    let start_dt = parse_timestamp(start)?;

    // End time defaults to now if not specified
    let end_dt = if let Some(e) = end {
        parse_timestamp(e)?
    } else {
        // Convert Utc::now() to fixed offset for type compatibility
        // .into() converts DateTime<Utc> to DateTime<FixedOffset>
        Utc::now().into()
    };

    // Calculate the signed duration (can be negative if end < start)
    let duration = end_dt.signed_duration_since(start_dt);

    // Break down into human-readable components
    // num_days() gives total days, we use modulo for smaller units
    let days = duration.num_days();
    let hours = duration.num_hours() % 24; // Hours not counted in days
    let minutes = duration.num_minutes() % 60; // Minutes not in hours
    let seconds = duration.num_seconds() % 60; // Seconds not in minutes

    println!(
        "{} days, {} hours, {} minutes, {} seconds",
        days.to_string().green(),
        hours.to_string().green(),
        minutes.to_string().green(),
        seconds.to_string().green()
    );
    // Also show total seconds for precision
    println!("Total seconds: {}", duration.num_seconds());

    Ok(())
}

/// Parse a timestamp from various common formats.
///
/// # Auto-Detection Strategy
///
/// We try formats in order of specificity:
/// 1. RFC 3339 (most precise, includes timezone)
/// 2. RFC 2822 (email format, includes timezone)
/// 3. Unix timestamp (plain number)
///
/// # Unix Timestamp Detection
///
/// Unix timestamps can be in seconds or milliseconds. We use a heuristic:
/// - If the number is > 10 billion, it's probably milliseconds
/// - Otherwise, it's probably seconds
///
/// **Why 10 billion?**
/// - 10,000,000,000 seconds = year 2286
/// - 10,000,000,000 milliseconds = year 1970 + ~4 months
///
/// This heuristic works for any date between 1970 and 2286.
///
/// # Returns
/// `DateTime<FixedOffset>` - A timezone-aware datetime
///
/// # Errors
/// Returns an error if the input doesn't match any known format.
fn parse_timestamp(s: &str) -> Result<DateTime<chrono::FixedOffset>> {
    // Try RFC 3339 first (e.g., "2023-11-14T22:13:20+00:00")
    // This is the most specific format with explicit timezone
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt);
    }

    // Try RFC 2822 (e.g., "Tue, 14 Nov 2023 22:13:20 +0000")
    // Common in email headers
    if let Ok(dt) = DateTime::parse_from_rfc2822(s) {
        return Ok(dt);
    }

    // Try parsing as a numeric Unix timestamp
    if let Ok(ts) = s.parse::<i64>() {
        // Heuristic: distinguish seconds from milliseconds
        // 10 billion seconds is year 2286, so anything larger is milliseconds
        let dt = if ts > 10_000_000_000 {
            // Interpret as milliseconds since epoch
            // timestamp_millis_opt returns LocalResult which may be ambiguous
            // .single() extracts the unique value or returns None
            Utc.timestamp_millis_opt(ts)
                .single()
                .context("Invalid millisecond timestamp")?
        } else {
            // Interpret as seconds since epoch
            // The second argument (0) is nanoseconds
            Utc.timestamp_opt(ts, 0)
                .single()
                .context("Invalid second timestamp")?
        };
        // Convert to FixedOffset (UTC is offset +00:00)
        return Ok(dt.fixed_offset());
    }

    // No format matched - give a helpful error
    bail!("Could not parse timestamp: {}", s)
}

/// Format a datetime according to the specified format.
///
/// # Generic over Timezone
/// This function works with any timezone type (`Utc`, `Local`, `FixedOffset`)
/// thanks to the generic `Tz: TimeZone` parameter.
///
/// # Format Strings
/// Chrono uses strftime-style format specifiers:
/// - `%Y`: 4-digit year
/// - `%m`: 2-digit month
/// - `%d`: 2-digit day
/// - `%H`: 24-hour hour
/// - `%M`: Minute
/// - `%S`: Second
/// - `%:z`: Timezone as +HH:MM
/// - `%z`: Timezone as +HHMM
/// - `%B`: Full month name
/// - `%I`: 12-hour hour
/// - `%p`: AM/PM
///
/// See: <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>
fn format_datetime<Tz: TimeZone>(dt: &DateTime<Tz>, format: TimeFormat) -> String
where
    // This bound ensures we can display the timezone offset
    Tz::Offset: std::fmt::Display,
{
    match format {
        // ISO 8601: The international standard for date/time
        // Example: 2023-11-14T22:13:20+00:00
        TimeFormat::Iso => dt.format("%Y-%m-%dT%H:%M:%S%:z").to_string(),

        // Unix timestamp: Seconds since 1970-01-01 00:00:00 UTC
        // No timezone info needed - it's always UTC by definition
        TimeFormat::Unix => dt.timestamp().to_string(),

        // Unix milliseconds: Common in JavaScript/Java
        // Useful for higher precision timestamps
        TimeFormat::UnixMs => dt.timestamp_millis().to_string(),

        // RFC 2822: Used in email headers
        // Example: Tue, 14 Nov 2023 22:13:20 +0000
        TimeFormat::Rfc2822 => dt.format("%a, %d %b %Y %H:%M:%S %z").to_string(),

        // RFC 3339: A profile of ISO 8601 for internet timestamps
        // Slightly stricter than ISO 8601, widely used in APIs
        TimeFormat::Rfc3339 => dt.to_rfc3339(),

        // Human-readable: For display to users
        // Example: November 14, 2023 at 10:13 PM
        TimeFormat::Human => dt.format("%B %d, %Y at %I:%M %p").to_string(),
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test parsing Unix timestamps in seconds.
    #[test]
    fn test_parse_unix_seconds() {
        let dt = parse_timestamp("1700000000").unwrap();
        assert_eq!(dt.timestamp(), 1700000000);
    }

    /// Test parsing Unix timestamps in milliseconds.
    /// The heuristic should detect this as milliseconds.
    #[test]
    fn test_parse_unix_millis() {
        let dt = parse_timestamp("1700000000000").unwrap();
        // Should convert to same second
        assert_eq!(dt.timestamp(), 1700000000);
    }

    /// Test parsing RFC 3339 formatted strings.
    #[test]
    fn test_parse_rfc3339() {
        let dt = parse_timestamp("2023-11-14T22:13:20+00:00").unwrap();
        assert_eq!(dt.timestamp(), 1700000000);
    }
}
