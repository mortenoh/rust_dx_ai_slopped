# Advanced Patterns

Advanced clap patterns for complex CLI applications.

## Global Flags

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[arg(short, long, global = true)]
    config: Option<String>,

    #[command(subcommand)]
    command: Commands,
}
```

## Mutual Exclusion

```rust
#[derive(Parser)]
struct Args {
    #[arg(long, conflicts_with = "quiet")]
    verbose: bool,

    #[arg(long)]
    quiet: bool,

    #[arg(long, group = "output")]
    json: bool,

    #[arg(long, group = "output")]
    yaml: bool,
}
```

## Argument Groups

```rust
use clap::{Parser, ArgGroup};

#[derive(Parser)]
#[command(group(ArgGroup::new("input").required(true).args(["file", "stdin"])))]
struct Args {
    #[arg(long)]
    file: Option<String>,

    #[arg(long)]
    stdin: bool,
}
```

## Conditional Requirements

```rust
#[derive(Parser)]
struct Args {
    #[arg(long)]
    format: Option<String>,

    #[arg(long, required_if_eq("format", "json"))]
    schema: Option<String>,

    #[arg(long, requires = "output")]
    compress: bool,

    #[arg(long)]
    output: Option<String>,
}
```

## Default Subcommand

```rust
#[derive(Subcommand)]
enum Commands {
    #[command(default = true)]
    Run { file: String },

    Build { target: String },
}
```

## External Subcommands

```rust
#[derive(Subcommand)]
enum Commands {
    Build,

    #[command(external_subcommand)]
    External(Vec<String>),
}
```

## Combining with Config Files

```rust
use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Deserialize)]
struct Args {
    #[arg(long)]
    #[serde(default)]
    verbose: bool,

    #[arg(long)]
    port: Option<u16>,
}

fn main() {
    let file_config: Args = load_config().unwrap_or_default();
    let cli = Args::parse();

    // CLI overrides file config
    let port = cli.port.or(file_config.port).unwrap_or(8080);
}
```
