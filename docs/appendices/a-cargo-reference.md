# Cargo Reference

Quick reference for common Cargo commands.

## Project Commands

```bash
# Create new project
cargo new myproject        # Binary
cargo new mylib --lib      # Library
cargo init                 # In existing directory

# Build
cargo build                # Debug build
cargo build --release      # Release build
cargo build --target x86_64-unknown-linux-musl

# Run
cargo run                  # Run binary
cargo run -- --help        # Pass arguments
cargo run --release        # Run release build
cargo run --example demo   # Run example

# Test
cargo test                 # All tests
cargo test test_name       # Specific test
cargo test --lib           # Library tests only
cargo test --doc           # Doc tests only
cargo test -- --nocapture  # Show println output

# Documentation
cargo doc                  # Build docs
cargo doc --open           # Build and open
cargo doc --no-deps        # Skip dependencies

# Check without building
cargo check                # Type check
cargo clippy               # Lint
cargo fmt                  # Format code
cargo fmt -- --check       # Check formatting

# Publish
cargo publish              # Publish to crates.io
cargo publish --dry-run    # Test publish
```

## Dependency Management

```bash
# Add dependencies
cargo add serde            # Latest version
cargo add serde@1.0        # Specific version
cargo add serde --features derive
cargo add tokio --features full
cargo add --dev criterion  # Dev dependency
cargo add --build cc       # Build dependency

# Remove dependencies
cargo remove serde

# Update dependencies
cargo update               # All dependencies
cargo update -p serde      # Specific package

# Show dependency tree
cargo tree
cargo tree -d              # Show duplicates
cargo tree -i serde        # Invert (who depends on serde)
```

## Cargo.toml Reference

```toml
[package]
name = "dx"
version = "1.0.0"
edition = "2021"
rust-version = "1.70"
description = "Developer CLI tools"
license = "MIT"
repository = "https://github.com/user/dx"
keywords = ["cli", "tools"]
categories = ["command-line-utilities"]

[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }

[dev-dependencies]
criterion = "0.5"

[build-dependencies]
cc = "1"

[features]
default = ["json"]
json = ["dep:serde_json"]
full = ["json", "yaml"]

[[bin]]
name = "dx"
path = "src/main.rs"

[[example]]
name = "demo"
path = "examples/demo.rs"

[[bench]]
name = "benchmarks"
harness = false

[profile.release]
opt-level = 3
lto = true
strip = true
```

## Workspaces

```toml
# Root Cargo.toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
version = "1.0.0"
edition = "2021"

[workspace.dependencies]
serde = "1"
```

```toml
# crates/core/Cargo.toml
[package]
name = "dx-core"
version.workspace = true
edition.workspace = true

[dependencies]
serde.workspace = true
```

## Environment Variables

```bash
CARGO_HOME           # Cargo home (~/.cargo)
CARGO_TARGET_DIR     # Build output directory
CARGO_BUILD_JOBS     # Parallel jobs
RUST_BACKTRACE=1     # Show backtrace on panic
RUSTFLAGS            # Compiler flags
```

## Useful Cargo Plugins

```bash
cargo install cargo-edit       # add/remove/upgrade
cargo install cargo-watch      # Watch for changes
cargo install cargo-outdated   # Check for updates
cargo install cargo-audit      # Security audit
cargo install cargo-bloat      # Binary size analysis
cargo install cargo-release    # Release automation
```
