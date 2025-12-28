//! # Config Command Implementation
//!
//! This module manages application configuration stored in a TOML file.
//! It provides a hierarchical key-value store with persistence.
//!
//! ## Configuration File Location
//!
//! Uses platform-specific directories via the `directories` crate:
//! - **Linux**: `~/.config/dx/config.toml`
//! - **macOS**: `~/Library/Application Support/dx/config.toml`
//! - **Windows**: `C:\Users\<User>\AppData\Roaming\dx\config.toml`
//!
//! ## Key Concepts
//!
//! ### TOML Format
//! TOML (Tom's Obvious Minimal Language) is a configuration file format
//! that's easy to read and write. It maps directly to hash tables.
//!
//! ```toml
//! [section]
//! key = "value"
//! number = 42
//! ```
//!
//! ### Dot-Notation Keys
//! Nested values can be accessed with dot notation:
//! - `editor` → top-level key
//! - `output.colors` → nested under `[output]` section
//!
//! ## Example Usage
//! ```bash
//! dx config show                    # Show all config (TOML format)
//! dx config show --format json      # Show as JSON
//! dx config get editor              # Get a specific value
//! dx config set editor vim          # Set a value
//! dx config unset editor            # Remove a value
//! dx config list                    # List all keys
//! dx config list --values           # List keys with values
//! dx config path                    # Show config file location
//! dx config edit                    # Open in $EDITOR
//! dx config reset                   # Reset to defaults
//! ```
//!
//! ## External Documentation
//! - TOML: <https://toml.io/>
//! - directories crate: <https://docs.rs/directories>

use crate::cli::commands::config::{ConfigArgs, ConfigCommand, ConfigFormat};
use crate::config::Settings;
use anyhow::{Context, Result};
use colored::Colorize;

/// Run the config command, dispatching to the appropriate subcommand.
///
/// Config has many subcommands for different operations:
/// - `show`: Display entire configuration
/// - `get`/`set`/`unset`: Manage individual keys
/// - `list`: Show all keys
/// - `path`: Show file location
/// - `edit`: Open in editor
/// - `reset`: Restore defaults
pub fn run(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommand::Show { key, format } => cmd_show(key.as_deref(), format),
        ConfigCommand::Get { key } => cmd_get(&key),
        ConfigCommand::Set { key, value } => cmd_set(&key, &value),
        ConfigCommand::Unset { key } => cmd_unset(&key),
        ConfigCommand::List { values } => cmd_list(values),
        ConfigCommand::Path => cmd_path(),
        ConfigCommand::Edit => cmd_edit(),
        ConfigCommand::Reset { force } => cmd_reset(force),
    }
}

/// Show current configuration, optionally filtered to a specific key.
///
/// # Output Formats
/// - **TOML**: Human-readable, matches the file format
/// - **JSON**: Machine-readable, useful for scripting
///
/// # Key Lookup
/// If a key is provided, only that value is printed (raw, no formatting).
/// This is useful for shell scripts: `editor=$(dx config show editor)`
fn cmd_show(key: Option<&str>, format: ConfigFormat) -> Result<()> {
    // Load settings from disk (or defaults if no file exists)
    let settings = Settings::load()?;

    if let Some(k) = key {
        // Single key lookup
        if let Some(value) = settings.get(k) {
            println!("{}", value);
        } else {
            eprintln!("{} Key '{}' not found", "✗".red().bold(), k);
            std::process::exit(1);
        }
    } else {
        // Show entire configuration
        match format {
            ConfigFormat::Toml => {
                // to_string_pretty adds nice formatting
                println!("{}", toml::to_string_pretty(&settings)?);
            }
            ConfigFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&settings)?);
            }
        }
    }

    Ok(())
}

/// Get a specific configuration value by key.
///
/// Similar to `show <key>` but more explicit in intent.
/// Exits with code 1 if the key doesn't exist.
fn cmd_get(key: &str) -> Result<()> {
    let settings = Settings::load()?;

    if let Some(value) = settings.get(key) {
        println!("{}", value);
    } else {
        eprintln!("{} Key '{}' not found", "✗".red().bold(), key);
        std::process::exit(1);
    }

    Ok(())
}

/// Set a configuration value.
///
/// # Persistence
/// Changes are immediately saved to disk. The Settings struct
/// handles serialization to TOML format.
///
/// # Creating Nested Keys
/// Dot-notation creates nested structures automatically:
/// `dx config set output.colors true` creates `[output]` section.
fn cmd_set(key: &str, value: &str) -> Result<()> {
    // Load existing settings (or defaults)
    let mut settings = Settings::load()?;

    // Set the value (handles dot-notation internally)
    settings.set(key, value)?;

    // Persist to disk
    settings.save()?;

    // Confirm the change to the user
    println!(
        "{} Set {} = {}",
        "✓".green().bold(),
        key.cyan(),
        value.yellow()
    );

    Ok(())
}

/// Remove a configuration key.
///
/// After unsetting, the key will return to its default value
/// (if one exists) or be absent from the config.
fn cmd_unset(key: &str) -> Result<()> {
    let mut settings = Settings::load()?;
    settings.unset(key)?;
    settings.save()?;

    println!("{} Removed {}", "✓".green().bold(), key.cyan());

    Ok(())
}

/// List all configuration keys.
///
/// # With Values
/// The `--values` flag shows `key = value` pairs instead of just keys.
/// Useful for quickly reviewing the entire configuration.
fn cmd_list(show_values: bool) -> Result<()> {
    let settings = Settings::load()?;

    // list() returns an iterator over (key, value) pairs
    for (key, value) in settings.list() {
        if show_values {
            println!("{} = {}", key.cyan(), value);
        } else {
            println!("{}", key.cyan());
        }
    }

    Ok(())
}

/// Show the path to the configuration file.
///
/// Useful for:
/// - Finding where config is stored
/// - Backing up configuration
/// - Manual editing with external tools
fn cmd_path() -> Result<()> {
    let path = Settings::config_path()?;
    println!("{}", path.display());
    Ok(())
}

/// Open the configuration file in the user's preferred editor.
///
/// # Editor Selection
/// Uses the `EDITOR` environment variable, falling back to `nano`.
/// Common values: `vim`, `nvim`, `code`, `nano`, `emacs`.
///
/// # File Creation
/// If the config file doesn't exist, we create it with defaults
/// first. This ensures the editor has something to open.
fn cmd_edit() -> Result<()> {
    let path = Settings::config_path()?;

    // Create config file if it doesn't exist
    // This prevents the editor from opening a non-existent file
    if !path.exists() {
        let settings = Settings::default();
        settings.save()?;
    }

    // Get editor from environment, default to nano
    // On macOS/Linux, nano is usually available
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

    // Spawn the editor process and wait for it to complete
    // The user edits the file, then control returns here
    std::process::Command::new(&editor)
        .arg(&path)
        .status()
        .with_context(|| format!("Failed to open editor: {}", editor))?;

    Ok(())
}

/// Reset configuration to default values.
///
/// # Confirmation
/// Without `--force`, prompts the user for confirmation.
/// This prevents accidental data loss.
///
/// # What Gets Reset
/// The entire config file is replaced with defaults.
/// All custom settings are lost.
fn cmd_reset(force: bool) -> Result<()> {
    if !force {
        // Simple stdin-based confirmation
        use std::io::{self, Write};

        print!("Reset configuration to defaults? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled");
            return Ok(());
        }
    }

    // Create default settings and save
    let settings = Settings::default();
    settings.save()?;

    println!("{} Configuration reset to defaults", "✓".green().bold());

    Ok(())
}
