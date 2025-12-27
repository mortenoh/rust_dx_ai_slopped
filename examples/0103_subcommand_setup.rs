//! # Subcommand Setup
//!
//! This example demonstrates how to structure subcommands with clap.
//!
//! Run with: `cargo run --example 0103_subcommand_setup`

#![allow(dead_code)]

use clap::{Args, Parser, Subcommand};

// =========================================================================
// TOP-LEVEL CLI
// =========================================================================

/// Example CLI with subcommands
#[derive(Parser, Debug)]
#[command(
    name = "mycli",
    about = "Example CLI with subcommands",
    arg_required_else_help = true,
    propagate_version = true
)]
struct Cli {
    /// Global flag available to all subcommands
    #[arg(long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

// =========================================================================
// SUBCOMMAND ENUM
// =========================================================================

/// Available commands
#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new item
    #[command(visible_alias = "a")]
    Add(AddArgs),

    /// Remove an item
    #[command(visible_alias = "rm")]
    Remove(RemoveArgs),

    /// List all items
    #[command(visible_alias = "ls")]
    List(ListArgs),

    /// Configuration management
    #[command(subcommand)]
    Config(ConfigCommands),
}

// =========================================================================
// SUBCOMMAND ARGUMENTS
// =========================================================================

/// Arguments for the 'add' command
#[derive(Args, Debug)]
struct AddArgs {
    /// Name of the item to add
    name: String,

    /// Optional description
    #[arg(short, long)]
    description: Option<String>,

    /// Tags for the item
    #[arg(short, long)]
    tags: Vec<String>,

    /// Priority level (1-5)
    #[arg(short, long, default_value = "3", value_parser = clap::value_parser!(u8).range(1..=5))]
    priority: u8,
}

/// Arguments for the 'remove' command
#[derive(Args, Debug)]
struct RemoveArgs {
    /// Name or ID of the item to remove
    target: String,

    /// Force removal without confirmation
    #[arg(short, long)]
    force: bool,

    /// Remove all matching items
    #[arg(long)]
    all: bool,
}

/// Arguments for the 'list' command
#[derive(Args, Debug)]
struct ListArgs {
    /// Filter by status
    #[arg(short, long)]
    status: Option<String>,

    /// Output format
    #[arg(short, long, default_value = "table")]
    format: OutputFormat,

    /// Maximum items to show
    #[arg(short, long)]
    limit: Option<usize>,
}

#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
enum OutputFormat {
    #[default]
    Table,
    Json,
    Csv,
}

// =========================================================================
// NESTED SUBCOMMANDS
// =========================================================================

/// Config subcommands (nested)
#[derive(Subcommand, Debug)]
enum ConfigCommands {
    /// Get a config value
    Get {
        /// Config key
        key: String,
    },

    /// Set a config value
    Set {
        /// Config key
        key: String,
        /// Config value
        value: String,
    },

    /// List all config values
    List,

    /// Reset to defaults
    Reset {
        /// Don't ask for confirmation
        #[arg(short, long)]
        force: bool,
    },
}

// =========================================================================
// COMMAND DISPATCH
// =========================================================================

fn run_cli(cli: Cli) {
    // Access global flags
    if cli.verbose {
        println!("[verbose mode enabled]");
    }

    // Dispatch to command handlers
    match cli.command {
        Commands::Add(args) => handle_add(args),
        Commands::Remove(args) => handle_remove(args),
        Commands::List(args) => handle_list(args),
        Commands::Config(cmd) => handle_config(cmd),
    }
}

fn handle_add(args: AddArgs) {
    println!("Adding item:");
    println!("  name: {}", args.name);
    println!("  description: {:?}", args.description);
    println!("  tags: {:?}", args.tags);
    println!("  priority: {}", args.priority);
}

fn handle_remove(args: RemoveArgs) {
    println!("Removing:");
    println!("  target: {}", args.target);
    println!("  force: {}", args.force);
    println!("  all: {}", args.all);
}

fn handle_list(args: ListArgs) {
    println!("Listing items:");
    println!("  status: {:?}", args.status);
    println!("  format: {:?}", args.format);
    println!("  limit: {:?}", args.limit);
}

fn handle_config(cmd: ConfigCommands) {
    match cmd {
        ConfigCommands::Get { key } => {
            println!("Getting config: {}", key);
        }
        ConfigCommands::Set { key, value } => {
            println!("Setting config: {} = {}", key, value);
        }
        ConfigCommands::List => {
            println!("Listing all config values");
        }
        ConfigCommands::Reset { force } => {
            println!("Resetting config (force: {})", force);
        }
    }
}

fn main() {
    println!("=== Subcommand Setup ===\n");

    // =========================================================================
    // SUBCOMMAND PATTERN
    // =========================================================================

    println!("--- Subcommand Pattern ---");
    println!(
        r#"
The pattern for subcommands:

1. Top-level CLI struct with #[command(subcommand)]
2. Subcommand enum with #[derive(Subcommand)]
3. Args struct for each command with #[derive(Args)]
4. Match on enum to dispatch

#[derive(Parser)]
struct Cli {{
    #[arg(long, global = true)]  // Available to all subcommands
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}}

#[derive(Subcommand)]
enum Commands {{
    Add(AddArgs),
    Remove(RemoveArgs),
    List(ListArgs),
}}

#[derive(Args)]
struct AddArgs {{
    name: String,
    #[arg(short, long)]
    force: bool,
}}
"#
    );

    println!();

    // =========================================================================
    // COMMAND ATTRIBUTES
    // =========================================================================

    println!("--- Command Attributes ---");
    println!(
        r#"
Useful attributes for subcommands:

#[derive(Subcommand)]
enum Commands {{
    // Alias shown in help
    #[command(visible_alias = "a")]
    Add(AddArgs),

    // Hidden alias (works but not shown)
    #[command(alias = "delete")]
    Remove(RemoveArgs),

    // Custom about text
    #[command(about = "List all items")]
    List(ListArgs),

    // Nested subcommands
    #[command(subcommand)]
    Config(ConfigCommands),

    // Hide from help
    #[command(hide = true)]
    Debug(DebugArgs),
}}
"#
    );

    println!();

    // =========================================================================
    // NESTED SUBCOMMANDS
    // =========================================================================

    println!("--- Nested Subcommands ---");
    println!(
        r#"
For commands like `mycli config set key value`:

#[derive(Subcommand)]
enum Commands {{
    #[command(subcommand)]
    Config(ConfigCommands),
}}

#[derive(Subcommand)]
enum ConfigCommands {{
    Get {{ key: String }},
    Set {{ key: String, value: String }},
    List,
}}

Inline syntax (no separate Args struct):
  Get {{ key: String }}

Struct syntax (for more options):
  Get(GetArgs)
"#
    );

    println!();

    // =========================================================================
    // DISPATCH PATTERN
    // =========================================================================

    println!("--- Command Dispatch ---");
    println!(
        r#"
The main.rs dispatch pattern:

fn main() -> Result<()> {{
    let cli = Cli::parse();

    match cli.command {{
        Commands::Add(args) => commands::add::run(args),
        Commands::Remove(args) => commands::remove::run(args),
        Commands::List(args) => commands::list::run(args),
        Commands::Config(cmd) => match cmd {{
            ConfigCommands::Get {{ key }} => ...,
            ConfigCommands::Set {{ key, value }} => ...,
            ConfigCommands::List => ...,
        }},
    }}
}}

Each command module has a run() function:

// commands/add.rs
pub fn run(args: AddArgs) -> Result<()> {{
    // Implementation
    Ok(())
}}
"#
    );

    println!();

    // =========================================================================
    // EXAMPLE PARSING
    // =========================================================================

    println!("--- Example: Parsing Subcommands ---");

    // Simulate different commands
    let test_cases = [
        vec!["mycli", "add", "item1", "-d", "A test item", "-t", "rust"],
        vec!["mycli", "--verbose", "remove", "item1", "--force"],
        vec!["mycli", "list", "--format", "json", "--limit", "10"],
        vec!["mycli", "config", "set", "theme", "dark"],
    ];

    for args in test_cases {
        println!("\n  Parsing: {:?}", args.join(" "));
        match Cli::try_parse_from(&args) {
            Ok(cli) => run_cli(cli),
            Err(e) => println!("  Error: {}", e),
        }
    }

    println!();

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Subcommand setup:");
    println!("  1. Use #[command(subcommand)] field in main CLI");
    println!("  2. Define Commands enum with #[derive(Subcommand)]");
    println!("  3. Create Args struct for each command");
    println!("  4. Use visible_alias for short command names");
    println!("  5. Nest subcommands with #[command(subcommand)]");
    println!("  6. Use match to dispatch to handlers");
}
