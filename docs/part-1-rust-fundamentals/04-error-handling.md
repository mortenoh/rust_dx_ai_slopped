# Error Handling

Rust distinguishes between recoverable and unrecoverable errors. This chapter covers both approaches and best practices for CLI applications.

## Unrecoverable Errors with panic!

For bugs and unrecoverable situations:

```rust
fn main() {
    panic!("crash and burn");
}
```

Output:

```
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```

### When to Panic

- Programming bugs (accessing invalid index)
- Invariant violations
- Prototype/example code
- Tests

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];  // Panics: index out of bounds
}
```

### Backtraces

Set `RUST_BACKTRACE=1` for full stack traces:

```bash
RUST_BACKTRACE=1 cargo run
```

## Recoverable Errors with Result

The `Result` enum represents success or failure:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Basic Usage

```rust
use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };
}
```

### Handling Different Errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = match File::open("hello.txt") {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file: {:?}", e),
            },
            other_error => panic!("Failed to open file: {:?}", other_error),
        },
    };
}
```

### Cleaner with Closures

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Failed to create file: {:?}", error);
            })
        } else {
            panic!("Failed to open file: {:?}", error);
        }
    });
}
```

## Shortcuts: unwrap and expect

### unwrap

Returns value or panics:

```rust
use std::fs::File;

fn main() {
    let file = File::open("hello.txt").unwrap();  // Panics if Err
}
```

### expect

Like unwrap, but with custom message:

```rust
use std::fs::File;

fn main() {
    let file = File::open("hello.txt")
        .expect("Failed to open hello.txt");  // Better error message
}
```

## Propagating Errors

Return errors to the caller:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = match File::open("username.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

## The ? Operator

Propagates errors concisely:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;  // Returns Err if failed
    let mut username = String::new();
    file.read_to_string(&mut username)?;         // Returns Err if failed
    Ok(username)
}
```

The `?` operator:
1. If `Ok`, extracts the value
2. If `Err`, returns the error from the function
3. Applies `From::from()` for error conversion

### Chaining with ?

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

### Even Shorter

```rust
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}
```

## ? with Option

The `?` operator also works with `Option`:

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let text = "Hello\nWorld";
    match last_char_of_first_line(text) {
        Some(c) => println!("Last char: {}", c),
        None => println!("No characters"),
    }
}
```

## Custom Error Types

### Simple Approach

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ParseConfigError {
    message: String,
}

impl fmt::Display for ParseConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config error: {}", self.message)
    }
}

impl Error for ParseConfigError {}

fn parse_config(input: &str) -> Result<Config, ParseConfigError> {
    if input.is_empty() {
        return Err(ParseConfigError {
            message: "Empty input".to_string(),
        });
    }
    // ... parse config
    Ok(Config { /* ... */ })
}
```

### Using thiserror

The `thiserror` crate simplifies custom errors:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid value for {field}: {value}")]
    InvalidValue { field: String, value: String },
}
```

Usage:

```rust
fn load_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // Converts io::Error
    let config: Config = toml::from_str(&content)?; // Converts toml::de::Error

    if config.name.is_empty() {
        return Err(ConfigError::MissingField("name".to_string()));
    }

    Ok(config)
}
```

## Using anyhow for Applications

For applications (not libraries), `anyhow` provides convenient error handling:

```rust
use anyhow::{Context, Result, bail, ensure};

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read config file")?;

    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;

    ensure!(!config.name.is_empty(), "Config name cannot be empty");

    if config.port == 0 {
        bail!("Invalid port: 0");
    }

    Ok(config)
}

fn main() -> Result<()> {
    let config = load_config("config.toml")?;
    println!("Loaded: {:?}", config);
    Ok(())
}
```

### anyhow Features

```rust
use anyhow::{anyhow, Context, Result};

// Create ad-hoc errors
fn example() -> Result<()> {
    Err(anyhow!("Something went wrong"))
}

// Add context to errors
fn load_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))
}

// Error chains
fn process() -> Result<()> {
    let content = load_file("data.txt")
        .context("Could not load data")?;
    // ...
    Ok(())
}
```

## Combining thiserror and anyhow

Use `thiserror` for library errors, `anyhow` in applications:

```rust
// In your library (lib.rs)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashError {
    #[error("Unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),

    #[error("Failed to read input: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn hash_file(path: &str, algorithm: &str) -> Result<String, HashError> {
    // ...
}

// In your application (main.rs)
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let hash = hash_file("file.txt", "sha256")
        .context("Failed to hash file")?;

    println!("Hash: {}", hash);
    Ok(())
}
```

## Error Handling Patterns

### Early Return Pattern

```rust
fn process(input: &str) -> Result<Output, Error> {
    if input.is_empty() {
        return Err(Error::EmptyInput);
    }

    let parsed = parse(input)?;
    let validated = validate(parsed)?;
    let result = transform(validated)?;

    Ok(result)
}
```

### Collecting Results

```rust
fn process_all(items: Vec<&str>) -> Result<Vec<Output>, Error> {
    items.iter()
        .map(|item| process(item))
        .collect()  // Stops at first error
}

fn process_all_results(items: Vec<&str>) -> Vec<Result<Output, Error>> {
    items.iter()
        .map(|item| process(item))
        .collect()  // Collects all results
}
```

### Partition Success and Errors

```rust
fn process_with_errors(items: Vec<&str>) -> (Vec<Output>, Vec<Error>) {
    let results: Vec<Result<Output, Error>> = items.iter()
        .map(|item| process(item))
        .collect();

    let (oks, errs): (Vec<_>, Vec<_>) = results.into_iter()
        .partition(Result::is_ok);

    let successes: Vec<Output> = oks.into_iter()
        .map(Result::unwrap)
        .collect();

    let failures: Vec<Error> = errs.into_iter()
        .map(Result::unwrap_err)
        .collect();

    (successes, failures)
}
```

## main() with Result

Return `Result` from main for clean error handling:

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    run(config)?;
    Ok(())
}
```

Or with anyhow:

```rust
use anyhow::Result;

fn main() -> Result<()> {
    let config = load_config()?;
    run(config)?;
    Ok(())
}
```

## CLI Error Handling Best Practices

### User-Friendly Errors

```rust
use anyhow::{Context, Result};
use std::process::ExitCode;

fn run() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .context("Usage: program <file>")?;

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Could not read file: {}", path))?;

    // Process content...
    Ok(())
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);  // Pretty-printed error chain
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
```

### Exit Codes

```rust
use std::process::ExitCode;

const EXIT_SUCCESS: u8 = 0;
const EXIT_GENERAL_ERROR: u8 = 1;
const EXIT_USAGE_ERROR: u8 = 2;
const EXIT_IO_ERROR: u8 = 3;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::from(EXIT_SUCCESS),
        Err(e) => {
            eprintln!("Error: {}", e);
            match e.downcast_ref::<ConfigError>() {
                Some(ConfigError::MissingField(_)) => ExitCode::from(EXIT_USAGE_ERROR),
                Some(ConfigError::IoError(_)) => ExitCode::from(EXIT_IO_ERROR),
                _ => ExitCode::from(EXIT_GENERAL_ERROR),
            }
        }
    }
}
```

## Summary

| Concept | When to Use |
|---------|-------------|
| `panic!` | Unrecoverable errors, bugs |
| `Result<T, E>` | Recoverable errors |
| `Option<T>` | Optional values (not errors) |
| `?` operator | Propagate errors concisely |
| `unwrap/expect` | Quick prototypes, tests |
| `thiserror` | Library error types |
| `anyhow` | Application error handling |

Best practices:
- Return `Result` from functions that can fail
- Use `?` for error propagation
- Add context to errors with `anyhow::Context`
- Provide user-friendly error messages
- Use appropriate exit codes
- Reserve `panic!` for bugs, not user errors
