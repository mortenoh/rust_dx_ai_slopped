# Dependencies

Complete documentation of all project dependencies, including descriptions and links.

---

## Direct Dependencies

### CLI Framework

#### clap
**Version:** 4.x | [crates.io](https://crates.io/crates/clap) | [docs.rs](https://docs.rs/clap) | [GitHub](https://github.com/clap-rs/clap)

Command Line Argument Parser for Rust. The most popular CLI framework with derive macros
for declarative argument definitions. Used for all command-line parsing, subcommands,
help generation, and shell completion support. Features: derive, env, wrap_help, color.

#### clap_complete
**Version:** 4.x | [crates.io](https://crates.io/crates/clap_complete) | [docs.rs](https://docs.rs/clap_complete) | [GitHub](https://github.com/clap-rs/clap)

Shell completion generator for clap. Generates completion scripts for Bash, Zsh, Fish,
PowerShell, and Elvish. Used by the `dx completions` command to create shell-specific
completion files that enable tab-completion of commands and arguments.

---

### Serialization

#### serde
**Version:** 1.x | [crates.io](https://crates.io/crates/serde) | [docs.rs](https://docs.rs/serde) | [GitHub](https://github.com/serde-rs/serde)

The de-facto serialization framework for Rust. Provides derive macros for automatic
serialization/deserialization implementation. Used throughout for JSON, TOML, and
configuration file handling. Features: derive.

#### serde_json
**Version:** 1.x | [crates.io](https://crates.io/crates/serde_json) | [docs.rs](https://docs.rs/serde_json) | [GitHub](https://github.com/serde-rs/json)

JSON serialization/deserialization using serde. Fast and correct JSON parsing with
streaming support. Used by the `dx json` command for formatting, validation, and
querying JSON data, as well as AST serialization in the expr evaluator.

#### toml
**Version:** 0.8.x | [crates.io](https://crates.io/crates/toml) | [docs.rs](https://docs.rs/toml) | [GitHub](https://github.com/toml-rs/toml)

TOML configuration file parser and serializer. Human-friendly configuration format
commonly used in Rust projects. Used by the `dx config` command and for reading
the application's configuration files.

---

### Encoding/Hashing

#### base64
**Version:** 0.22.x | [crates.io](https://crates.io/crates/base64) | [docs.rs](https://docs.rs/base64) | [GitHub](https://github.com/marshallpierce/rust-base64)

Base64 encoding and decoding with multiple alphabet support. Fast implementation
with configurable padding and line wrapping. Used by `dx encode base64` for
encoding strings and files to/from base64 format.

#### hex
**Version:** 0.4.x | [crates.io](https://crates.io/crates/hex) | [docs.rs](https://docs.rs/hex) | [GitHub](https://github.com/KokaKiwi/rust-hex)

Hexadecimal encoding and decoding. Simple API for converting bytes to hex strings
and vice versa. Used by `dx encode hex` and for displaying hash outputs in
hexadecimal format.

#### sha2
**Version:** 0.10.x | [crates.io](https://crates.io/crates/sha2) | [docs.rs](https://docs.rs/sha2) | [GitHub](https://github.com/RustCrypto/hashes)

SHA-2 family hash functions (SHA-256, SHA-384, SHA-512). Part of the RustCrypto
project with pure Rust implementation. Used by `dx hash sha256` and `dx hash sha512`
for computing cryptographic hashes of files and strings.

#### md-5
**Version:** 0.10.x | [crates.io](https://crates.io/crates/md-5) | [docs.rs](https://docs.rs/md-5) | [GitHub](https://github.com/RustCrypto/hashes)

MD5 hash function implementation. Part of RustCrypto, provided for legacy compatibility.
Note: MD5 is cryptographically broken and should not be used for security purposes.
Used by `dx hash md5` for checksums and legacy system compatibility.

---

### Utilities

#### uuid
**Version:** 1.x | [crates.io](https://crates.io/crates/uuid) | [docs.rs](https://docs.rs/uuid) | [GitHub](https://github.com/uuid-rs/uuid)

UUID generation and parsing. Supports all UUID versions with optional features.
Used by `dx uuid` for generating v4 (random) and v7 (timestamp-based) UUIDs.
Features: v4, v7.

#### chrono
**Version:** 0.4.x | [crates.io](https://crates.io/crates/chrono) | [docs.rs](https://docs.rs/chrono) | [GitHub](https://github.com/chronotope/chrono)

Date and time library with timezone support. Comprehensive API for parsing,
formatting, and manipulating dates and times. Used by `dx time` for timestamp
conversion and formatting. Features: serde.

#### directories
**Version:** 6.x | [crates.io](https://crates.io/crates/directories) | [docs.rs](https://docs.rs/directories) | [GitHub](https://github.com/dirs-dev/directories-rs)

Cross-platform standard directory paths. Provides access to config, cache, data,
and other platform-specific directories. Used for locating the application's
configuration file on different operating systems.

#### thiserror
**Version:** 2.x | [crates.io](https://crates.io/crates/thiserror) | [docs.rs](https://docs.rs/thiserror) | [GitHub](https://github.com/dtolnay/thiserror)

Derive macro for custom error types. Simplifies implementing std::error::Error
with minimal boilerplate. Used for defining domain-specific error types in
library code (e.g., expression parser errors).

#### anyhow
**Version:** 1.x | [crates.io](https://crates.io/crates/anyhow) | [docs.rs](https://docs.rs/anyhow) | [GitHub](https://github.com/dtolnay/anyhow)

Flexible error handling with context. Provides easy error propagation with the `?`
operator and context chaining. Used as the primary error type for command functions
(`Result<()>` return type) and in main.rs.

---

### Terminal UI

#### colored
**Version:** 3.x | [crates.io](https://crates.io/crates/colored) | [docs.rs](https://docs.rs/colored) | [GitHub](https://github.com/colored-rs/colored)

Terminal text coloring with ANSI escape codes. Simple trait-based API for adding
colors and styles to strings. Used throughout for colorful output, respects
NO_COLOR environment variable and --no-color flag.

#### dialoguer
**Version:** 0.11.x | [crates.io](https://crates.io/crates/dialoguer) | [docs.rs](https://docs.rs/dialoguer) | [GitHub](https://github.com/console-rs/dialoguer)

Interactive command-line prompts. Provides select menus, confirmations, input
fields, and password prompts. Used for interactive configuration and user input
scenarios in various commands.

#### dx-progress
**Version:** 0.1.x | [local crate](./crates/progress)

Our custom terminal progress library with OSC 9;4 support. Zero dependencies,
integrates with Ghostty, Windows Terminal, and ConEmu native progress bars.
Provides spinners, progress bars, and terminal-native progress reporting.

#### comfy-table
**Version:** 7.x | [crates.io](https://crates.io/crates/comfy-table) | [docs.rs](https://docs.rs/comfy-table) | [GitHub](https://github.com/nukesor/comfy-table)

Pretty-printed tables for terminal output. Supports Unicode, colors, dynamic
column widths, and various presets. Used for tabular output in commands like
`dx env` and `dx expr list`.

#### rand
**Version:** 0.9.x | [crates.io](https://crates.io/crates/rand) | [docs.rs](https://docs.rs/rand) | [GitHub](https://github.com/rust-random/rand)

Random number generation with multiple algorithms. Cryptographically secure
and non-secure RNGs available. Used by `dx rand` for generating random numbers,
strings, passwords, dice rolls, and coin flips.

#### heck
**Version:** 0.5.x | [crates.io](https://crates.io/crates/heck) | [docs.rs](https://docs.rs/heck) | [GitHub](https://github.com/withoutboats/heck)

Case conversion utilities. Converts between camelCase, snake_case, kebab-case,
PascalCase, and more. Used by `dx text` for case transformation commands.

#### lipsum
**Version:** 0.9.x | [crates.io](https://crates.io/crates/lipsum) | [docs.rs](https://docs.rs/lipsum) | [GitHub](https://github.com/mgeisler/lipsum)

Lorem ipsum text generator. Generates placeholder text for testing and mockups.
Used by `dx text lorem` to generate Lorem ipsum paragraphs.

#### bytesize
**Version:** 2.3.x | [crates.io](https://crates.io/crates/bytesize) | [docs.rs](https://docs.rs/bytesize) | [GitHub](https://github.com/bytesize-rs/bytesize)

Byte size formatting and parsing. Converts between bytes and human-readable
formats (KB, MB, GB, etc.). Used by `dx calc bytes` for size conversions.

#### humantime
**Version:** 2.3.x | [crates.io](https://crates.io/crates/humantime) | [docs.rs](https://docs.rs/humantime) | [GitHub](https://github.com/tailhook/humantime)

Human-readable duration parsing and formatting. Parses strings like "2h30m" into
durations and vice versa. Used by `dx calc time` for duration conversions.

---

### Networking

#### url
**Version:** 2.5.x | [crates.io](https://crates.io/crates/url) | [docs.rs](https://docs.rs/url) | [GitHub](https://github.com/servo/rust-url)

URL parsing and manipulation following the WHATWG URL Standard. Handles encoding,
normalization, and URL components. Used by `dx net` for URL parsing and validation.

#### local-ip-address
**Version:** 0.6.x | [crates.io](https://crates.io/crates/local-ip-address) | [docs.rs](https://docs.rs/local-ip-address) | [GitHub](https://github.com/EstebanBorai/local-ip-address)

Cross-platform local IP address detection. Finds the machine's local network IP
address across Windows, macOS, and Linux. Used by `dx net ip` to display local
network information.

#### tokio
**Version:** 1.48.x | [crates.io](https://crates.io/crates/tokio) | [docs.rs](https://docs.rs/tokio) | [GitHub](https://github.com/tokio-rs/tokio)

Asynchronous runtime for Rust. The most widely used async runtime, providing
event loops, I/O, timers, and task scheduling. Used by `dx chat` for async
gRPC communication. Features: rt-multi-thread, macros, sync, io-std, io-util.

#### tonic
**Version:** 0.14.x | [crates.io](https://crates.io/crates/tonic) | [docs.rs](https://docs.rs/tonic) | [GitHub](https://github.com/hyperium/tonic)

gRPC client and server implementation. Built on hyper and tower for high performance.
Used by `dx chat` for real-time gRPC-based messaging.

#### prost
**Version:** 0.14.x | [crates.io](https://crates.io/crates/prost) | [docs.rs](https://docs.rs/prost) | [GitHub](https://github.com/tokio-rs/prost)

Protocol Buffers implementation for Rust. Generates Rust types from .proto files.
Used with tonic for gRPC message serialization in the chat command.

#### prost-types
**Version:** 0.14.x | [crates.io](https://crates.io/crates/prost-types) | [docs.rs](https://docs.rs/prost-types) | [GitHub](https://github.com/tokio-rs/prost)

Well-known Protocol Buffer types (Timestamp, Duration, etc.). Provides standard
protobuf types for use with prost-generated code.

#### tokio-stream
**Version:** 0.1.x | [crates.io](https://crates.io/crates/tokio-stream) | [docs.rs](https://docs.rs/tokio-stream) | [GitHub](https://github.com/tokio-rs/tokio)

Stream utilities for tokio. Provides adapters for working with async streams.
Used for gRPC streaming in the chat command. Features: sync.

#### tonic-prost
**Version:** 0.14.x | [crates.io](https://crates.io/crates/tonic-prost) | [docs.rs](https://docs.rs/tonic-prost) | [GitHub](https://github.com/hyperium/tonic)

Integration between tonic and prost. Provides codec implementations for using
prost-generated types with tonic gRPC services.

---

## Dev Dependencies

#### assert_cmd
**Version:** 2.x | [crates.io](https://crates.io/crates/assert_cmd) | [docs.rs](https://docs.rs/assert_cmd) | [GitHub](https://github.com/assert-rs/assert_cmd)

CLI testing utilities. Wraps std::process::Command for easy testing of binary
crates. Used in `tests/cli.rs` for all CLI integration tests.

#### predicates
**Version:** 3.x | [crates.io](https://crates.io/crates/predicates) | [docs.rs](https://docs.rs/predicates) | [GitHub](https://github.com/assert-rs/predicates-rs)

Composable assertion predicates. Provides matchers for stdout, stderr, and exit
codes. Used with assert_cmd for expressive test assertions.

#### indicatif
**Version:** 0.17.x | [crates.io](https://crates.io/crates/indicatif) | [docs.rs](https://docs.rs/indicatif) | [GitHub](https://github.com/console-rs/indicatif)

Progress bars and spinners for terminal applications. Feature-rich with many
styles and templates. Only used in examples; main code uses dx-progress.

#### insta
**Version:** 1.x | [crates.io](https://crates.io/crates/insta) | [docs.rs](https://docs.rs/insta) | [GitHub](https://github.com/mitsuhiko/insta)

Snapshot testing library. Automatically manages test snapshots with cargo-insta
CLI. Used for testing complex outputs that would be tedious to assert manually.
Features: yaml.

#### tempfile
**Version:** 3.x | [crates.io](https://crates.io/crates/tempfile) | [docs.rs](https://docs.rs/tempfile) | [GitHub](https://github.com/Stebalien/tempfile)

Secure temporary file and directory creation. Automatically cleaned up when
dropped. Used in tests for creating temporary files to hash or process.

#### criterion
**Version:** 0.5.x | [crates.io](https://crates.io/crates/criterion) | [docs.rs](https://docs.rs/criterion) | [GitHub](https://github.com/bheisler/criterion.rs)

Statistics-driven benchmarking library. Provides accurate measurements with
statistical analysis and HTML reports. Used in `benches/` for performance testing.
Features: html_reports.

#### proptest
**Version:** 1.x | [crates.io](https://crates.io/crates/proptest) | [docs.rs](https://docs.rs/proptest) | [GitHub](https://github.com/proptest-rs/proptest)

Property-based testing framework. Generates random inputs to find edge cases.
Used for testing the expression parser with randomly generated expressions.

---

## Build Dependencies

#### clap_mangen
**Version:** 0.2.x | [crates.io](https://crates.io/crates/clap_mangen) | [docs.rs](https://docs.rs/clap_mangen) | [GitHub](https://github.com/clap-rs/clap)

Man page generator for clap. Generates Unix man pages from clap command
definitions. Used in build.rs to generate man pages during compilation.

#### tonic-build
**Version:** 0.14.x | [crates.io](https://crates.io/crates/tonic-build) | [docs.rs](https://docs.rs/tonic-build) | [GitHub](https://github.com/hyperium/tonic)

Code generator for tonic gRPC services. Compiles .proto files to Rust code
during build. Used in build.rs to generate the chat service client/server.

#### tonic-prost-build
**Version:** 0.14.x | [crates.io](https://crates.io/crates/tonic-prost-build) | [docs.rs](https://docs.rs/tonic-prost-build) | [GitHub](https://github.com/hyperium/tonic)

Integration between tonic-build and prost-build. Configures prost code generation
for use with tonic services.

---

## Key Transitive Dependencies

These are notable dependencies pulled in by our direct dependencies:

#### hyper
Pulled by: tonic | [crates.io](https://crates.io/crates/hyper)

Fast HTTP implementation for Rust. Powers tonic's HTTP/2 transport layer.

#### tower
Pulled by: tonic | [crates.io](https://crates.io/crates/tower)

Modular service framework. Provides middleware, retry logic, and load balancing.

#### digest
Pulled by: sha2, md-5 | [crates.io](https://crates.io/crates/digest)

Cryptographic hash function traits. Common interface used by RustCrypto hashes.

#### getrandom
Pulled by: rand, uuid | [crates.io](https://crates.io/crates/getrandom)

Cross-platform random number generation. Provides OS-level randomness.

#### console
Pulled by: dialoguer, indicatif | [crates.io](https://crates.io/crates/console)

Terminal styling and interaction utilities. Handles terminal capabilities.

#### unicode-width
Pulled by: comfy-table | [crates.io](https://crates.io/crates/unicode-width)

Unicode character width calculation. Essential for proper table formatting.

---

## Dependency Policy

1. **Minimize dependencies** - Prefer stdlib when reasonable
2. **Audit security** - Check `cargo audit` regularly
3. **Pin major versions** - Use `"1"` not `"1.2.3"` for flexibility
4. **Document purpose** - Every dependency should have a clear use case
5. **Prefer well-maintained** - Check activity, issues, and bus factor
