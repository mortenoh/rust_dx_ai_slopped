# Logging with tracing

Structured logging for debugging and monitoring.

## Setup

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## Basic Usage

```rust
use tracing::{info, debug, warn, error, trace};
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting application");
    debug!(version = env!("CARGO_PKG_VERSION"), "Version info");
}
```

## Log Levels

```rust
trace!("Very detailed tracing");      // TRACE
debug!("Debugging information");       // DEBUG
info!("General information");          // INFO
warn!("Warning conditions");           // WARN
error!("Error conditions");            // ERROR
```

## Structured Fields

```rust
info!(
    file = %path.display(),
    size = file_size,
    "Processing file"
);

// Output: Processing file file=/path/to/file size=1024
```

## Spans

```rust
use tracing::{span, Level};

fn process_file(path: &Path) {
    let span = span!(Level::INFO, "process_file", ?path);
    let _enter = span.enter();

    debug!("Reading file");
    // ... operations are associated with this span
}
```

## With #[instrument]

```rust
use tracing::instrument;

#[instrument]
fn hash_file(path: &Path) -> Result<String> {
    // Automatically creates span with function name and arguments
    let content = std::fs::read(path)?;
    Ok(compute_hash(&content))
}

#[instrument(skip(data), fields(size = data.len()))]
fn process_data(data: &[u8]) -> Result<()> {
    // Skip sensitive data, add custom fields
    Ok(())
}
```

## Environment Configuration

```bash
# Set log level
RUST_LOG=debug dx hash file.txt

# Per-module levels
RUST_LOG=dx=debug,hyper=warn dx serve

# With targets
RUST_LOG=dx::hash=trace dx hash file.txt
```

## Subscriber Configuration

```rust
fn init_logging(verbose: u8) {
    let filter = match verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(filter))
        )
        .with_target(false)      // Hide target module
        .with_thread_ids(false)  // Hide thread IDs
        .without_time()          // Hide timestamp (for CLI)
        .init();
}
```

## JSON Output

```rust
tracing_subscriber::fmt()
    .json()
    .init();

// {"timestamp":"...","level":"INFO","message":"Processing","file":"x.txt"}
```

## File Logging

```rust
use tracing_appender::rolling;

let file_appender = rolling::daily("/var/log/dx", "dx.log");
let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

tracing_subscriber::fmt()
    .with_writer(file_writer)
    .init();
```

## CLI Integration

```rust
#[derive(Parser)]
struct Cli {
    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Silence all output
    #[arg(short, long)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();

    if !cli.quiet {
        init_logging(cli.verbose);
    }
}
```
