# Binary Size Optimization

Reduce your binary size for faster downloads and less disk usage.

## Measure Current Size

```bash
# Size of release binary
ls -lh target/release/dx

# Detailed breakdown
cargo bloat --release

# What's contributing to size
cargo bloat --release --crates
```

## Size Optimization Profile

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
panic = "abort"      # Remove panic unwinding
strip = true         # Remove symbols
```

## Strip Symbols

```toml
[profile.release]
strip = true
```

Or manually:
```bash
strip target/release/dx
```

## Abort on Panic

```toml
[profile.release]
panic = "abort"
```

Saves ~10% by removing unwinding code.

## UPX Compression

```bash
# Install UPX
brew install upx  # macOS
apt install upx   # Linux

# Compress binary
upx --best target/release/dx

# Check compression ratio
upx -l target/release/dx
```

## Reduce Dependencies

### Audit Dependencies

```bash
cargo tree --duplicates
cargo bloat --release --crates
```

### Use Minimal Features

```toml
# Instead of
clap = "4"

# Use only needed features
clap = { version = "4", default-features = false, features = ["derive", "std"] }
```

### Replace Heavy Dependencies

| Heavy | Lighter Alternative |
|-------|---------------------|
| `reqwest` | `ureq` |
| `chrono` | `time` |
| `regex` | `regex-lite` |
| `serde_json` | `miniserde` |

## Cargo Features

```toml
[features]
default = ["full"]
full = ["json", "yaml", "toml"]
minimal = []

# Only include what's needed
json = ["dep:serde_json"]
yaml = ["dep:serde_yaml"]
```

```bash
# Build with minimal features
cargo build --release --no-default-features
```

## Static vs Dynamic Linking

```bash
# Static (larger but portable)
cargo build --release --target x86_64-unknown-linux-musl

# Dynamic (smaller but needs libraries)
cargo build --release --target x86_64-unknown-linux-gnu
```

## Analyze Binary Contents

```bash
# Install cargo-binutils
cargo install cargo-binutils
rustup component add llvm-tools-preview

# See section sizes
cargo size --release

# List symbols by size
cargo nm --release | sort -k2 -n -r | head -20
```

## Example Results

```
Optimization Steps:
1. Default release:     12.5 MB
2. opt-level = "z":     10.2 MB  (-18%)
3. LTO enabled:          8.1 MB  (-35%)
4. panic = "abort":      7.4 MB  (-41%)
5. strip = true:         2.8 MB  (-78%)
6. UPX compression:      1.1 MB  (-91%)
```

## Size Budget

Track size in CI:

```yaml
- name: Check binary size
  run: |
    SIZE=$(stat -f%z target/release/dx)
    if [ $SIZE -gt 5000000 ]; then
      echo "Binary too large: $SIZE bytes"
      exit 1
    fi
```
