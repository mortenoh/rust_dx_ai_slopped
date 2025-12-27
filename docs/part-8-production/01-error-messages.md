# Error Messages

Provide helpful, actionable error messages.

## The Problem

```
// Unhelpful
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }

// Helpful
Error: Could not read configuration file 'config.toml'
Cause: No such file or directory
Hint: Run 'dx init' to create a default configuration
```

## Using thiserror

```toml
[dependencies]
thiserror = "1"
```

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DxError {
    #[error("Could not read file '{path}'")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Unknown hash algorithm: {0}")]
    UnknownAlgorithm(String),
}
```

## Using anyhow

```toml
[dependencies]
anyhow = "1"
```

```rust
use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Could not read config file '{}'", path))?;

    toml::from_str(&content)
        .with_context(|| format!("Invalid TOML in '{}'", path))
}
```

## Error Display

```rust
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);

        // Show cause chain
        let mut source = e.source();
        while let Some(cause) = source {
            eprintln!("Caused by: {}", cause);
            source = cause.source();
        }

        std::process::exit(1);
    }
}
```

## Actionable Hints

```rust
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found at '{path}'")]
    NotFound { path: String },
}

impl ConfigError {
    pub fn hint(&self) -> Option<&str> {
        match self {
            Self::NotFound { .. } =>
                Some("Run 'dx init' to create a default configuration"),
        }
    }
}

// In main
if let Some(hint) = error.hint() {
    eprintln!("Hint: {}", hint);
}
```

## Colored Output

```toml
[dependencies]
colored = "2"
```

```rust
use colored::*;

fn print_error(e: &anyhow::Error) {
    eprintln!("{} {}", "Error:".red().bold(), e);

    for cause in e.chain().skip(1) {
        eprintln!("  {} {}", "Caused by:".yellow(), cause);
    }
}
```

## Exit Codes

```rust
pub enum ExitCode {
    Success = 0,
    GeneralError = 1,
    InvalidArgs = 2,
    IoError = 3,
    ConfigError = 4,
}

fn main() {
    let code = match run() {
        Ok(_) => ExitCode::Success,
        Err(DxError::InvalidConfig(_)) => ExitCode::ConfigError,
        Err(DxError::ReadFile { .. }) => ExitCode::IoError,
        Err(_) => ExitCode::GeneralError,
    };

    std::process::exit(code as i32);
}
```

## Best Practices

1. **Be specific**: "File 'x.txt' not found" not "File not found"
2. **Show context**: What was the program trying to do?
3. **Give solutions**: How can the user fix it?
4. **Use stderr**: Errors go to stderr, output to stdout
5. **Exit codes**: Use meaningful codes for scripting
