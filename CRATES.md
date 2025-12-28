# Workspace Crates Guide

This document explains how to create and manage workspace crates in the dx CLI project.

## Overview

We use a Cargo workspace to organize reusable code into separate crates. This provides:

- **Modularity**: Each crate has a single responsibility
- **Reusability**: Crates can be published to crates.io independently
- **Faster compilation**: Only changed crates are recompiled
- **Cleaner dependencies**: Each crate declares only what it needs

## Current Workspace Structure

```
dx/
├── Cargo.toml              # Workspace root + main binary
├── src/                    # Main CLI source
│   ├── main.rs             # Entry point
│   ├── lib.rs              # Library exports
│   ├── commands/           # Command implementations
│   ├── cli/                # CLI argument definitions
│   └── expr/               # Re-exports dx-expr
├── crates/
│   ├── progress/           # dx-progress - Terminal progress bars
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/lib.rs
│   └── expr/               # dx-expr - Expression evaluator
│       ├── Cargo.toml
│       ├── README.md
│       └── src/
│           ├── lib.rs
│           ├── ast.rs
│           └── parser.rs
└── tests/                  # Integration tests
```

## Creating a New Crate

### Step 1: Create the Directory Structure

```bash
mkdir -p crates/mylib/src
```

### Step 2: Create Cargo.toml

Create `crates/mylib/Cargo.toml`:

```toml
[package]
name = "dx-mylib"
version = "0.1.0"
edition.workspace = true          # Inherit from workspace
description = "Brief description of what this crate does"
authors.workspace = true          # Inherit from workspace
license.workspace = true          # Inherit from workspace
repository.workspace = true       # Inherit from workspace
readme = "README.md"
keywords = ["relevant", "keywords"]
categories = ["relevant-category"]

[dependencies]
# Add your dependencies here
# Use workspace = true for shared deps:
# serde = { workspace = true }

[dev-dependencies]
# Test-only dependencies
```

### Step 3: Add to Workspace

Edit the root `Cargo.toml`:

```toml
[workspace]
members = [".", "crates/progress", "crates/expr", "crates/mylib"]
```

### Step 4: Create the Library

Create `crates/mylib/src/lib.rs`:

```rust
//! # dx-mylib
//!
//! Brief description of what this crate does.
//!
//! ## Example
//!
//! ```
//! use dx_mylib::do_something;
//!
//! let result = do_something("input");
//! assert_eq!(result, "expected");
//! ```

/// Main function documentation
pub fn do_something(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_something() {
        assert_eq!(do_something("hello"), "hello");
    }
}
```

### Step 5: Create README

Create `crates/mylib/README.md`:

```markdown
# dx-mylib

Brief description.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dx-mylib = "0.1"
```

## Usage

```rust
use dx_mylib::do_something;

let result = do_something("input");
```

## License

MIT
```

### Step 6: Use in Main Crate

Add to root `Cargo.toml` dependencies:

```toml
dx-mylib = { path = "crates/mylib" }
```

Optionally re-export from main crate for backwards compatibility:

```rust
// src/mylib/mod.rs
pub use dx_mylib::*;
```

## Best Practices

### Naming

- Use `dx-` prefix for all crates (e.g., `dx-progress`, `dx-expr`)
- Use lowercase with hyphens for crate names
- Use underscores in Rust code (e.g., `use dx_progress::...`)

### Dependencies

Use workspace-level dependencies for shared crates:

```toml
# Root Cargo.toml
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
anyhow = "1"

# Crate Cargo.toml
[dependencies]
serde.workspace = true
anyhow.workspace = true
```

### Documentation

- Every public item should have documentation
- Include examples in doc comments
- Use `cargo doc --open` to preview docs
- Doctests serve as both examples and tests

### Testing

```bash
# Test a specific crate
cargo test -p dx-mylib

# Test all workspace crates
cargo test --workspace

# Test with doc tests
cargo test --doc
```

### Publishing

Before publishing to crates.io:

1. Ensure `README.md` exists
2. Verify all metadata in `Cargo.toml`
3. Run `cargo publish --dry-run -p dx-mylib`
4. Create a git tag: `git tag dx-mylib-v0.1.0`

## Workspace Configuration

The root `Cargo.toml` defines shared settings:

```toml
[workspace]
members = [".", "crates/progress", "crates/expr"]

[workspace.package]
edition = "2021"
authors = ["Your Name <email@example.com>"]
license = "MIT"
repository = "https://github.com/user/repo"

# Shared dependencies (optional)
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
```

## Current Crates

### dx-progress

Terminal progress reporting with OSC 9;4 support for modern terminals.

**Features:**
- Progress bars with percentage
- Bouncing/indeterminate progress
- Spinner animations
- Terminal-native progress via OSC 9;4

**Dependencies:** None (zero-dependency crate)

### dx-expr

Expression evaluator with variables, functions, lambdas, and closures.

**Features:**
- Arithmetic, comparison, logical operators
- User-defined functions and lambdas
- Closures with captured environment
- AST serialization via serde

**Dependencies:** anyhow, serde, serde_json

## Candidates for Extraction

Modules that could be extracted into crates:

| Module | Lines | Candidate Crate | Notes |
|--------|-------|-----------------|-------|
| `config/` | ~510 | `dx-config` | Configuration management |
| `commands/hash.rs` | ~390 | `dx-hash` | Hashing utilities |
| `commands/encrypt.rs` | ~186 | `dx-crypto` | Encryption utilities |
| `commands/json.rs` | ~390 | `dx-formats` | Data format conversions |

When extracting:
1. Identify public API surface
2. Minimize dependencies
3. Maintain backwards compatibility via re-exports
4. Add comprehensive tests and docs

## Commands

```bash
# Build all crates
cargo build --workspace

# Format all crates
cargo fmt --all

# Lint all crates
cargo clippy --workspace

# Test all crates
cargo test --workspace

# Doc all crates
cargo doc --workspace --no-deps

# Clean build artifacts
cargo clean
```
