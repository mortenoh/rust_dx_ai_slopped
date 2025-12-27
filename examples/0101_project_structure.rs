//! # Project Structure
//!
//! This example explains the structure of a production-ready CLI project.
//!
//! Run with: `cargo run --example 0101_project_structure`

#![allow(dead_code)]

fn main() {
    println!("=== Production CLI Project Structure ===\n");

    // =========================================================================
    // CARGO.TOML CONFIGURATION
    // =========================================================================

    println!("--- Cargo.toml Configuration ---");
    println!(
        r#"
A production CLI needs several key sections in Cargo.toml:

[package]
name = "rust_cli_complete"
version = "0.1.0"
edition = "2024"
description = "A production-ready CLI toolkit"

# Binary definition
[[bin]]
name = "dx"                    # The executable name
path = "src/main.rs"           # Entry point

# Dependencies organized by purpose
[dependencies]
# CLI framework
clap = {{ version = "4", features = ["derive", "env", "wrap_help", "color"] }}
clap_complete = "4"            # Shell completions

# Serialization
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
toml = "0.8"

# Error handling
thiserror = "2"                # Custom error types
anyhow = "1"                   # Application errors

# Terminal UI
colored = "3"                  # Colored output
indicatif = "0.17"             # Progress bars
dialoguer = "0.11"             # Interactive prompts

[dev-dependencies]
assert_cmd = "2"               # CLI testing
predicates = "3"               # Test assertions
insta = "1"                    # Snapshot testing
tempfile = "3"                 # Temporary files
criterion = "0.5"              # Benchmarks
proptest = "1"                 # Property testing

[build-dependencies]
clap = {{ version = "4", features = ["derive"] }}
clap_complete = "4"
clap_mangen = "0.2"            # Man page generation

# Benchmark configuration
[[bench]]
name = "hash_bench"
harness = false

# Release optimizations
[profile.release]
lto = true                     # Link-time optimization
strip = true                   # Strip symbols
codegen-units = 1              # Better optimization
"#
    );

    println!();

    // =========================================================================
    // DIRECTORY STRUCTURE
    // =========================================================================

    println!("--- Directory Structure ---");
    println!(
        r#"
rust_cli_complete/
├── Cargo.toml                 # Project manifest
├── ROADMAP.md                 # Tutorial progress tracking
│
├── src/
│   ├── main.rs                # CLI entry point
│   ├── lib.rs                 # Library exports (for testing)
│   │
│   ├── cli/                   # CLI argument definitions
│   │   ├── mod.rs
│   │   ├── args.rs            # Top-level CLI struct
│   │   └── commands/          # Subcommand arguments
│   │       ├── mod.rs
│   │       ├── hash.rs
│   │       ├── encode.rs
│   │       └── ...
│   │
│   ├── commands/              # Command implementations
│   │   ├── mod.rs
│   │   ├── hash.rs            # Hash logic + tests
│   │   ├── encode.rs
│   │   └── ...
│   │
│   ├── config/                # Configuration management
│   │   ├── mod.rs
│   │   └── settings.rs
│   │
│   └── utils/                 # Shared utilities
│       ├── mod.rs
│       └── output.rs
│
├── tests/                     # Integration tests
│   ├── cli_tests.rs           # End-to-end CLI tests
│   └── snapshots/             # Insta snapshots
│
├── benches/                   # Criterion benchmarks
│   └── hash_bench.rs
│
├── examples/                  # Tutorial examples
│   ├── 0101_project_structure.rs
│   └── ...
│
├── docs/                      # mdbook documentation
│   ├── book.toml
│   └── src/
│       ├── SUMMARY.md
│       └── ...
│
├── .github/
│   └── workflows/
│       ├── ci.yml             # CI testing
│       └── release.yml        # Release automation
│
└── .cargo/
    └── config.toml            # Cross-compilation targets
"#
    );

    println!();

    // =========================================================================
    // LIB VS BIN PATTERN
    // =========================================================================

    println!("--- Library + Binary Pattern ---");
    println!(
        r#"
Separating lib.rs from main.rs enables:

1. Unit testing of core logic
2. Integration testing of CLI
3. Reusable library code
4. Better code organization

// main.rs - thin wrapper
use anyhow::Result;
use clap::Parser;
use rust_cli_complete::cli::{{Cli, Commands}};
use rust_cli_complete::commands;

fn main() -> Result<()> {{
    let cli = Cli::parse();

    match cli.command {{
        Commands::Hash(args) => commands::hash::run(args),
        Commands::Encode(args) => commands::encode::run(args),
        // ...
    }}
}}

// lib.rs - exports modules
pub mod cli;
pub mod commands;
pub mod config;
pub mod utils;

pub use cli::{{Cli, Commands}};
"#
    );

    println!();

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Key principles for production CLI structure:");
    println!("  1. Separate CLI definitions from command logic");
    println!("  2. Use lib.rs for testable code, main.rs as thin wrapper");
    println!("  3. Organize by feature (cli/, commands/, config/, utils/)");
    println!("  4. Include tests/, benches/, examples/, docs/");
    println!("  5. Configure profiles for optimized release builds");
}
