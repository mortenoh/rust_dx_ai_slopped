# The Derive Macro

The clap derive macro transforms structs and enums into argument parsers. This chapter covers all derive attributes in detail.

## Derive Attributes Overview

```rust
use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser)]           // Entry point
#[derive(Subcommand)]       // Subcommand enum
#[derive(Args)]             // Reusable argument groups
#[derive(ValueEnum)]        // Enum for option values
```

## Parser - The Entry Point

### Basic Usage

```rust
use clap::Parser;

#[derive(Parser)]
struct Cli {
    // arguments and options
}

fn main() {
    let cli = Cli::parse();
}
```

### Command Attributes

Apply to the struct with `#[command(...)]`:

```rust
#[derive(Parser)]
#[command(
    name = "myapp",                    // Binary name
    version = "1.0.0",                 // Version string
    author = "Your Name",              // Author info
    about = "Short description",       // Shows with -h
    long_about = "Detailed description", // Shows with --help
)]
struct Cli {}
```

### All Command Attributes

| Attribute | Description |
|-----------|-------------|
| `name` | Binary/command name |
| `version` | Version string |
| `author` | Author information |
| `about` | Short description |
| `long_about` | Long description |
| `after_help` | Text after options |
| `before_help` | Text before options |
| `after_long_help` | Text after long help |
| `before_long_help` | Text before long help |
| `help_template` | Custom help format |
| `override_usage` | Custom usage line |
| `override_help` | Override -h |
| `propagate_version` | Share version with subcommands |
| `disable_version_flag` | Remove --version |
| `disable_help_flag` | Remove --help |
| `disable_colored_help` | Plain help |
| `hide` | Hide from help |
| `subcommand_required` | Must provide subcommand |
| `arg_required_else_help` | Show help if no args |
| `allow_missing_positional` | Allow gaps in positionals |
| `trailing_var_arg` | Last arg captures remaining |
| `dont_delimit_trailing_values` | No -- delimiter |
| `infer_subcommands` | Partial subcommand names |
| `infer_long_args` | Partial arg names |

### Example: Full Command Configuration

```rust
#[derive(Parser)]
#[command(
    name = "dx",
    version,
    author,
    about = "Developer toolkit",
    long_about = "A comprehensive developer toolkit for common tasks.",
    after_help = "Use dx <command> --help for more info",
    propagate_version = true,
    subcommand_required = true,
    arg_required_else_help = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```

## Arg Attributes

Apply to fields with `#[arg(...)]`:

### Naming

```rust
#[derive(Parser)]
struct Args {
    #[arg(short)]                    // -v
    verbose: bool,

    #[arg(long)]                     // --output
    output: String,

    #[arg(short, long)]              // -o, --output
    out: String,

    #[arg(short = 'V', long = "verb")] // -V, --verb
    verbosity: bool,
}
```

### Value Configuration

```rust
#[derive(Parser)]
struct Args {
    #[arg(value_name = "FILE")]      // Name in help
    input: String,

    #[arg(default_value = "output.txt")]
    output: String,

    #[arg(default_value_t = 10)]     // For non-String types
    count: u32,

    #[arg(default_value_ifs = [
        ("format", "json", Some("data.json")),
        ("format", "xml", Some("data.xml")),
    ])]
    file: Option<String>,
}
```

### Requirements

```rust
#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    input: String,

    #[arg(required_if_eq("format", "json"))]
    schema: Option<String>,

    #[arg(required_unless_present = "stdin")]
    file: Option<String>,

    #[arg(requires = "output")]      // If set, output is required
    compress: bool,

    #[arg(requires_all = ["format", "output"])]
    convert: bool,

    #[arg(requires_if("compress", "true", "level"))]
    level: Option<u32>,
}
```

### Conflicts and Groups

```rust
#[derive(Parser)]
struct Args {
    #[arg(short, long, conflicts_with = "quiet")]
    verbose: bool,

    #[arg(short, long)]
    quiet: bool,

    #[arg(short, long, conflicts_with_all = ["json", "xml"])]
    text: bool,

    #[arg(group = "output_format")]
    json: bool,

    #[arg(group = "output_format")]
    xml: bool,
}
```

### Actions

```rust
use clap::ArgAction;

#[derive(Parser)]
struct Args {
    // Set to true when present
    #[arg(long, action = ArgAction::SetTrue)]
    verbose: bool,

    // Set to false when present
    #[arg(long, action = ArgAction::SetFalse)]
    no_cache: bool,

    // Count occurrences (-v -v -v = 3)
    #[arg(short, action = ArgAction::Count)]
    verbosity: u8,

    // Collect all values
    #[arg(short, action = ArgAction::Append)]
    include: Vec<String>,

    // Store the value
    #[arg(long, action = ArgAction::Set)]
    format: Option<String>,
}
```

### All Arg Attributes

| Attribute | Description |
|-----------|-------------|
| `short` | Enable short flag |
| `long` | Enable long flag |
| `value_name` | Name in help |
| `default_value` | String default |
| `default_value_t` | Typed default |
| `default_missing_value` | Value when flag but no value |
| `required` | Must be provided |
| `requires` | Other arg required if set |
| `conflicts_with` | Cannot use together |
| `overrides_with` | Later overrides earlier |
| `global` | Available to subcommands |
| `action` | How to handle the arg |
| `value_parser` | Custom value parsing |
| `num_args` | Number of values |
| `value_delimiter` | Split values |
| `env` | Read from environment |
| `hide` | Hide from help |
| `hide_default_value` | Don't show default |
| `hide_possible_values` | Don't show values |
| `help` | Short help text |
| `long_help` | Long help text |
| `visible_alias` | Shown alternative name |
| `alias` | Hidden alternative name |

## Args - Reusable Groups

Group related arguments for reuse:

```rust
use clap::{Args, Parser};

#[derive(Args)]
struct OutputOptions {
    /// Output file
    #[arg(short, long)]
    output: Option<String>,

    /// Output format
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Pretty print
    #[arg(short, long)]
    pretty: bool,
}

#[derive(Args)]
struct VerbosityOptions {
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Quiet mode
    #[arg(short, long, conflicts_with = "verbose")]
    quiet: bool,
}

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    output: OutputOptions,

    #[command(flatten)]
    verbosity: VerbosityOptions,

    /// Input file
    input: String,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbosity.verbose {
        println!("Format: {}", cli.output.format);
    }
}
```

## ValueEnum - Constrained Values

For options with a fixed set of values:

```rust
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Json,
    Yaml,
    Toml,
    #[value(alias = "txt")]
    Text,
}

#[derive(ValueEnum, Clone, Debug)]
#[value(rename_all = "SCREAMING_SNAKE_CASE")]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Format::Json)]
    format: Format,

    #[arg(long, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
}

fn main() {
    let args = Args::parse();
    println!("Format: {:?}", args.format);
}
```

Usage:
```bash
program --format json --log-level DEBUG
program --format txt   # Uses alias
```

### ValueEnum Attributes

| Attribute | Description |
|-----------|-------------|
| `rename_all` | Case transformation |
| `alias` | Alternative value name |
| `aliases` | Multiple alternatives |
| `skip` | Skip this variant |
| `hide` | Hide from help |
| `help` | Custom help text |

## Subcommand

Define subcommands with an enum:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new item
    Add {
        /// Item name
        name: String,
    },

    /// Remove an item
    Remove {
        /// Item ID
        id: u32,

        /// Force removal
        #[arg(short, long)]
        force: bool,
    },

    /// List all items
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name } => println!("Adding {}", name),
        Commands::Remove { id, force } => println!("Removing {} (force={})", id, force),
        Commands::List => println!("Listing..."),
    }
}
```

### Nested Subcommands

```rust
#[derive(Subcommand)]
enum Commands {
    /// Configuration commands
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Get a config value
    Get { key: String },

    /// Set a config value
    Set { key: String, value: String },

    /// List all config
    List,
}
```

### Optional Subcommands

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
```

## Flattening

Embed other structs:

```rust
#[derive(Args)]
struct GlobalOptions {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[arg(short, long, global = true)]
    config: Option<String>,
}

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    global: GlobalOptions,

    #[command(subcommand)]
    command: Commands,
}
```

## External Subcommands

Allow unknown subcommands:

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Known command
    Build,

    /// Pass through to external command
    #[command(external_subcommand)]
    External(Vec<String>),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => println!("Building..."),
        Commands::External(args) => {
            // args[0] is the subcommand name
            println!("External: {:?}", args);
        }
    }
}
```

## Deriving Debug

Always derive Debug for easy printing:

```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}
```

## Summary

| Derive | Purpose |
|--------|---------|
| `Parser` | Main entry point struct |
| `Subcommand` | Subcommand enum |
| `Args` | Reusable argument group |
| `ValueEnum` | Constrained value set |

| Container Attribute | Description |
|---------------------|-------------|
| `#[command(...)]` | Configure command |
| `#[command(subcommand)]` | Mark as subcommand field |
| `#[command(flatten)]` | Embed Args struct |

| Field Attribute | Description |
|-----------------|-------------|
| `#[arg(...)]` | Configure argument |
