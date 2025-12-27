# Introduction to Clap

Clap (Command Line Argument Parser) is the most popular Rust library for building command-line interfaces. It provides a declarative way to define arguments, options, subcommands, and help messages.

## Why Clap?

### Features

- **Declarative**: Define CLI structure with derive macros or builder pattern
- **Validation**: Type checking, value constraints, required arguments
- **Help generation**: Automatic help messages and version info
- **Shell completions**: Generate completions for bash, zsh, fish, etc.
- **Error handling**: User-friendly error messages
- **Extensible**: Custom validators, value parsers, and completions

### Alternatives

| Crate | Style | Best For |
|-------|-------|----------|
| **clap** | Derive or builder | Full-featured CLIs |
| **structopt** | Derive (legacy) | Merged into clap 3+ |
| **argh** | Derive | Google-style, minimal |
| **pico-args** | Minimal | Tiny, zero-dependency |
| **lexopt** | Low-level | Custom parsing |

## Installation

Add clap to your project:

```bash
cargo add clap --features derive
```

Or in `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

### Feature Flags

| Feature | Description |
|---------|-------------|
| `derive` | Enable derive macros |
| `cargo` | Read version from Cargo.toml |
| `env` | Read from environment variables |
| `unicode` | Unicode support |
| `wrap_help` | Wrap help text at terminal width |
| `string` | Use String instead of &str |

## Quick Example

```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
```

Run it:

```bash
$ cargo run -- --help
Simple program to greet a person

Usage: greet [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of the person to greet

Options:
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version

$ cargo run -- Alice --count 3
Hello Alice!
Hello Alice!
Hello Alice!
```

## Two Approaches

### Derive Macros (Recommended)

Declarative, compile-time checked:

```rust
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();
}
```

### Builder API

Programmatic, more flexible:

```rust
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("my_app")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");
}
```

Most of this guide uses derive macros for clarity and type safety.

## Anatomy of a CLI

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "mycli",
    version = "1.0.0",
    author = "Your Name",
    about = "Does awesome things",
    long_about = "A longer description of what this CLI does.\n\nWith multiple paragraphs."
)]
struct Cli {
    /// Global verbose flag
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process files
    Process {
        /// Input files
        files: Vec<String>,

        /// Output directory
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Show configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current config
    Show,
    /// Set a config value
    Set {
        key: String,
        value: String,
    },
}
```

## What You'll Learn

This part covers:

1. **Clap Basics** - Setting up projects, parsing arguments
2. **Derive Macro** - All derive attributes and options
3. **Arguments and Options** - Positional args, flags, options
4. **Subcommands** - Nested command structures
5. **Value Validation** - Custom parsers and constraints
6. **Environment Variables** - Reading from environment
7. **Shell Completions** - Generating completion scripts
8. **Help Customization** - Formatting and styling help
9. **Advanced Patterns** - Global flags, mutual exclusion
10. **Real-World Examples** - Complete CLI applications

## Project Setup

Throughout this section, we'll build a real CLI tool. Create a new project:

```bash
cargo new dx
cd dx
cargo add clap --features derive,env,wrap_help
cargo add anyhow thiserror
```

Initial structure:

```
dx/
├── Cargo.toml
└── src/
    ├── main.rs
    └── cli/
        ├── mod.rs
        └── commands/
            └── mod.rs
```

Let's build something useful!
