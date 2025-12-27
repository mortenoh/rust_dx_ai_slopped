# Clap Basics

This chapter covers the fundamentals of building CLIs with clap: parsing arguments, handling options, and structuring your code.

## Your First CLI

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "greet")]
#[command(about = "A simple greeting program")]
struct Args {
    /// The name to greet
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.name);
}
```

## Parsing Methods

### parse() - Standard Parsing

```rust
fn main() {
    let args = Args::parse();  // Exits on error
}
```

### try_parse() - Returns Result

```rust
fn main() {
    match Args::try_parse() {
        Ok(args) => println!("Got: {:?}", args),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
```

### parse_from() - Custom Arguments

```rust
fn main() {
    // Parse from custom iterator
    let args = Args::parse_from(["myapp", "Alice"]);

    // Useful for testing
    let args = Args::parse_from(vec!["test", "--verbose", "file.txt"]);
}
```

## Positional Arguments

Arguments without flags:

```rust
#[derive(Parser)]
struct Args {
    /// Input file
    input: String,

    /// Output file
    output: String,
}
// Usage: program input.txt output.txt
```

### Optional Positional Arguments

```rust
#[derive(Parser)]
struct Args {
    /// Input file
    input: String,

    /// Output file (optional)
    output: Option<String>,
}
// Usage: program input.txt [output.txt]
```

### Multiple Positional Arguments

```rust
#[derive(Parser)]
struct Args {
    /// Files to process
    files: Vec<String>,
}
// Usage: program file1.txt file2.txt file3.txt
```

### Required Minimum

```rust
#[derive(Parser)]
struct Args {
    /// Files to process (at least one required)
    #[arg(required = true)]
    files: Vec<String>,
}
```

## Options (Flags with Values)

### Short and Long Options

```rust
#[derive(Parser)]
struct Args {
    /// Output format
    #[arg(short, long)]
    format: String,
}
// Usage: program -f json
// Usage: program --format json
```

### Custom Short/Long Names

```rust
#[derive(Parser)]
struct Args {
    /// Verbosity level
    #[arg(short = 'v', long = "verbose")]
    verbosity: bool,

    /// Output file
    #[arg(short = 'o', long = "out")]
    output: String,
}
```

### Optional Options

```rust
#[derive(Parser)]
struct Args {
    /// Configuration file
    #[arg(short, long)]
    config: Option<String>,
}
// Usage: program
// Usage: program --config settings.toml
```

### Default Values

```rust
#[derive(Parser)]
struct Args {
    /// Output format
    #[arg(short, long, default_value = "json")]
    format: String,

    /// Retry count
    #[arg(short, long, default_value_t = 3)]
    retries: u32,
}
```

## Flags (Boolean Options)

### Simple Flag

```rust
#[derive(Parser)]
struct Args {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}
// Usage: program --verbose
```

### Counting Flags

```rust
use clap::ArgAction;

#[derive(Parser)]
struct Args {
    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
}
// Usage: program -vvv  (verbose = 3)
```

### Negatable Flags

```rust
#[derive(Parser)]
struct Args {
    /// Enable colors (--color / --no-color)
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    color: bool,

    #[arg(long = "no-color", action = ArgAction::SetFalse, hide = true)]
    no_color: (),
}
```

## Value Types

Clap automatically parses values into Rust types:

```rust
use std::path::PathBuf;
use std::net::IpAddr;

#[derive(Parser)]
struct Args {
    /// Port number
    #[arg(short, long)]
    port: u16,

    /// Server address
    #[arg(short, long)]
    address: IpAddr,

    /// Config file path
    #[arg(short, long)]
    config: PathBuf,

    /// Timeout in seconds
    #[arg(short, long)]
    timeout: f64,
}
```

### Supported Types

Any type implementing `FromStr`:

- Integers: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`
- Floats: `f32`, `f64`
- Strings: `String`, `OsString`
- Paths: `PathBuf`
- Network: `IpAddr`, `SocketAddr`
- And more...

## Value Names

Customize how values appear in help:

```rust
#[derive(Parser)]
struct Args {
    /// Output format
    #[arg(short, long, value_name = "FORMAT")]
    format: String,

    /// Number of threads
    #[arg(short, long, value_name = "NUM")]
    threads: Option<u32>,
}
```

Help output:
```
Options:
  -f, --format <FORMAT>  Output format
  -t, --threads <NUM>    Number of threads
```

## Aliases

### Option Aliases

```rust
#[derive(Parser)]
struct Args {
    /// Configuration file
    #[arg(short, long, visible_alias = "cfg", alias = "conf")]
    config: Option<String>,
}
// Usage: --config, --cfg, --conf all work
// --cfg appears in help, --conf is hidden
```

### Short Aliases

```rust
#[derive(Parser)]
struct Args {
    #[arg(short = 'o', short_alias = 'O', long)]
    output: String,
}
// Usage: -o file.txt, -O file.txt
```

## Help and Documentation

### Doc Comments

```rust
#[derive(Parser)]
#[command(about = "Short description")]
#[command(long_about = "Longer description\n\nWith multiple paragraphs")]
struct Args {
    /// This appears in help
    #[arg(short, long)]
    verbose: bool,

    /// Short help text
    ///
    /// This longer text appears with --help but not -h
    #[arg(short, long)]
    config: Option<String>,
}
```

### Custom Help

```rust
#[derive(Parser)]
#[command(
    about = "Process data files",
    after_help = "Examples:\n  program input.txt\n  program -v input.txt output.txt"
)]
struct Args {
    #[arg(short, long, help = "Enable verbose mode")]
    verbose: bool,

    #[arg(long, long_help = "Detailed explanation of this option\nthat spans multiple lines")]
    complex: bool,
}
```

## Version Information

### From Cargo.toml

```rust
#[derive(Parser)]
#[command(version)]
struct Args {}
```

### Custom Version

```rust
#[derive(Parser)]
#[command(version = "1.2.3")]
struct Args {}
```

### Detailed Version

```rust
const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\ncommit: ",
    env!("GIT_HASH", "unknown"),
    "\nbuilt: ",
    env!("BUILD_DATE", "unknown"),
);

#[derive(Parser)]
#[command(version = VERSION)]
struct Args {}
```

## Error Handling

```rust
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    count: u32,
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    // Your application logic
    process(args.count)?;

    Ok(())
}

fn process(count: u32) -> Result<()> {
    for i in 0..count {
        println!("Processing {}", i);
    }
    Ok(())
}
```

## Complete Example

```rust
use clap::Parser;
use std::path::PathBuf;

/// A simple file processor
#[derive(Parser, Debug)]
#[command(name = "fileproc")]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file to process
    input: PathBuf,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Processing format
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Quiet mode (no output)
    #[arg(short, long, conflicts_with = "verbose")]
    quiet: bool,

    /// Number of processing threads
    #[arg(short = 't', long, default_value_t = 4, value_name = "NUM")]
    threads: u32,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("Input: {:?}", args.input);
        println!("Output: {:?}", args.output);
        println!("Format: {}", args.format);
        println!("Threads: {}", args.threads);
    }

    // Process the file...
}
```

## Summary

| Concept | Syntax |
|---------|--------|
| Positional arg | `name: String` |
| Optional positional | `name: Option<String>` |
| Multiple positional | `names: Vec<String>` |
| Short option | `#[arg(short)]` |
| Long option | `#[arg(long)]` |
| Default value | `#[arg(default_value = "x")]` |
| Flag | `verbose: bool` |
| Count | `#[arg(action = ArgAction::Count)]` |
| Help text | `/// Doc comment` |
| Version | `#[command(version)]` |
