# dx - Developer Toolkit CLI

A production-ready CLI toolkit demonstrating best practices for Rust CLI development.

## Features

| Command | Alias | Description |
|---------|-------|-------------|
| `hash` | `h` | Compute hashes (MD5, SHA-256, SHA-512, Bcrypt, Argon2) |
| `encode` | `e` | Encode/decode data (Base64, hex, URL) |
| `uuid` | `u` | Generate UUIDs (v4, v7) |
| `time` | `t` | Time utilities and conversions |
| `json` | `j` | JSON formatting, validation, and querying |
| `yaml` | `y` | YAML formatting, validation, and conversion |
| `csv` | - | CSV formatting, querying, and conversion |
| `xml` | - | XML formatting, validation, and conversion |
| `jwt` | - | JWT decoding, encoding, and verification |
| `encrypt` | - | Encrypt/decrypt with AES-GCM or ChaCha20-Poly1305 |
| `diff` | - | Text diffing (unified, inline, compact) |
| `template` | - | Jinja2-style template rendering with Tera |
| `markdown` | `md` | Markdown to HTML and TOC extraction |
| `compress` | - | Gzip/Zstd compression and decompression |
| `env` | - | Environment variable utilities |
| `config` | `cfg` | Configuration management |
| `rand` | `r` | Random generation (numbers, strings, passwords) |
| `text` | - | Text transformations (case, slugify) |
| `calc` | `c` | Unit conversions (bytes, time, base, percent) |
| `expr` | `x` | Expression evaluator with functions and variables |
| `net` | - | Network utilities (IP, DNS, ports) |
| `chat` | - | gRPC-based real-time chat |
| `fun` | - | Fun terminal effects (matrix, life, qr, clock, banner, spinners, work, fortune, hacker, progress, countdown, bounce) |
| `grep` | `g` | Regex search in files with context |
| `http` | - | HTTP client (GET, POST, PUT, DELETE, HEAD) |
| `watch` | `w` | Watch files and run commands on changes |
| `system` | `sys` | System information (CPU, memory, OS, uptime) |
| `polars` | `pl` | DataFrame operations (view, random data generation) |
| `dhis2` | - | DHIS2 API utilities (org units, data elements, data values) |
| `ui` | - | Interactive TUI dashboard |
| `egui` | - | Native GUI demos |
| `completions` | - | Generate shell completions |

## Installation

### From Source

```bash
cargo install --path .
```

### Pre-built Binaries

Download from the [Releases](https://github.com/mortenoh/rust_dx_ai_slopped/releases/latest) page:

```bash
# Linux/macOS quick install
curl -L https://github.com/mortenoh/rust_dx_ai_slopped/releases/download/latest/dx-linux-x86_64.tar.gz | tar xz
sudo mv dx /usr/local/bin/
```

Available platforms: Linux (x86_64, ARM64, musl), macOS (x86_64, ARM64), Windows (x86_64).

## Quick Examples

```bash
# Hash a string (SHA-256)
dx hash -s "hello world"

# Password hashing with bcrypt
dx hash -a bcrypt -s "mypassword"

# Password hashing with argon2
dx hash -a argon2 -s "mypassword"

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

# Grep with regex
dx grep "fn main" src/                      # Find all main functions
dx grep -i "error" logs/ -C 2               # Case-insensitive with context

# HTTP requests
dx http get https://api.github.com/zen
dx http post https://httpbin.org/post -d '{"key": "value"}'

# Watch files and run commands
dx watch src/ -- cargo test                 # Re-run tests on changes
dx watch . -e rs -- cargo build             # Rebuild on .rs changes

# System information
dx system info                              # CPU, memory, OS details
dx system uptime                            # System uptime

# DataFrame operations with Polars
dx polars random                            # Generate random data (screen)
dx polars random data.parquet               # Generate Parquet file
dx polars random data.csv -n 1000           # Generate 1000 rows as CSV
dx polars random -c "id:id,name:fruit,city:city,amount:float"
dx polars view data.parquet                 # View data from file
dx polars view data.csv --stats             # Show statistics
dx polars view data.parquet --schema        # Show schema only

# Advanced data generation (60+ generator types)
dx polars random -c "id:id,name:name,email:email,company:company"
dx polars random -c "score:int[0;100],price:float[9.99;99.99]"
dx polars random -c "kunde:name[de],client:name[fr]"  # Locale-specific
dx polars random -c "vin:vin,plate:license_plate,make:vehicle"
dx polars random -c "tx:ulid,btc:btc,eth:eth"         # Crypto/IDs
dx polars random -c "color:hex_color,mime:mime,ver:semver"
dx polars random -n 100 -f json              # Output as JSON

# Fun terminal effects
dx fun matrix                               # Matrix-style falling code
dx fun life                                 # Conway's Game of Life
dx fun qr "https://github.com"              # Generate QR code
dx fun clock                                # Big ASCII clock
dx fun banner "HELLO"                       # ASCII text banner
dx fun spinners                             # Showcase spinner styles

# Data format conversions
dx yaml format config.yaml                  # Pretty-print YAML
dx yaml to-json config.yaml                 # Convert YAML to JSON
dx csv format data.csv                      # Pretty-print CSV as table
dx csv to-json data.csv                     # Convert CSV to JSON
dx xml format doc.xml                       # Pretty-print XML
dx xml to-json doc.xml                      # Convert XML to JSON

# JWT handling
dx jwt decode "eyJhbG..."                   # Decode and display JWT
dx jwt encode --secret "key" --payload '{}' # Create a JWT

# Encryption (ChaCha20-Poly1305 by default)
dx encrypt encrypt -s "secret" --password "pw"   # Encrypt string
dx encrypt decrypt -s "..." --password "pw"      # Decrypt string

# Text diffing
dx diff file1.txt file2.txt                 # Unified diff
dx diff file1.txt file2.txt --format inline # Inline diff

# Template rendering (Jinja2-style)
dx template render template.tera --data data.json

# Markdown utilities
dx markdown render README.md                # Convert to HTML
dx markdown toc README.md                   # Extract TOC

# Compression
dx compress compress file.txt               # Compress to file.txt.gz
dx compress decompress file.txt.gz          # Decompress

# TUI dashboard
dx ui                                       # Interactive system dashboard

# GUI demos
dx egui demo                                # Hello world window
dx egui counter                             # Counter with buttons
dx egui clock                               # Live updating clock
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

# Windows targets (MSVC via cargo-xwin)
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
```

Install Homebrew dependencies:

```bash
# Required for cross-compilation
cargo install cargo-zigbuild  # Linux targets
cargo install cargo-xwin      # Windows MSVC targets
brew install llvm             # Required for cargo-xwin
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
├── dx-x86_64-pc-windows-msvc.exe
└── dx-aarch64-pc-windows-msvc.exe
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
| Windows x86_64 | `target/x86_64-pc-windows-msvc/release/dx.exe` |
| Windows ARM64 | `target/aarch64-pc-windows-msvc/release/dx.exe` |

## License

MIT
