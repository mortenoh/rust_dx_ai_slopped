# Compile Time Optimization

Speed up your build times.

## Measure Compile Time

```bash
# Time the build
cargo build --timings

# Clean build time
cargo clean && time cargo build

# Incremental build time
touch src/main.rs && time cargo build
```

## Development Profile

```toml
# Fast compile for dev
[profile.dev]
opt-level = 0
debug = true

# Optimize dependencies only
[profile.dev.package."*"]
opt-level = 2
```

## Reduce Dependencies

```bash
# Count dependencies
cargo tree | wc -l

# Find duplicates
cargo tree --duplicates

# See dependency tree
cargo tree --depth 2
```

### Use Minimal Features

```toml
# Heavy: includes everything
tokio = "1"

# Light: only what you need
tokio = { version = "1", default-features = false, features = ["rt", "fs"] }
```

## Workspace Settings

```toml
# Cargo.toml (workspace)
[workspace]
members = ["crates/*"]

[profile.dev]
opt-level = 0

[profile.dev.package."*"]
opt-level = 2
```

## Linker Optimization

### mold (Linux)

```bash
# Install mold
sudo apt install mold  # or build from source
```

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

### lld (Cross-platform)

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

## sccache

```bash
cargo install sccache
```

```toml
# .cargo/config.toml
[build]
rustc-wrapper = "sccache"
```

```bash
# Check cache stats
sccache --show-stats
```

## Cranelift Backend

```bash
# Install
rustup component add rustc-codegen-cranelift-preview
```

```toml
# .cargo/config.toml
[unstable]
codegen-backend = true

[profile.dev]
codegen-backend = "cranelift"
```

## Parallel Frontend

```toml
# Cargo.toml - nightly only
[unstable]
parallel-compiler = true
```

## Split Large Crates

```
my-cli/
├── Cargo.toml
└── crates/
    ├── cli/           # Main binary
    ├── core/          # Core logic
    ├── hash/          # Hash module
    └── encode/        # Encode module
```

Only changed crates recompile.

## Reduce Generics

```rust
// Compiles separately for each type
fn process<T: Display>(item: T) { ... }

// Single compilation
fn process(item: &dyn Display) { ... }
```

## Build Cache

```yaml
# GitHub Actions
- uses: Swatinem/rust-cache@v2
```

## Summary

| Technique | Improvement |
|-----------|-------------|
| mold linker | 2-5x faster |
| sccache | Caches rebuilds |
| Cranelift | 2x faster dev |
| Minimal features | Variable |
| Workspaces | Better incremental |
