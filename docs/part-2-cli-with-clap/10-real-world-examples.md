# Real-World Examples

Complete examples of production-ready CLI applications.

## File Processor CLI

```rust
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fproc", version, about = "File processing toolkit")]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert file formats
    Convert {
        #[arg(required = true)]
        input: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,

        #[arg(short, long, value_enum)]
        format: Format,
    },

    /// Analyze file contents
    Analyze {
        files: Vec<PathBuf>,

        #[arg(long)]
        json: bool,
    },
}

#[derive(Clone, ValueEnum)]
enum Format { Json, Yaml, Toml }

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { input, output, format } => {
            let out = output.unwrap_or_else(|| input.with_extension("out"));
            println!("Converting {:?} to {:?}", input, out);
        }
        Commands::Analyze { files, json } => {
            for file in files {
                println!("Analyzing {:?}", file);
            }
        }
    }
    Ok(())
}
```

## API Client CLI

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "api")]
struct Cli {
    #[arg(long, env = "API_URL", default_value = "https://api.example.com")]
    url: String,

    #[arg(long, env = "API_KEY", hide_env_values = true)]
    api_key: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List resources
    List {
        #[arg(long, default_value_t = 10)]
        limit: u32,
    },

    /// Get a resource
    Get { id: String },

    /// Create a resource
    Create {
        #[arg(long)]
        name: String,

        #[arg(long)]
        data: Option<String>,
    },
}
```

## Multi-Tool CLI (like dx)

```rust
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "dx", version, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hash(HashArgs),
    Encode(EncodeArgs),
    Uuid(UuidArgs),
    Time(TimeArgs),
}

#[derive(Args)]
struct HashArgs {
    file: String,
    #[arg(short, long, default_value = "sha256")]
    algorithm: String,
}

#[derive(Args)]
struct EncodeArgs {
    input: String,
    #[arg(short, long)]
    decode: bool,
}

#[derive(Args)]
struct UuidArgs {
    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

#[derive(Args)]
struct TimeArgs {
    #[arg(default_value = "now")]
    timestamp: String,
}
```

This pattern keeps each command's arguments organized and testable independently.
