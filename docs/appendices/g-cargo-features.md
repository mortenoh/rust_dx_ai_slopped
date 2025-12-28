# Appendix G: Cargo Features

Cargo features allow optional functionality and conditional compilation.

## Defining Features

In `Cargo.toml`:

```toml
[features]
default = []                        # Features enabled by default
ui = ["dep:ratatui", "dep:crossterm"]  # Optional UI feature

[dependencies]
ratatui = { version = "0.29", optional = true }
crossterm = { version = "0.28", optional = true }
```

## Feature Syntax

| Syntax | Meaning |
|--------|---------|
| `default = ["foo"]` | Enable `foo` by default |
| `foo = []` | Simple feature flag |
| `foo = ["dep:bar"]` | Enable optional dependency |
| `foo = ["bar", "baz"]` | Enable multiple features |
| `foo = ["other-crate/feature"]` | Enable dependency feature |

## Using Features

### Build Commands

```bash
# Build with default features
cargo build

# Build with specific feature
cargo build --features ui

# Build with multiple features
cargo build --features "ui,networking"

# Build without default features
cargo build --no-default-features

# Build with only specific features
cargo build --no-default-features --features ui
```

### Installation

```bash
# Install with feature
cargo install --path . --features ui
```

## Conditional Compilation

Use `#[cfg(feature = "...")]` for conditional code:

```rust
// Conditional module
#[cfg(feature = "ui")]
pub mod ui;

// Conditional use
#[cfg(feature = "ui")]
use crate::ui::UiArgs;

// Conditional function
#[cfg(feature = "ui")]
pub fn run_ui() {
    // UI-specific code
}

// Conditional in enum
pub enum Commands {
    Hash(HashArgs),
    #[cfg(feature = "ui")]
    Ui(UiArgs),
}

// Conditional in match
match command {
    Commands::Hash(args) => handle_hash(args),
    #[cfg(feature = "ui")]
    Commands::Ui(args) => handle_ui(args),
}
```

## The dx `ui` Feature

The `dx` CLI uses a feature for the TUI dashboard:

```toml
[features]
default = []
ui = ["dep:ratatui", "dep:crossterm"]
```

This keeps the default binary small and avoids TUI dependencies for users who don't need them.

### Why Use Features?

1. **Smaller binaries** - Don't include unused code
2. **Faster compilation** - Fewer dependencies to build
3. **Cross-compilation** - Some deps don't cross-compile well
4. **Optional functionality** - Let users opt-in

### dx Feature Pattern

```
src/cli/commands/ui.rs      # Only compiled with --features ui
src/commands/ui.rs          # Only compiled with --features ui
```

Both files are gated:

```rust
// In mod.rs
#[cfg(feature = "ui")]
pub mod ui;

// In args.rs
#[cfg(feature = "ui")]
Ui(UiArgs),
```

## Best Practices

### 1. Keep Default Features Minimal

```toml
[features]
default = []  # Nothing by default
full = ["ui", "networking", "extras"]
```

### 2. Document Features

```toml
[package.metadata.docs.rs]
all-features = true  # Build docs with all features

[features]
## Enable TUI dashboard (adds ratatui, crossterm)
ui = ["dep:ratatui", "dep:crossterm"]
```

### 3. Test All Feature Combinations

```bash
# In CI
cargo test
cargo test --features ui
cargo test --all-features
cargo test --no-default-features
```

### 4. Use `dep:` Prefix for Optional Dependencies

```toml
# Modern syntax (Rust 1.60+)
ui = ["dep:ratatui"]

# Old syntax (creates implicit feature)
# ratatui = { version = "0.29", optional = true }
# ui = ["ratatui"]  # Works but creates "ratatui" feature too
```

## Feature Detection at Runtime

Features are compile-time only. For runtime detection:

```rust
// This is compile-time, not runtime
#[cfg(feature = "ui")]
const HAS_UI: bool = true;

#[cfg(not(feature = "ui"))]
const HAS_UI: bool = false;

fn main() {
    if HAS_UI {
        println!("Built with UI support");
    }
}
```

## Platform + Feature Combinations

Combine platform and feature checks:

```rust
#[cfg(all(feature = "ui", unix))]
fn unix_ui_specific() {
    // Only on Unix with UI feature
}

#[cfg(all(feature = "ui", windows))]
fn windows_ui_specific() {
    // Only on Windows with UI feature
}
```
