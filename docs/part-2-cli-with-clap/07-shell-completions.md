# Shell Completions

Shell completions help users discover commands and options. Clap can generate completion scripts for Bash, Zsh, Fish, and PowerShell.

## Setup

```bash
cargo add clap_complete
```

## Generating Completions

```rust
use clap::{Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::io;

#[derive(Parser)]
#[command(name = "myapp")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate shell completions
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Completions { shell } => {
            generate(shell, &mut Cli::command(), "myapp", &mut io::stdout());
        }
    }
}
```

## Installation

```bash
# Bash
myapp completions bash > ~/.local/share/bash-completion/completions/myapp

# Zsh
myapp completions zsh > ~/.zfunc/_myapp

# Fish
myapp completions fish > ~/.config/fish/completions/myapp.fish
```

## Value Hints

```rust
use clap::{Parser, ValueHint};

#[derive(Parser)]
struct Args {
    #[arg(value_hint = ValueHint::FilePath)]
    input: String,

    #[arg(long, value_hint = ValueHint::DirPath)]
    output: String,

    #[arg(long, value_hint = ValueHint::Url)]
    remote: Option<String>,
}
```

Available hints: `FilePath`, `DirPath`, `Url`, `Hostname`, `Username`, `CommandName`.
