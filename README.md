# dx - Developer Toolkit CLI

A production-ready CLI toolkit demonstrating best practices for Rust CLI development.

## Features

- **hash** - Compute file and string hashes (SHA-256, SHA-512, MD5)
- **encode** - Encode/decode data (Base64, hex, URL)
- **uuid** - Generate UUIDs (v4, v7)
- **time** - Time utilities and conversions
- **json** - JSON formatting and validation
- **env** - Environment variable utilities
- **config** - Configuration management

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Hash a file
dx hash file.txt

# Encode to base64
dx encode --base64 "hello world"

# Generate UUID
dx uuid

# Format JSON
dx json format data.json
```

## Documentation

Comprehensive documentation is available as an mdbook:

```bash
# Serve locally
mdbook serve --open

# Build static site
mdbook build
```

### Documentation Contents

- **Part 1**: Rust Fundamentals
- **Part 2**: CLI Development with Clap
- **Part 3**: Command Reference
- **Part 4**: Testing Strategies
- **Part 5**: Benchmarking
- **Part 6**: Documentation
- **Part 7**: Cross-Platform Development
- **Part 8**: Production Readiness
- **Part 9**: Optimization

## Development

```bash
make help    # Show all available commands
make build   # Build debug binary
make test    # Run tests
make bench   # Run benchmarks
make lint    # Run clippy
make fmt     # Format code
```

## Cross-Compilation

### Prerequisites

Install Rust targets:

```bash
# Linux targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-gnu

# macOS targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Windows targets (gnullvm = LLVM linker, no mingw-w64 needed)
rustup target add x86_64-pc-windows-gnullvm
rustup target add aarch64-pc-windows-gnullvm
```

Install Homebrew dependencies:

```bash
# Required for cross-compilation (Linux and Windows)
cargo install cargo-zigbuild
```

### Building for All Platforms

```bash
# Build for all platforms (debug)
make build-all

# Build for all platforms (release)
make release-all

# Build for individual platforms
make build-linux     # Linux (x86_64, x86_64-musl, aarch64)
make build-macos     # macOS (x86_64, aarch64)
make build-windows   # Windows (x86_64, aarch64)

# Release builds
make release-linux
make release-macos
make release-windows
```

### Distribution

Build all platforms and collect binaries into a single `dist/` directory:

```bash
make dist
```

This creates:

```
dist/
├── dx-x86_64-unknown-linux-gnu
├── dx-x86_64-unknown-linux-musl
├── dx-aarch64-unknown-linux-gnu
├── dx-x86_64-apple-darwin
├── dx-aarch64-apple-darwin
├── dx-x86_64-pc-windows-gnullvm.exe
└── dx-aarch64-pc-windows-gnullvm.exe
```

### Compressed Distribution (UPX)

For smaller binaries (~60% size reduction), use UPX compression:

```bash
# Install UPX (optional)
brew install upx

# Build and compress (Linux/Windows only)
make dist-compressed
```

**Note:** UPX compression is only applied to Linux and Windows binaries. macOS binaries
cannot be compressed with UPX due to code signing requirements (Gatekeeper will kill them).

| Platform | Original | Compressed | Ratio |
|----------|----------|------------|-------|
| Linux x86_64 | ~1.9 MB | ~764 KB | ~40% |
| Linux musl | ~1.8 MB | ~775 KB | ~41% |
| Linux ARM64 | ~1.6 MB | ~705 KB | ~44% |
| Windows x86_64 | ~1.8 MB | ~690 KB | ~39% |
| Windows ARM64 | ~1.4 MB | - | UPX not yet supported |
| macOS (any) | - | - | Breaks code signing |

### Output Locations

Individual binaries are placed in `target/<triple>/release/`:

| Platform | Binary |
|----------|--------|
| Linux x86_64 | `target/x86_64-unknown-linux-gnu/release/dx` |
| Linux x86_64 (static) | `target/x86_64-unknown-linux-musl/release/dx` |
| Linux ARM64 | `target/aarch64-unknown-linux-gnu/release/dx` |
| macOS x86_64 | `target/x86_64-apple-darwin/release/dx` |
| macOS ARM64 | `target/aarch64-apple-darwin/release/dx` |
| Windows x86_64 | `target/x86_64-pc-windows-gnullvm/release/dx.exe` |
| Windows ARM64 | `target/aarch64-pc-windows-gnullvm/release/dx.exe` |

## License

MIT
