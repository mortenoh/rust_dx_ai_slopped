//! # Clap Derive Basics
//!
//! This example covers the fundamentals of clap's derive macros.
//!
//! Run with: `cargo run --example 0102_clap_derive_basics`

#![allow(dead_code)]

use clap::{Parser, ValueEnum};

// =========================================================================
// BASIC CLI STRUCT
// =========================================================================

/// A simple CLI demonstrating clap derive basics
#[derive(Parser, Debug)]
#[command(
    name = "example",
    author = "Developer",
    version = "1.0.0",
    about = "Example CLI for learning clap",
    long_about = "This is a longer description that appears with --help"
)]
struct SimpleCli {
    /// A required positional argument
    name: String,

    /// An optional positional argument
    greeting: Option<String>,

    /// A flag (boolean, present = true)
    #[arg(short, long)]
    verbose: bool,

    /// A counted flag (-v, -vv, -vvv)
    #[arg(short = 'V', long, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// An optional value with short and long form
    #[arg(short, long)]
    output: Option<String>,

    /// A required option (not positional)
    #[arg(short, long)]
    config: Option<String>,

    /// A value with a default
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// An enum value
    #[arg(short, long, default_value = "info")]
    level: LogLevel,

    /// Multiple values (can be repeated: -f a -f b -f c)
    #[arg(short, long)]
    files: Vec<String>,
}

/// Log level enum
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum LogLevel {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

// =========================================================================
// ARGUMENT ATTRIBUTES
// =========================================================================

/// Demonstrating various argument attributes
#[derive(Parser, Debug)]
#[command(name = "attrs")]
struct AttributesCli {
    /// Environment variable fallback
    #[arg(long, env = "MY_API_KEY")]
    api_key: Option<String>,

    /// Value with validation
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..=65535))]
    port: Option<u16>,

    /// Hidden from help
    #[arg(long, hide = true)]
    secret: Option<String>,

    /// Deprecated argument
    #[arg(long, hide = true)]
    #[deprecated(note = "Use --new-flag instead")]
    old_flag: bool,

    /// Conflicts with another arg
    #[arg(long, conflicts_with = "json")]
    yaml: bool,

    #[arg(long)]
    json: bool,

    /// Requires another arg
    #[arg(long, requires = "output")]
    compress: bool,

    #[arg(long)]
    output: Option<String>,

    /// Global flag (applies to subcommands too)
    #[arg(long, global = true)]
    debug: bool,
}

fn main() {
    println!("=== Clap Derive Basics ===\n");

    // =========================================================================
    // DERIVE ATTRIBUTES
    // =========================================================================

    println!("--- #[derive(Parser)] ---");
    println!(
        r#"
The Parser derive macro creates a full argument parser:

#[derive(Parser, Debug)]
#[command(name = "myapp", version, author, about)]
struct Cli {{
    // arguments go here
}}

Key struct-level attributes:
  #[command(name = "...")] - executable name
  #[command(version)]      - from Cargo.toml
  #[command(author)]       - from Cargo.toml
  #[command(about)]        - short description
  #[command(long_about)]   - --help description
"#
    );

    println!();

    // =========================================================================
    // ARGUMENT TYPES
    // =========================================================================

    println!("--- Argument Types ---");
    println!(
        r#"
1. POSITIONAL ARGUMENTS
   name: String              // Required positional
   name: Option<String>      // Optional positional
   names: Vec<String>        // Multiple positional

2. FLAGS (boolean)
   #[arg(short, long)]
   verbose: bool             // --verbose or -v

3. OPTIONS (with values)
   #[arg(short, long)]
   output: Option<String>    // --output FILE or -o FILE

   #[arg(short, long, default_value = "8080")]
   port: u16                 // --port 3000 (default: 8080)

4. COUNTED FLAGS
   #[arg(short, long, action = ArgAction::Count)]
   verbosity: u8             // -vvv gives 3

5. MULTIPLE VALUES
   #[arg(short, long)]
   files: Vec<String>        // -f a -f b -f c

6. ENUMS (ValueEnum)
   #[arg(short, long)]
   level: LogLevel           // --level debug
"#
    );

    println!();

    // =========================================================================
    // COMMON ATTRIBUTES
    // =========================================================================

    println!("--- Common #[arg(...)] Attributes ---");
    println!(
        r#"
Naming:
  short              // -v (first letter)
  short = 'x'        // -x (specific letter)
  long               // --verbose (field name)
  long = "verb"      // --verb (custom name)

Values:
  default_value = "x"       // String default
  default_value_t = 42      // Typed default
  value_name = "FILE"       // Shown in help
  value_parser = ...        // Custom parsing

Validation:
  required = true           // Must be provided
  conflicts_with = "other"  // Can't use both
  requires = "other"        // Must have other too

Display:
  help = "description"      // Short help
  long_help = "..."         // --help description
  hide = true               // Don't show in help

Environment:
  env = "MY_VAR"            // Fallback to env var

Actions:
  action = ArgAction::Set       // Default
  action = ArgAction::Append    // Collect multiples
  action = ArgAction::Count     // Count occurrences
  action = ArgAction::SetTrue   // Flag behavior
"#
    );

    println!();

    // =========================================================================
    // VALUE ENUM
    // =========================================================================

    println!("--- ValueEnum for Choices ---");
    println!(
        r#"
Define allowed values with an enum:

#[derive(Clone, Copy, ValueEnum)]
enum LogLevel {{
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}}

#[derive(Parser)]
struct Cli {{
    #[arg(long, default_value = "info")]
    level: LogLevel,
}}

Usage:
  myapp --level debug
  myapp --level warn

Clap auto-generates:
  - Case-insensitive matching
  - Help text with valid values
  - Tab completion values
"#
    );

    println!();

    // =========================================================================
    // PARSING
    // =========================================================================

    println!("--- Parsing Arguments ---");
    println!(
        r#"
use clap::Parser;

fn main() {{
    // Parse from std::env::args()
    let cli = Cli::parse();

    // Or try_parse() for Result
    let cli = Cli::try_parse()?;

    // Parse from custom iterator
    let cli = Cli::parse_from(["prog", "--verbose", "name"]);

    // Access fields directly
    println!("Name: {{}}", cli.name);
    if cli.verbose {{
        println!("Verbose mode enabled");
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // EXAMPLE USAGE
    // =========================================================================

    println!("--- Example: Parsing Test Args ---");

    // Simulate parsing (without actually parsing CLI args)
    let simulated_args = ["example", "World", "--verbose", "--port", "3000"];
    match SimpleCli::try_parse_from(simulated_args) {
        Ok(cli) => {
            println!("  Parsed successfully:");
            println!("    name: {}", cli.name);
            println!("    verbose: {}", cli.verbose);
            println!("    port: {}", cli.port);
        }
        Err(e) => {
            println!("  Parse error: {}", e);
        }
    }

    println!();

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Key clap derive concepts:");
    println!("  1. #[derive(Parser)] on struct for CLI definition");
    println!("  2. #[command(...)] for app metadata");
    println!("  3. #[arg(...)] for field configuration");
    println!("  4. #[derive(ValueEnum)] for enum choices");
    println!("  5. Cli::parse() to parse std::env::args()");
}
