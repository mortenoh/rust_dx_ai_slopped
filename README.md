# dx - Developer Toolkit CLI

A production-ready CLI toolkit demonstrating best practices for Rust CLI development.

## Features

| Command | Alias | Description |
|---------|-------|-------------|
| `hash` | `h` | Compute file/string hashes (SHA-256, SHA-512, MD5) |
| `encode` | `e` | Encode/decode data (Base64, hex, URL) |
| `uuid` | `u` | Generate UUIDs (v4, v7) |
| `time` | `t` | Time utilities and conversions |
| `json` | `j` | JSON formatting, validation, and querying |
| `env` | - | Environment variable utilities |
| `config` | `cfg` | Configuration management |
| `rand` | `r` | Random generation (numbers, strings, passwords) |
| `text` | - | Text transformations (case, slugify) |
| `calc` | `c` | Unit conversions (bytes, time, base, percent) |
| `expr` | `x` | Expression evaluator with functions and variables |
| `net` | - | Network utilities (IP, DNS, ports) |
| `chat` | - | gRPC-based real-time chat |
| `completions` | - | Generate shell completions |

## Installation

```bash
cargo install --path .
```

Or download pre-built binaries from the [Releases](https://github.com/mortenoh/rust_dx_ai_slopped/releases) page.

## Quick Examples

```bash
# Hash a string
dx hash sha256 "hello world"

# Encode to base64
dx encode base64 "hello world"

# Generate UUID
dx uuid v4

# Format JSON
echo '{"a":1}' | dx json fmt -

# Expression evaluation
dx expr eval "2 + 3 * 4"                    # 14
dx expr eval "sqrt(16) + pi"                # 7.14159...
dx expr eval "def square(x) = x*x; square(5)"  # 25
```

## Expression Language

The `expr` command provides a powerful mathematical expression evaluator:

```bash
# Basic math with operator precedence
dx expr eval "2 + 3 * 4"           # 14
dx expr eval "2 ^ 10"              # 1024

# Built-in functions and constants
dx expr eval "sin(pi / 2)"         # 1
dx expr eval "log2(1024)"          # 10
dx expr eval "max(3, 7)"           # 7
dx expr eval "clamp(15, 0, 10)"    # 10

# Variables and multi-statement programs
dx expr eval "x = 5; y = x + 3; y * 2"   # 16

# User-defined functions
dx expr eval "def factorial(n) = if n <= 1 then 1 else n * factorial(n-1); factorial(5)"  # 120

# Lambda expressions and closures
dx expr eval "double = x => x * 2; double(10)"   # 20

# Conditionals
dx expr eval "if 5 > 3 then 100 else 200"        # 100

# Run from file
dx expr run script.dx
```

**Features:**
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `^`, `**`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `and`, `or`, `not` (or `&&`, `||`, `!`)
- Conditionals: `if ... then ... else ...`
- 30+ built-in functions (trig, log, rounding, etc.)
- User-defined functions with `def`
- Lambda expressions: `x => x * 2`
- Closures that capture outer scope
- Comments: `# comment`

See the [Expression Language Guide](https://mortenoh.github.io/rust_dx_ai_slopped/appendices/e-expr-language.html) for the complete reference.

## Documentation

Comprehensive documentation is available as an mdbook:

```bash
# Serve locally
mdbook serve --open

# Build static site
mdbook build
```

**Online:** [https://mortenoh.github.io/rust_dx_ai_slopped/](https://mortenoh.github.io/rust_dx_ai_slopped/)

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
- **Appendices**: Cargo Reference, Common Crates, Expression Language Guide

## Development

```bash
make help    # Show all available commands
make build   # Build debug binary
make test    # Run tests
make bench   # Run benchmarks
make lint    # Run clippy and fmt
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
