# Release Builds

Configure optimal release builds.

## Basic Release Build

```bash
cargo build --release
```

Binary at: `target/release/dx`

## Release Profile

```toml
# Cargo.toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = true             # Link-time optimization
codegen-units = 1      # Better optimization
panic = "abort"        # Smaller binary
strip = true           # Remove symbols
```

## Optimization Levels

| Level | Description | Use Case |
|-------|-------------|----------|
| 0 | No optimization | Debug |
| 1 | Basic optimization | Faster debug |
| 2 | Most optimization | Balanced |
| 3 | All optimization | Max speed |
| "s" | Optimize for size | Small binary |
| "z" | Min size | Smallest binary |

```toml
[profile.release]
opt-level = 3   # Speed
# or
opt-level = "z" # Size
```

## Link-Time Optimization (LTO)

```toml
[profile.release]
lto = true          # Full LTO
# or
lto = "thin"        # Faster compile, less optimization
# or
lto = "fat"         # Same as true
```

## Codegen Units

```toml
[profile.release]
codegen-units = 1   # Better optimization, slower compile
# default is 16
```

## Panic Handling

```toml
[profile.release]
panic = "abort"     # Smaller binary, no unwinding
# default is "unwind"
```

## Strip Symbols

```toml
[profile.release]
strip = true        # Remove all symbols
# or
strip = "symbols"   # Same as true
strip = "debuginfo" # Remove debug info only
strip = "none"      # Keep everything
```

## Debug Info in Release

```toml
[profile.release]
debug = true        # Include debug symbols
# Useful for profiling release builds
```

## Custom Profiles

```toml
# Fast compile, some optimization
[profile.dev.package."*"]
opt-level = 2

# Profiling build
[profile.profiling]
inherits = "release"
debug = true
strip = false

# Minimal size
[profile.min-size]
inherits = "release"
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

```bash
cargo build --profile profiling
cargo build --profile min-size
```

## Target-Specific Settings

```toml
[profile.release]
opt-level = 3

# More optimization for crypto
[profile.release.package.sha2]
opt-level = 3

# Less for rarely-used code
[profile.release.package.toml]
opt-level = 2
```

## Environment Variables

```bash
# Parallel codegen
CARGO_BUILD_JOBS=8 cargo build --release

# Custom target directory
CARGO_TARGET_DIR=/tmp/target cargo build --release
```

## Recommended Configuration

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```
