//! # Error Handling
//!
//! This example covers error handling patterns for CLI applications
//! using thiserror and anyhow.
//!
//! Run with: `cargo run --example 0104_error_handling`

#![allow(dead_code)]

use std::io;
use std::num::ParseIntError;
use thiserror::Error;

// =========================================================================
// THISERROR - LIBRARY ERRORS
// =========================================================================

/// Custom error type for our application
#[derive(Error, Debug)]
pub enum AppError {
    /// File not found
    #[error("file not found: {path}")]
    FileNotFound { path: String },

    /// Invalid configuration
    #[error("invalid configuration: {message}")]
    InvalidConfig { message: String },

    /// Parse error with source
    #[error("failed to parse number")]
    ParseError(#[from] ParseIntError),

    /// IO error with source
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    /// Generic error with context
    #[error("{context}: {source}")]
    WithContext {
        context: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Validation error
    #[error("validation failed: {0}")]
    Validation(String),
}

// =========================================================================
// ERROR CONSTRUCTION
// =========================================================================

fn demonstrate_error_construction() {
    // Field-based errors
    let _e1 = AppError::FileNotFound {
        path: "/etc/config.toml".to_string(),
    };

    let _e2 = AppError::InvalidConfig {
        message: "missing required field 'api_key'".to_string(),
    };

    // Tuple-based errors
    let _e3 = AppError::Validation("port must be between 1 and 65535".to_string());

    // From conversion (automatic with #[from])
    let parse_err: Result<i32, ParseIntError> = "not a number".parse();
    let _e4: Result<i32, AppError> = parse_err.map_err(AppError::from);
}

// =========================================================================
// ANYHOW - APPLICATION ERRORS
// =========================================================================

use anyhow::{anyhow, bail, Context, Result};

/// Example function returning anyhow::Result
fn load_config(path: &str) -> Result<String> {
    // Use context() to add information
    std::fs::read_to_string(path).context(format!("Failed to load config from {}", path))
}

/// Example function with bail! macro
fn validate_port(port: u16) -> Result<()> {
    if port == 0 {
        bail!("Port cannot be zero");
    }
    if port < 1024 {
        bail!("Port {} requires root privileges", port);
    }
    Ok(())
}

/// Example function with anyhow! macro
fn process_data(data: &str) -> Result<i32> {
    if data.is_empty() {
        return Err(anyhow!("Data cannot be empty"));
    }

    data.parse::<i32>()
        .context("Failed to parse data as integer")
}

// =========================================================================
// ERROR CHAIN PATTERN
// =========================================================================

fn high_level_operation() -> Result<()> {
    // Each level adds context
    low_level_operation().context("High level operation failed")?;
    Ok(())
}

fn low_level_operation() -> Result<()> {
    // This might fail
    let _value = "abc"
        .parse::<i32>()
        .context("Could not parse value in low level")?;
    Ok(())
}

// =========================================================================
// COMBINING THISERROR AND ANYHOW
// =========================================================================

/// Use thiserror for library code
pub mod library {
    use super::*;

    #[derive(Error, Debug)]
    pub enum LibraryError {
        #[error("invalid input: {0}")]
        InvalidInput(String),

        #[error("operation failed")]
        OperationFailed(#[source] io::Error),
    }

    pub fn library_function(input: &str) -> std::result::Result<(), LibraryError> {
        if input.is_empty() {
            return Err(LibraryError::InvalidInput("input cannot be empty".into()));
        }
        Ok(())
    }
}

/// Use anyhow in application code
pub mod application {
    use super::*;

    pub fn run(input: &str) -> Result<()> {
        // Convert library errors to anyhow with context
        library::library_function(input).context("Library call failed")?;
        Ok(())
    }
}

// =========================================================================
// EXIT CODES
// =========================================================================

/// Standard exit codes for CLI applications
mod exit_codes {
    pub const SUCCESS: i32 = 0;
    pub const GENERAL_ERROR: i32 = 1;
    pub const USAGE_ERROR: i32 = 2;
    pub const IO_ERROR: i32 = 74;
    pub const CONFIG_ERROR: i32 = 78;
}

fn main() {
    println!("=== Error Handling ===\n");

    // =========================================================================
    // THISERROR OVERVIEW
    // =========================================================================

    println!("--- thiserror for Custom Errors ---");
    println!(
        r##"
thiserror creates std::error::Error implementations:

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {{
    // Simple message
    #[error("file not found: {{path}}")]
    FileNotFound {{ path: String }},

    // With source error (automatic From impl)
    #[error("IO error")]
    IoError(#[from] std::io::Error),

    // With explicit source
    #[error("{{context}}")]
    WithContext {{
        context: String,
        #[source]
        source: Box<dyn std::error::Error>,
    }},
}}

Key attributes:
  #[error("...")]     - Display message (supports {{field}})
  #[from]             - Auto-impl From trait
  #[source]           - Chain to cause
"##
    );

    println!();

    // =========================================================================
    // ANYHOW OVERVIEW
    // =========================================================================

    println!("--- anyhow for Application Errors ---");
    println!(
        r#"
anyhow provides flexible error handling for applications:

use anyhow::{{anyhow, bail, Context, Result}};

// Result<T> is alias for Result<T, anyhow::Error>
fn load_file(path: &str) -> Result<String> {{
    // Add context to any error
    std::fs::read_to_string(path)
        .context("Failed to load file")?

    // Or create errors directly
    if path.is_empty() {{
        bail!("Path cannot be empty");
    }}

    // Or with anyhow! macro
    return Err(anyhow!("Something went wrong"));
}}

Key features:
  Result<T>      - Convenient type alias
  .context()     - Add context to errors
  bail!()        - Early return with error
  anyhow!()      - Create error inline
"#
    );

    println!();

    // =========================================================================
    // ERROR CHAIN
    // =========================================================================

    println!("--- Error Chain Pattern ---");
    println!(
        r#"
Build informative error chains:

fn main() -> Result<()> {{
    process_request()
        .context("Request processing failed")?;
    Ok(())
}}

fn process_request() -> Result<()> {{
    load_config("/etc/app.toml")
        .context("Could not load configuration")?;
    Ok(())
}}

fn load_config(path: &str) -> Result<Config> {{
    let content = std::fs::read_to_string(path)
        .context("Failed to read file")?;
    parse_config(&content)
        .context("Failed to parse TOML")
}}

Error output:
  Error: Request processing failed

  Caused by:
      0: Could not load configuration
      1: Failed to read file
      2: No such file or directory (os error 2)
"#
    );

    println!();

    // =========================================================================
    // BEST PRACTICES
    // =========================================================================

    println!("--- Best Practices ---");
    println!(
        r#"
1. USE THISERROR FOR LIBRARIES
   - Explicit error types
   - Users can match on variants
   - Good for public APIs

2. USE ANYHOW FOR APPLICATIONS
   - Flexible error handling
   - Rich context chains
   - Easy error propagation

3. COMBINE BOTH
   - thiserror in lib/modules
   - anyhow at application level
   - Convert with .context()

4. MEANINGFUL MESSAGES
   - What happened
   - What was being attempted
   - Any relevant values

5. EXIT CODES
   pub const SUCCESS: i32 = 0;
   pub const ERROR: i32 = 1;
   pub const USAGE: i32 = 2;
"#
    );

    println!();

    // =========================================================================
    // EXAMPLE: ERROR IN ACTION
    // =========================================================================

    println!("--- Example: Error Chain ---");

    let result = high_level_operation();
    if let Err(e) = result {
        println!("  Error: {}", e);
        println!("  Chain:");
        for (i, cause) in e.chain().enumerate() {
            println!("    {}: {}", i, cause);
        }
    }

    println!();

    // =========================================================================
    // CLI MAIN PATTERN
    // =========================================================================

    println!("--- CLI main() Pattern ---");
    println!(
        r#"
fn main() {{
    if let Err(e) = run() {{
        eprintln!("Error: {{:#}}", e);  // Full chain
        std::process::exit(1);
    }}
}}

fn run() -> Result<()> {{
    let cli = Cli::parse();

    match cli.command {{
        Commands::Hash(args) => hash::run(args),
        Commands::Encode(args) => encode::run(args),
    }}
}}

Using {{:#}} shows the full error chain.
Using {{}} shows just the top-level error.
"#
    );

    println!();

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Error handling patterns:");
    println!("  1. thiserror for custom error types in libraries");
    println!("  2. anyhow for flexible errors in applications");
    println!("  3. .context() to build informative error chains");
    println!("  4. bail!/anyhow! for quick error creation");
    println!("  5. Combine both: thiserror in modules, anyhow at top");
    println!("  6. Use proper exit codes for CLI applications");
}
