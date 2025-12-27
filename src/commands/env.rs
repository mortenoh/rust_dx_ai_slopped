//! # Environment Command Implementation
//!
//! This module provides utilities for working with environment variables.
//! Useful for debugging, scripting, and configuration management.
//!
//! ## Key Concepts
//!
//! ### Environment Variables
//! Environment variables are key-value pairs inherited by processes.
//! They're commonly used for:
//! - Configuration (`DATABASE_URL`, `API_KEY`)
//! - Path settings (`PATH`, `HOME`)
//! - Runtime behavior (`DEBUG`, `LOG_LEVEL`)
//!
//! ### Prefix Filtering
//! Many tools use prefixes to namespace their variables:
//! - `AWS_*` - AWS SDK configuration
//! - `DOCKER_*` - Docker settings
//! - `npm_*` - npm/Node.js settings
//!
//! ## Export Formats
//!
//! | Format | Example | Use Case |
//! |--------|---------|----------|
//! | Shell | `export FOO='bar'` | Source in bash/zsh |
//! | Docker | `FOO=bar` | Docker --env-file |
//! | JSON | `{"FOO": "bar"}` | API/programmatic use |
//! | TOML | `FOO = "bar"` | Config files |
//!
//! ## Example Usage
//! ```bash
//! dx env list                      # List all variables
//! dx env list --prefix AWS_        # Only AWS_* variables
//! dx env list --values --sort      # Show sorted with values
//! dx env get HOME                  # Get specific variable
//! dx env get MISSING --default x   # With fallback
//! dx env check PATH HOME           # Verify variables exist
//! dx env export --format docker    # Export for Docker
//! ```
//!
//! ## External Documentation
//! - std::env: <https://doc.rust-lang.org/std/env/>

use crate::cli::commands::env::{EnvArgs, EnvCommand, ExportFormat};
use anyhow::Result;
use colored::Colorize;
use std::collections::BTreeMap;
use std::env;

/// Run the environment command, dispatching to the appropriate subcommand.
///
/// Subcommands:
/// - `list`: Show environment variables
/// - `get`: Get a specific variable
/// - `check`: Verify variables are set
/// - `export`: Output in various formats
pub fn run(args: EnvArgs) -> Result<()> {
    match args.command {
        EnvCommand::List {
            prefix,
            show_values,
            sort,
        } => cmd_list(prefix.as_deref(), show_values, sort),
        EnvCommand::Get { name, default } => cmd_get(&name, default.as_deref()),
        EnvCommand::Check { names, strict } => cmd_check(&names, strict),
        EnvCommand::Export { format, prefix } => cmd_export(format, prefix.as_deref()),
    }
}

/// List environment variables with optional filtering and sorting.
///
/// # Filtering
/// When `prefix` is provided, only variables starting with that prefix
/// are shown. This is useful for focusing on a specific tool's config.
///
/// # Sorting
/// By default, variables are in arbitrary order (hash map iteration).
/// With `--sort`, they're alphabetically ordered for easier reading.
fn cmd_list(prefix: Option<&str>, show_values: bool, sort: bool) -> Result<()> {
    // Collect all environment variables, optionally filtered
    let vars: Vec<(String, String)> = env::vars()
        .filter(|(k, _)| {
            if let Some(p) = prefix {
                // Only include variables starting with the prefix
                k.starts_with(p)
            } else {
                // No filter: include all
                true
            }
        })
        .collect();

    // Optionally sort alphabetically by key
    let vars = if sort {
        let mut sorted = vars;
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        sorted
    } else {
        vars
    };

    // Print each variable
    for (key, value) in vars {
        if show_values {
            // Show key=value (value not colored to avoid confusion)
            println!("{}={}", key.cyan(), value);
        } else {
            // Just show the key name
            println!("{}", key.cyan());
        }
    }

    Ok(())
}

/// Get a specific environment variable's value.
///
/// # Default Values
/// If the variable isn't set and a default is provided, the default
/// is printed instead. This is useful for scripts:
/// ```bash
/// PORT=$(dx env get PORT --default 8080)
/// ```
///
/// # Exit Codes
/// - 0: Variable found (or default used)
/// - 1: Variable not found and no default
fn cmd_get(name: &str, default: Option<&str>) -> Result<()> {
    match env::var(name) {
        Ok(value) => {
            // Variable exists: print its value
            println!("{}", value);
            Ok(())
        }
        Err(_) => {
            if let Some(d) = default {
                // Variable missing but default provided
                println!("{}", d);
                Ok(())
            } else {
                // Variable missing, no default: error
                eprintln!("{} Variable '{}' not set", "✗".red().bold(), name);
                std::process::exit(1);
            }
        }
    }
}

/// Check if one or more environment variables are set.
///
/// # Use Cases
/// - Pre-flight checks before running a script
/// - Validation in CI/CD pipelines
/// - Debugging missing configuration
///
/// # Strict Mode
/// With `--strict`, the command exits with code 1 if ANY variable
/// is missing. Without it, it's purely informational.
fn cmd_check(names: &[String], strict: bool) -> Result<()> {
    let mut all_set = true;

    for name in names {
        match env::var(name) {
            Ok(_) => {
                // Variable is set (we don't care about the value)
                println!("{} {} is set", "✓".green().bold(), name.cyan());
            }
            Err(_) => {
                // Variable is not set
                println!("{} {} is NOT set", "✗".red().bold(), name.yellow());
                all_set = false;
            }
        }
    }

    // In strict mode, exit with error if any variable was missing
    if strict && !all_set {
        std::process::exit(1);
    }

    Ok(())
}

/// Export environment variables in different formats.
///
/// # Format Details
///
/// ## Shell Format
/// ```bash
/// export KEY='value with spaces'
/// ```
/// Single quotes prevent shell expansion. Internal single quotes are
/// escaped using the `'\''` technique (end quote, literal quote, start quote).
///
/// ## Docker Format
/// ```text
/// KEY=value
/// ```
/// Simple key=value format for Docker's `--env-file` option.
/// No quoting, so values with special characters may need care.
///
/// ## JSON Format
/// ```json
/// {"KEY": "value"}
/// ```
/// Standard JSON object, useful for programmatic consumption.
///
/// ## TOML Format
/// ```toml
/// KEY = "value"
/// ```
/// TOML key-value pairs with proper string escaping.
fn cmd_export(format: ExportFormat, prefix: Option<&str>) -> Result<()> {
    // Use BTreeMap for consistent (sorted) output order
    let vars: BTreeMap<String, String> = env::vars()
        .filter(|(k, _)| {
            if let Some(p) = prefix {
                k.starts_with(p)
            } else {
                true
            }
        })
        .collect();

    match format {
        ExportFormat::Shell => {
            for (key, value) in &vars {
                // Escape single quotes in value using the shell idiom:
                // 'don'\''t' = 'don' + \' + 't'
                // This ends the string, adds a literal ', then continues
                let escaped = value.replace('\'', "'\\''");
                println!("export {}='{}'", key, escaped);
            }
        }
        ExportFormat::Docker => {
            // Docker .env file format: simple KEY=VALUE
            // No quoting needed for Docker's parser
            for (key, value) in &vars {
                println!("{}={}", key, value);
            }
        }
        ExportFormat::Json => {
            // serde_json handles all escaping automatically
            println!("{}", serde_json::to_string_pretty(&vars)?);
        }
        ExportFormat::Toml => {
            for (key, value) in &vars {
                // TOML string escaping: backslash and double-quote
                let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
                println!("{} = \"{}\"", key, escaped);
            }
        }
    }

    Ok(())
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// PATH should exist on all Unix-like systems and Windows.
    #[test]
    fn test_get_existing_var() {
        assert!(env::var("PATH").is_ok());
    }
}
