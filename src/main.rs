//! # dx - Developer Experience CLI
//!
//! A production-ready developer toolkit demonstrating CLI best practices in Rust.
//!
//! ## Architecture Overview
//!
//! This CLI follows the "thin main" pattern:
//! - `main.rs` handles CLI parsing and command dispatch (minimal logic)
//! - Business logic lives in library modules (`src/commands/`)
//! - Configuration is centralized (`src/config/`)
//!
//! ## Application Flow
//!
//! ```text
//! main()
//!   │
//!   ├─► Cli::parse()           Parse arguments using clap
//!   │
//!   ├─► Handle global flags    --no-color, --verbose, etc.
//!   │
//!   └─► match command          Dispatch to appropriate handler
//!         ├── hash  ──► commands::hash::run()
//!         ├── encode ─► commands::encode::run()
//!         ├── uuid ──► commands::uuid::run()
//!         ├── time ──► commands::time::run()
//!         ├── json ──► commands::json::run()
//!         ├── env ───► commands::env::run()
//!         └── config ► commands::config::run()
//! ```
//!
//! ## Error Handling
//!
//! We use `anyhow::Result` as the return type from `main()`. This enables:
//! - Automatic error formatting to stderr
//! - Non-zero exit code on error (standard Unix convention)
//! - Backtrace support when `RUST_BACKTRACE=1`
//!
//! ## Why Result from main()?
//!
//! Rust 1.26+ allows `main()` to return `Result<(), E>` where `E: Termination`.
//! This is cleaner than manually calling `process::exit()` or `unwrap()`.
//!
//! When `main()` returns:
//! - `Ok(())` → exit code 0 (success)
//! - `Err(e)` → prints error to stderr, exit code 1 (failure)
//!
//! ## External Documentation
//! - Clap derive: <https://docs.rs/clap/latest/clap/_derive/index.html>
//! - Anyhow: <https://docs.rs/anyhow>
//! - Colored: <https://docs.rs/colored>

use anyhow::Result;
use clap::Parser;
use rust_cli_complete::cli::{Cli, Commands};
use rust_cli_complete::commands;

/// Application entry point.
///
/// # Return Value
///
/// Returns `Result<()>` which allows clean error handling:
/// - Success: returns `Ok(())`, process exits with code 0
/// - Failure: returns `Err(e)`, error is printed, exits with code 1
///
/// # Panics
///
/// This function doesn't panic under normal operation. All errors are
/// propagated via the `?` operator and handled by the Result return type.
fn main() -> Result<()> {
    // Parse CLI arguments using clap's derive macros.
    // This happens before any I/O, so argument errors are reported immediately.
    // See Cli struct in src/cli/args.rs for argument definitions.
    let cli = Cli::parse();

    // Handle global flags that affect all commands.
    // --no-color: Disable ANSI color codes in output.
    // This is useful for:
    // - Piping output to files or other commands
    // - Terminals that don't support colors
    // - CI environments where colors may be garbled
    if cli.no_color {
        // colored::control provides global control over color output.
        // set_override(false) forces all .red(), .green(), etc. to be no-ops.
        // This is a global setting - affects all threads.
        colored::control::set_override(false);
    }

    // Dispatch to the appropriate command handler based on the subcommand.
    //
    // Each command module exposes a `run(args)` function that:
    // 1. Takes the parsed arguments struct
    // 2. Performs the command's logic
    // 3. Returns Result<()> for error propagation
    //
    // The `?` operator at the end (implicit via the match expression's use
    // as the function's return value) propagates any errors to main's caller.
    match cli.command {
        // Hash command: generate cryptographic hashes of files/strings
        Commands::Hash(args) => commands::hash::run(args),

        // Encode command: base64, hex encoding/decoding
        Commands::Encode(args) => commands::encode::run(args),

        // UUID command: generate UUIDs (v4 random, v7 timestamp)
        Commands::Uuid(args) => commands::uuid::run(args),

        // Time command: parse, format, convert timestamps
        Commands::Time(args) => commands::time::run(args),

        // JSON command: format, validate, query JSON data
        Commands::Json(args) => commands::json::run(args),

        // Env command: inspect and export environment variables
        Commands::Env(args) => commands::env::run(args),

        // Config command: manage application configuration
        Commands::Config(args) => commands::config::run(args),

        // Rand command: generate random data
        Commands::Rand(args) => commands::rand::run(args),

        // Text command: text transformations
        Commands::Text(args) => commands::text::run(args),

        // Calc command: unit conversions
        Commands::Calc(args) => commands::calc::run(args),

        // Expr command: expression evaluator
        Commands::Expr(args) => commands::expr::run(args),

        // Net command: network utilities
        Commands::Net(args) => commands::net::run(args),

        // Chat command: gRPC-based real-time chat (async)
        // Uses tokio runtime since chat is the only async command
        Commands::Chat(args) => tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime")
            .block_on(commands::chat::run(args)),

        // Fun command: fun terminal effects
        Commands::Fun(args) => commands::fun::run(args),

        // Grep command: search for patterns in files
        Commands::Grep(args) => commands::grep::run(args),

        // Http command: make HTTP requests
        Commands::Http(args) => commands::http::run(args),

        // Watch command: watch files and run commands
        Commands::Watch(args) => commands::watch::run(args),

        // System command: system information and utilities
        Commands::System(args) => commands::system::run(args),

        // UI command: interactive TUI dashboard (requires --features ui)
        #[cfg(feature = "ui")]
        Commands::Ui(args) => commands::ui::run(args),

        // Completions command: generate shell completions
        Commands::Completions { shell } => {
            Cli::print_completions(shell);
            Ok(())
        }
    }
}
