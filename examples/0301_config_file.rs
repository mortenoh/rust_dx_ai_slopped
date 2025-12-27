//! # Configuration File Management
//!
//! This example shows how to manage TOML config with the directories crate.
//!
//! Run with: `cargo run --example 0301_config_file`

#![allow(dead_code)]

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// =========================================================================
// CONFIGURATION STRUCTURE
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub output: OutputConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_true")]
    pub color: bool,
    #[serde(default)]
    pub verbose: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_indent")]
    pub json_indent: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    #[serde(default = "default_algorithm")]
    pub hash_algorithm: String,
    #[serde(default = "default_uuid_version")]
    pub uuid_version: String,
}

fn default_true() -> bool {
    true
}
fn default_format() -> String {
    "text".to_string()
}
fn default_indent() -> usize {
    2
}
fn default_algorithm() -> String {
    "sha256".to_string()
}
fn default_uuid_version() -> String {
    "v4".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                color: true,
                verbose: false,
            },
            output: OutputConfig {
                format: "text".to_string(),
                json_indent: 2,
            },
            defaults: DefaultsConfig {
                hash_algorithm: "sha256".to_string(),
                uuid_version: "v4".to_string(),
            },
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            color: true,
            verbose: false,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: "text".to_string(),
            json_indent: 2,
        }
    }
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            hash_algorithm: "sha256".to_string(),
            uuid_version: "v4".to_string(),
        }
    }
}

// =========================================================================
// CONFIG FILE OPERATIONS
// =========================================================================

/// Get the config directory path
pub fn config_dir() -> Option<PathBuf> {
    ProjectDirs::from("com", "dx", "dx").map(|dirs| dirs.config_dir().to_path_buf())
}

/// Get the config file path
pub fn config_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("config.toml"))
}

/// Load configuration from disk
pub fn load_config() -> Config {
    if let Some(path) = config_path() {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str(&content) {
                    return config;
                }
            }
        }
    }
    Config::default()
}

/// Save configuration to disk
pub fn save_config(config: &Config) -> Result<(), String> {
    let dir = config_dir().ok_or("Could not determine config directory")?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let path = dir.join("config.toml");
    let content = toml::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    println!("=== Configuration File Management ===\n");

    // =========================================================================
    // DIRECTORIES CRATE
    // =========================================================================

    println!("--- directories Crate ---");
    println!(
        r#"
The `directories` crate provides cross-platform paths:

use directories::ProjectDirs;

let dirs = ProjectDirs::from("com", "mycompany", "myapp");

dirs.config_dir()     // ~/.config/myapp (Linux)
                      // ~/Library/Application Support/myapp (macOS)
                      // C:\Users\X\AppData\Roaming\myapp (Windows)

dirs.data_dir()       // App data
dirs.cache_dir()      // Cache files
dirs.state_dir()      // Runtime state
"#
    );

    println!();

    // =========================================================================
    // CONFIG PATHS
    // =========================================================================

    println!("--- Config Paths on This System ---");
    if let Some(dir) = config_dir() {
        println!("  Config dir:  {}", dir.display());
    }
    if let Some(path) = config_path() {
        println!("  Config file: {}", path.display());
    }

    println!();

    // =========================================================================
    // DEFAULT CONFIG
    // =========================================================================

    println!("--- Default Configuration ---");
    let config = Config::default();
    println!("{}", toml::to_string_pretty(&config).unwrap());

    println!();

    // =========================================================================
    // SERDE WITH DEFAULTS
    // =========================================================================

    println!("--- Serde with Defaults ---");
    println!(
        r##"
Use #[serde(default)] for missing fields:

#[derive(Serialize, Deserialize)]
pub struct Config {{
    #[serde(default)]          // Uses Default trait
    pub verbose: bool,

    #[serde(default = "default_port")]  // Custom default
    pub port: u16,
}}

fn default_port() -> u16 {{ 8080 }}

This allows partial config files - only specify
what you want to change from defaults.
"##
    );

    println!();

    // =========================================================================
    // LOADING PATTERN
    // =========================================================================

    println!("--- Loading Pattern ---");
    println!(
        r#"
pub fn load_config() -> Config {{
    let path = config_path()?;

    if path.exists() {{
        let content = fs::read_to_string(&path)?;
        toml::from_str(&content)?
    }} else {{
        Config::default()
    }}
}}

// Usage in commands:
let config = load_config();
let algorithm = &config.defaults.hash_algorithm;
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Config file management:");
    println!("  1. Use `directories` for cross-platform paths");
    println!("  2. Use TOML for human-readable config");
    println!("  3. Use #[serde(default)] for optional fields");
    println!("  4. Load with fallback to defaults");
    println!("  5. Create config dir on first save");
}
