//! # Unit Conversion Command
//!
//! Convert between units and number bases.
//!
//! ## Examples
//! ```bash
//! dx calc bytes 1.5gb            # Shows in B, KB, MB, GB
//! dx calc time 3665s             # 1h 1m 5s
//! dx calc percent 15 of 200      # 7.5%
//! dx calc base 255 10 16         # ff
//! ```

use crate::cli::commands::calc::{CalcArgs, CalcCommand};
use anyhow::{bail, Context, Result};
use colored::Colorize;

pub fn run(args: CalcArgs) -> Result<()> {
    match args.command {
        CalcCommand::Bytes { value } => cmd_bytes(&value),
        CalcCommand::Time { value } => cmd_time(&value),
        CalcCommand::Percent { value, total } => cmd_percent(value, total),
        CalcCommand::Base { number, from, to } => cmd_base(&number, from, to),
    }
}

/// Convert byte sizes
fn cmd_bytes(value: &str) -> Result<()> {
    let bytes = parse_bytes(value)?;

    println!("{}: {}", "bytes".cyan(), bytes);
    println!("{}: {:.2}", "KB".cyan(), bytes as f64 / 1024.0);
    println!("{}: {:.2}", "MB".cyan(), bytes as f64 / (1024.0 * 1024.0));
    println!(
        "{}: {:.2}",
        "GB".cyan(),
        bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    println!(
        "{}: {:.4}",
        "TB".cyan(),
        bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0)
    );

    // Also show human readable
    let bs = bytesize::ByteSize::b(bytes);
    println!("{}: {}", "human".cyan(), bs);

    Ok(())
}

/// Parse byte string like "1.5gb" or "1024"
fn parse_bytes(s: &str) -> Result<u64> {
    let s = s.trim().to_lowercase();

    // Try parsing as plain number first
    if let Ok(n) = s.parse::<u64>() {
        return Ok(n);
    }

    // Try bytesize parsing
    if let Ok(bs) = s.parse::<bytesize::ByteSize>() {
        return Ok(bs.as_u64());
    }

    // Manual parsing for formats like "1.5gb"
    let multipliers = [
        ("tb", 1024u64 * 1024 * 1024 * 1024),
        ("gb", 1024u64 * 1024 * 1024),
        ("mb", 1024u64 * 1024),
        ("kb", 1024u64),
        ("b", 1u64),
    ];

    for (suffix, mult) in multipliers {
        if s.ends_with(suffix) {
            let num_part = s.trim_end_matches(suffix).trim();
            let num: f64 = num_part.parse().context("Invalid number")?;
            return Ok((num * mult as f64) as u64);
        }
    }

    bail!("Cannot parse byte size: {}", s)
}

/// Convert/parse durations
fn cmd_time(value: &str) -> Result<()> {
    let secs = parse_duration_secs(value)?;

    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    let s = secs % 60;

    println!("{}: {}", "seconds".cyan(), secs);
    println!("{}: {}", "minutes".cyan(), secs as f64 / 60.0);
    println!("{}: {}", "hours".cyan(), secs as f64 / 3600.0);
    println!("{}: {}", "days".cyan(), secs as f64 / 86400.0);

    // Human readable
    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if mins > 0 {
        parts.push(format!("{}m", mins));
    }
    if s > 0 || parts.is_empty() {
        parts.push(format!("{}s", s));
    }

    println!("{}: {}", "human".cyan(), parts.join(" "));

    Ok(())
}

/// Parse duration string to seconds
fn parse_duration_secs(s: &str) -> Result<u64> {
    let s = s.trim().to_lowercase();

    // Try as plain seconds
    if let Ok(n) = s.parse::<u64>() {
        return Ok(n);
    }

    // Try humantime parsing
    if let Ok(dur) = humantime::parse_duration(&s) {
        return Ok(dur.as_secs());
    }

    // Manual parsing for compact format like "1h30m"
    let mut total_secs = 0u64;
    let mut num_buf = String::new();

    for c in s.chars() {
        if c.is_ascii_digit() || c == '.' {
            num_buf.push(c);
        } else if !num_buf.is_empty() {
            let num: f64 = num_buf.parse().unwrap_or(0.0);
            num_buf.clear();

            total_secs += match c {
                'd' => (num * 86400.0) as u64,
                'h' => (num * 3600.0) as u64,
                'm' => (num * 60.0) as u64,
                's' => num as u64,
                _ => 0,
            };
        }
    }

    if total_secs > 0 {
        Ok(total_secs)
    } else {
        bail!("Cannot parse duration: {}", s)
    }
}

/// Calculate percentage
fn cmd_percent(value: f64, total: f64) -> Result<()> {
    if total == 0.0 {
        bail!("Cannot divide by zero");
    }

    let percent = (value / total) * 100.0;
    println!("{:.2}%", percent);
    Ok(())
}

/// Convert between number bases
fn cmd_base(number: &str, from: u32, to: u32) -> Result<()> {
    if !(2..=36).contains(&from) || !(2..=36).contains(&to) {
        bail!("Base must be between 2 and 36");
    }

    // Parse from source base
    let decimal = i64::from_str_radix(number, from)
        .with_context(|| format!("Invalid number '{}' for base {}", number, from))?;

    // Convert to target base
    let result = format_radix(decimal, to);
    println!("{}", result);

    Ok(())
}

/// Format number in given radix (base)
fn format_radix(mut n: i64, radix: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let negative = n < 0;
    if negative {
        n = -n;
    }

    let mut digits = Vec::new();
    while n > 0 {
        let digit = (n % radix as i64) as u32;
        digits.push(char::from_digit(digit, radix).unwrap_or('?'));
        n /= radix as i64;
    }

    digits.reverse();
    let result: String = digits.into_iter().collect();

    if negative {
        format!("-{}", result)
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bytes() {
        assert_eq!(parse_bytes("1024").unwrap(), 1024);
        // bytesize uses SI units (1000), our manual parser uses binary (1024)
        assert_eq!(parse_bytes("1kib").unwrap(), 1024); // KiB = binary
        assert_eq!(parse_bytes("1mib").unwrap(), 1024 * 1024); // MiB = binary
    }

    #[test]
    fn test_format_radix() {
        assert_eq!(format_radix(255, 16), "ff");
        assert_eq!(format_radix(10, 2), "1010");
    }

    // Parser tests are now in src/expr/parser.rs
}
