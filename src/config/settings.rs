//! # Application Settings Module
//!
//! This module provides persistent configuration storage for the dx CLI.
//! Settings are stored in a TOML file at a platform-specific location.
//!
//! ## Configuration File Location
//!
//! Uses the `directories` crate for cross-platform paths:
//!
//! | Platform | Path |
//! |----------|------|
//! | Linux | `~/.config/dx/config.toml` |
//! | macOS | `~/Library/Application Support/dx/config.toml` |
//! | Windows | `C:\Users\<User>\AppData\Roaming\dx\config.toml` |
//!
//! ## Configuration Structure
//!
//! ```toml
//! [general]
//! output_format = "text"
//! color = true
//!
//! [hash]
//! algorithm = "sha256"
//!
//! [output]
//! json_indent = 2
//!
//! # Custom keys are also supported
//! my_custom_key = "value"
//! ```
//!
//! ## Key Concepts
//!
//! ### Serde Attributes
//! - `#[serde(default)]`: Use `Default::default()` if field is missing
//! - `#[serde(default = "fn_name")]`: Use custom function for default
//! - `#[serde(flatten)]`: Merge nested struct fields into parent level
//!
//! ### BTreeMap for Custom Keys
//! Using `BTreeMap` instead of `HashMap` ensures:
//! - Sorted key order (deterministic output)
//! - Consistent serialization (easier diffs)
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use rust_cli_complete::config::Settings;
//!
//! // Load settings (creates defaults if file doesn't exist)
//! let settings = Settings::load()?;
//!
//! // Access a value
//! if let Some(algo) = settings.get("hash.algorithm") {
//!     println!("Default algorithm: {}", algo);
//! }
//!
//! // Modify and save
//! let mut settings = Settings::load()?;
//! settings.set("hash.algorithm", "sha512")?;
//! settings.save()?;
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! ## External Documentation
//! - directories crate: <https://docs.rs/directories>
//! - TOML specification: <https://toml.io/>
//! - Serde attributes: <https://serde.rs/attributes.html>

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

// =============================================================================
// MAIN SETTINGS STRUCT
// =============================================================================

/// Application settings container.
///
/// This is the top-level struct that holds all configuration.
/// It's serialized to/from TOML format for persistence.
///
/// # Serde Attributes
///
/// - `Serialize`: Enables conversion to TOML/JSON
/// - `Deserialize`: Enables parsing from TOML/JSON
/// - `Default`: Provides sensible defaults for all fields
///
/// # Field Attributes
///
/// - `#[serde(default)]`: If the field is missing when deserializing,
///   use the type's `Default` implementation instead of failing.
/// - `#[serde(flatten)]`: Merge the `custom` map's keys directly into
///   the parent object (no nesting under a "custom" key).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    /// General application settings.
    /// Contains format preferences and color settings.
    #[serde(default)]
    pub general: GeneralSettings,

    /// Hash command defaults.
    /// Users can set their preferred algorithm here.
    #[serde(default)]
    pub hash: HashSettings,

    /// Output formatting settings.
    /// Controls JSON indentation and similar options.
    #[serde(default)]
    pub output: OutputSettings,

    /// Custom key-value settings.
    ///
    /// The `#[serde(flatten)]` attribute merges these into the root level.
    /// This allows users to add arbitrary keys without modifying the schema.
    ///
    /// Example TOML:
    /// ```toml
    /// [general]
    /// color = true
    ///
    /// my_custom_key = "value"  # Goes into `custom` map
    /// another_key = "data"     # Also goes into `custom`
    /// ```
    ///
    /// We use `toml::Value` to accept any TOML type (string, number, bool, etc).
    #[serde(default, flatten)]
    pub custom: BTreeMap<String, toml::Value>,
}

// =============================================================================
// SECTION STRUCTS
// =============================================================================

/// General application settings.
///
/// These settings affect the overall behavior of the CLI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    /// Default output format ("text", "json", etc).
    ///
    /// The `#[serde(default = "default_output_format")]` attribute specifies
    /// a function to call for the default value. This is more flexible than
    /// `#[serde(default)]` which uses `Default::default()` (empty string).
    #[serde(default = "default_output_format")]
    pub output_format: String,

    /// Enable colored output.
    ///
    /// When true, output may include ANSI color codes.
    /// Can be overridden by the `--no-color` CLI flag.
    #[serde(default = "default_true")]
    pub color: bool,
}

/// Manual Default implementation for GeneralSettings.
///
/// We implement `Default` manually instead of deriving because:
/// 1. We need specific default values (not just empty strings/false)
/// 2. The default functions are also used by serde for missing fields
impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            output_format: default_output_format(),
            color: true,
        }
    }
}

/// Hash command default settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashSettings {
    /// Default hashing algorithm.
    ///
    /// Valid values: "sha256", "sha512", "md5"
    /// SHA-256 is a good default: secure, fast, widely supported.
    #[serde(default = "default_algorithm")]
    pub algorithm: String,
}

impl Default for HashSettings {
    fn default() -> Self {
        Self {
            algorithm: default_algorithm(),
        }
    }
}

/// Output formatting settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSettings {
    /// JSON indentation level (number of spaces).
    ///
    /// Common values:
    /// - 2: Compact but readable (default)
    /// - 4: More spacious, matches many style guides
    /// - 0: No indentation (compact output)
    #[serde(default = "default_indent")]
    pub json_indent: usize,
}

impl Default for OutputSettings {
    fn default() -> Self {
        Self {
            json_indent: default_indent(),
        }
    }
}

// =============================================================================
// DEFAULT VALUE FUNCTIONS
// =============================================================================
//
// These functions are referenced by serde's `default = "fn_name"` attribute.
// They must be free functions (not methods) and return the field type.

/// Default output format is plain text.
fn default_output_format() -> String {
    "text".to_string()
}

/// Default for boolean true (used for color setting).
fn default_true() -> bool {
    true
}

/// Default hash algorithm is SHA-256.
/// It's a good balance of security, speed, and compatibility.
fn default_algorithm() -> String {
    "sha256".to_string()
}

/// Default JSON indentation is 2 spaces.
fn default_indent() -> usize {
    2
}

// =============================================================================
// SETTINGS IMPLEMENTATION
// =============================================================================

impl Settings {
    /// Get the configuration file path.
    ///
    /// Uses the `directories` crate which follows platform conventions:
    /// - Linux: Uses XDG Base Directory spec (`~/.config/`)
    /// - macOS: Uses `~/Library/Application Support/`
    /// - Windows: Uses `%APPDATA%`
    ///
    /// # ProjectDirs Arguments
    ///
    /// `ProjectDirs::from(qualifier, organization, application)`:
    /// - `qualifier`: Reverse-domain (used mainly on macOS)
    /// - `organization`: Company/author name
    /// - `application`: Application name
    ///
    /// These combine to create a unique directory for the app.
    ///
    /// # Returns
    ///
    /// Full path to the config file (e.g., `~/.config/dx/config.toml`)
    pub fn config_path() -> Result<PathBuf> {
        // ProjectDirs::from returns Option because it can fail
        // if the home directory can't be determined
        let proj_dirs =
            ProjectDirs::from("com", "dx", "dx").context("Could not determine config directory")?;

        let config_dir = proj_dirs.config_dir();

        // Create the config directory if it doesn't exist.
        // create_dir_all is idempotent - safe to call if dir exists.
        fs::create_dir_all(config_dir).context("Failed to create config directory")?;

        Ok(config_dir.join("config.toml"))
    }

    /// Load settings from disk.
    ///
    /// # Behavior
    ///
    /// - If config file exists: Parse it and return settings
    /// - If config file doesn't exist: Return default settings
    /// - If config file is invalid TOML: Return an error
    ///
    /// # Partial Configuration
    ///
    /// Thanks to `#[serde(default)]` on all fields, users only need
    /// to specify the settings they want to change. Missing fields
    /// get their default values.
    ///
    /// Example: This minimal config is valid:
    /// ```toml
    /// [hash]
    /// algorithm = "md5"
    /// ```
    /// All other settings use defaults.
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if path.exists() {
            // Read file contents as a string
            let content = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read {}", path.display()))?;

            // Parse TOML into our Settings struct
            // toml::from_str uses serde deserialization
            toml::from_str(&content).context("Failed to parse config file")
        } else {
            // No config file - return defaults
            // The caller can save() to create the file
            Ok(Self::default())
        }
    }

    /// Save settings to disk.
    ///
    /// Creates the config file if it doesn't exist, or overwrites it.
    ///
    /// # TOML Formatting
    ///
    /// Uses `to_string_pretty()` for human-readable output with:
    /// - Proper indentation
    /// - Section headers like `[general]`
    /// - Comments preserved (if any were in the original)
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        // Serialize to pretty TOML format
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        // Write atomically (well, as atomic as a simple write can be)
        // For true atomicity, you'd write to a temp file then rename
        fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))?;

        Ok(())
    }

    /// Get a value by key using dot-notation.
    ///
    /// # Path Syntax
    ///
    /// - `"general.color"` → `settings.general.color`
    /// - `"hash.algorithm"` → `settings.hash.algorithm`
    /// - `"custom_key"` → `settings.custom.get("custom_key")`
    ///
    /// # Returns
    ///
    /// - `Some(String)`: The value as a string
    /// - `None`: Key not found
    ///
    /// # Note
    ///
    /// All values are converted to strings for simplicity.
    /// Booleans become "true"/"false", numbers become their string form.
    pub fn get(&self, key: &str) -> Option<String> {
        // Split the key by dots to navigate the nested structure
        let parts: Vec<&str> = key.split('.').collect();

        // Match known paths to their struct fields
        // This is explicit rather than using reflection (which Rust doesn't have)
        match parts.as_slice() {
            ["general", "output_format"] => Some(self.general.output_format.clone()),
            ["general", "color"] => Some(self.general.color.to_string()),
            ["hash", "algorithm"] => Some(self.hash.algorithm.clone()),
            ["output", "json_indent"] => Some(self.output.json_indent.to_string()),
            // Fall back to custom keys for anything else
            _ => self.custom.get(key).map(|v| v.to_string()),
        }
    }

    /// Set a value by key using dot-notation.
    ///
    /// # Path Syntax
    ///
    /// Same as `get()`:
    /// - `"general.color"` → sets `settings.general.color`
    /// - `"custom_key"` → adds to `settings.custom`
    ///
    /// # Type Conversion
    ///
    /// The value string is parsed according to the field type:
    /// - Booleans: "true"/"false"
    /// - Numbers: parsed as usize
    /// - Strings: used as-is
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails (e.g., "not_a_bool" for a bool field).
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let parts: Vec<&str> = key.split('.').collect();

        match parts.as_slice() {
            ["general", "output_format"] => {
                self.general.output_format = value.to_string();
            }
            ["general", "color"] => {
                // parse::<bool>() accepts "true" or "false"
                self.general.color = value.parse().context("Invalid boolean value")?;
            }
            ["hash", "algorithm"] => {
                self.hash.algorithm = value.to_string();
            }
            ["output", "json_indent"] => {
                self.output.json_indent = value.parse().context("Invalid number")?;
            }
            _ => {
                // Unknown keys go into the custom map
                // Store as TOML string value
                self.custom
                    .insert(key.to_string(), toml::Value::String(value.to_string()));
            }
        }

        Ok(())
    }

    /// Remove a value by key.
    ///
    /// # Behavior
    ///
    /// - For custom keys: Removes from the custom map
    /// - For built-in keys: No-op (they always exist with defaults)
    ///
    /// To "reset" a built-in key, set it to its default value instead.
    pub fn unset(&mut self, key: &str) -> Result<()> {
        // Only custom keys can be removed
        // Built-in keys always exist with their defaults
        self.custom.remove(key);
        Ok(())
    }

    /// List all settings as key-value pairs.
    ///
    /// Returns a vector of (key, value) tuples for all settings,
    /// including both built-in and custom keys.
    ///
    /// # Use Cases
    ///
    /// - Display all settings to the user
    /// - Export configuration
    /// - Debugging
    ///
    /// # Ordering
    ///
    /// Built-in settings come first, followed by custom keys in
    /// alphabetical order (due to BTreeMap).
    pub fn list(&self) -> Vec<(String, String)> {
        // Start with built-in settings in a logical order
        let mut result = vec![
            (
                "general.output_format".to_string(),
                self.general.output_format.clone(),
            ),
            ("general.color".to_string(), self.general.color.to_string()),
            ("hash.algorithm".to_string(), self.hash.algorithm.clone()),
            (
                "output.json_indent".to_string(),
                self.output.json_indent.to_string(),
            ),
        ];

        // Append custom settings
        // BTreeMap iterates in sorted key order
        for (k, v) in &self.custom {
            result.push((k.clone(), v.to_string()));
        }

        result
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that default settings have expected values.
    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.general.output_format, "text");
        assert!(settings.general.color);
        assert_eq!(settings.hash.algorithm, "sha256");
    }

    /// Test get/set for built-in keys.
    #[test]
    fn test_get_set() {
        let mut settings = Settings::default();
        settings.set("hash.algorithm", "md5").unwrap();
        assert_eq!(settings.get("hash.algorithm"), Some("md5".to_string()));
    }

    /// Test custom keys are stored and retrieved correctly.
    #[test]
    fn test_custom_keys() {
        let mut settings = Settings::default();
        settings.set("my.custom.key", "value").unwrap();
        assert!(settings.get("my.custom.key").is_some());
    }
}
