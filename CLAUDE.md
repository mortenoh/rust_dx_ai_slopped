# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Important Rules

### No Attribution
Do not add any AI attribution to commits, PRs, or anywhere else. This means:
- No "Generated with Claude Code" in commit messages
- No "Co-Authored-By: Claude" lines
- No AI-related comments in code or documentation

### Pre-commit Checks
Always run `make lint` before committing. This runs `cargo fmt` and `cargo clippy --fix`.

### Adding Dependencies
Always use `cargo add` to add new dependencies instead of manually editing Cargo.toml. This ensures we get the latest compatible versions:
```bash
cargo add polars --features lazy,csv,parquet
cargo add some-crate --no-default-features --features feat1,feat2
```

### Feature Completeness
When adding or modifying features, always update ALL related artifacts:
- **README.md** - Update feature descriptions and examples
- **docs/** - Update relevant documentation in the mdbook
- **tests/cli.rs** - Add integration tests for new functionality
- **CLAUDE.md** - Update command descriptions if applicable

### GitHub CLI
Use `gh` (GitHub CLI) to check PRs, build status, and other GitHub operations.

### Conventional Commits
Use conventional commits for all commit messages and branch names:

**Commit format:** `<type>(<scope>): <description>`

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`

Examples:
```
feat(expr): add modulo operator support
fix(hash): handle empty input correctly
docs(readme): update installation instructions
refactor(cli): extract common argument parsing
test(calc): add unit conversion tests
```

**Branch naming:** `<type>/<short-description>`

Examples:
```
feat/add-grep-command
fix/hash-empty-input
refactor/extract-parser
```

## Project Overview

**dx** (Developer Experience CLI) is a production-ready developer toolkit written in Rust. It demonstrates CLI best practices including proper error handling, comprehensive testing, shell completions, and cross-platform support.

## Architecture

### Thin Main Pattern

The codebase follows the "thin main" pattern:
- `src/main.rs` - CLI parsing and command dispatch only (minimal logic)
- `src/lib.rs` - Library root, exposes public modules
- `src/commands/` - Business logic for each command
- `src/cli/` - Argument definitions using clap derive macros
- `src/config/` - Configuration management
- `src/expr/` - Expression parser library (reusable)

### Module Structure

```
src/
├── main.rs                 # Entry point, command dispatch
├── lib.rs                  # Library exports
├── cli/
│   ├── mod.rs              # Re-exports
│   ├── args.rs             # Top-level CLI struct, Commands enum
│   └── commands/           # Argument structs per command
│       ├── mod.rs
│       ├── hash.rs         # HashArgs, HashCommand
│       ├── encode.rs       # EncodeArgs, EncodeCommand
│       └── ...
├── commands/               # Command implementations
│   ├── mod.rs
│   ├── hash.rs             # pub fn run(args: HashArgs) -> Result<()>
│   ├── encode.rs
│   └── ...
├── config/                 # Configuration system
│   ├── mod.rs
│   └── settings.rs
├── expr/                   # Re-exports dx-expr crate
│   └── mod.rs              # pub use dx_expr::*
└── utils/
    ├── mod.rs
    └── output.rs
```

## Available Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `hash`  | `h`   | Compute hashes (MD5, SHA256, SHA512, Bcrypt, Argon2) |
| `encode`| `e`   | Base64, hex encoding/decoding |
| `uuid`  | `u`   | Generate UUIDs (v4, v7) |
| `time`  | `t`   | Parse, format, convert timestamps |
| `json`  | `j`   | Format, validate, query JSON |
| `yaml`  | `y`   | Format, validate, convert YAML |
| `csv`   | -     | Format, query, convert CSV |
| `xml`   | -     | Format, validate, convert XML |
| `jwt`   | -     | Decode, encode, verify JWT tokens |
| `encrypt` | -   | Encrypt/decrypt with AES-GCM or ChaCha20-Poly1305 |
| `diff`  | -     | Text diffing (unified, inline, compact) |
| `template` | -  | Jinja2-style template rendering with Tera |
| `markdown` | `md` | Markdown to HTML and TOC extraction |
| `compress` | -  | Gzip/Zstd compression and decompression |
| `env`   | -     | Inspect/export environment variables |
| `config`| `cfg` | Manage application configuration |
| `rand`  | `r`   | Generate random data (numbers, strings, passwords) |
| `text`  | -     | Text transformations (case, slugify) |
| `calc`  | `c`   | Unit conversions (bytes, time, base, percent) |
| `expr`  | `x`   | Expression evaluator (math, functions, constants) |
| `net`   | -     | Network utilities (IP, DNS, ports) |
| `chat`  | -     | gRPC-based real-time chat (async) |
| `fun`   | -     | Fun terminal effects (matrix, life, qr, clock, banner) |
| `grep`  | `g`   | Regex search in files with context |
| `http`  | -     | HTTP client (GET, POST, PUT, DELETE, HEAD) |
| `watch` | `w`   | Watch files and run commands on changes |
| `system`| `sys` | System information (CPU, memory, OS, uptime) |
| `ui`    | -     | TUI dashboard (requires `--features ui`) |
| `egui`  | -     | GUI demos (requires `--features egui`) |
| `completions` | - | Generate shell completions |

## Adding a New Command

1. **Create argument struct** in `src/cli/commands/<name>.rs`:
   ```rust
   use clap::{Args, Subcommand};

   #[derive(Args, Debug)]
   pub struct FooArgs {
       #[command(subcommand)]
       pub command: FooCommand,
   }

   #[derive(Subcommand, Debug)]
   pub enum FooCommand {
       /// Description of subcommand
       Bar { value: String },
   }
   ```

2. **Export from** `src/cli/commands/mod.rs`:
   ```rust
   pub use foo::FooArgs;
   ```

3. **Add variant** to `Commands` enum in `src/cli/args.rs`:
   ```rust
   #[command(visible_alias = "f")]
   Foo(FooArgs),
   ```

4. **Create command implementation** in `src/commands/<name>.rs`:
   ```rust
   use crate::cli::commands::foo::{FooArgs, FooCommand};
   use anyhow::Result;

   pub fn run(args: FooArgs) -> Result<()> {
       match args.command {
           FooCommand::Bar { value } => cmd_bar(&value),
       }
   }

   fn cmd_bar(value: &str) -> Result<()> {
       println!("{}", value);
       Ok(())
   }
   ```

5. **Export from** `src/commands/mod.rs`:
   ```rust
   pub mod foo;
   ```

6. **Add dispatch** in `src/main.rs`:
   ```rust
   Commands::Foo(args) => commands::foo::run(args),
   ```

7. **Add tests** in `tests/cli.rs`:
   ```rust
   #[test]
   fn test_foo_bar() {
       let mut cmd = Command::cargo_bin("dx").unwrap();
       cmd.args(["foo", "bar", "test"])
           .assert()
           .success()
           .stdout(predicate::str::contains("test"));
   }
   ```

## Expression Library (`src/expr/`)

A full-featured expression evaluator with user-defined functions, lambdas, and closures:

```rust
use rust_cli_complete::expr::{eval, eval_program, eval_with_context, Context};

// Simple evaluation
let result = eval("2 + 3 * 4")?;  // 14.0

// Multi-statement programs with variables
let result = eval_program("x = 5; y = x + 3; y * 2")?;  // 16.0

// User-defined functions
let result = eval_program("def square(x) = x * x; square(5)")?;  // 25.0

// With predefined variables
let mut ctx = Context::new();
ctx.set("radius", 5.0);
let area = eval_with_context("pi * radius ^ 2", &mut ctx)?;
```

**Supported features:**
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `^`, `**`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `and`/`&&`, `or`/`||`, `not`/`!`
- Conditionals: `if ... then ... else ...`
- Constants: `pi`, `e`, `tau`, `true`, `false`
- 30+ built-in functions (trig, log, rounding, multi-arg, variadic)
- Variables and multi-statement programs
- User-defined functions: `def name(params) = expr`
- Lambda expressions: `x => expr`, `(a, b) => expr`
- Closures that capture outer scope
- Comments: `# to end of line`
- AST serialization to JSON with serde

## Development Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build

# Test
cargo test                     # All tests
cargo test --test cli          # CLI integration tests only

# Lint and format
cargo clippy                   # Linting
cargo fmt                      # Format code

# Run
cargo run -- hash sha256 "hello"
cargo run -- expr eval "2^10"

# Generate completions
cargo run -- completions bash > dx.bash
cargo run -- completions zsh > _dx
cargo run -- completions fish > dx.fish

# Cross-compile (requires cargo-zigbuild for Linux, cargo-xwin for Windows)
make build-all                 # All targets
make release-all               # Release builds for all targets
```

## Testing

Tests are in `tests/cli.rs` using `assert_cmd` and `predicates`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_command() {
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["subcommand", "arg"])
        .assert()
        .success()
        .stdout(predicate::str::contains("expected"));
}

// Test stdin input
#[test]
fn test_stdin() {
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["hash", "sha256", "-"])
        .write_stdin("input")
        .assert()
        .success();
}

// Test error cases
#[test]
fn test_error() {
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["invalid"])
        .assert()
        .failure();
}
```

## Key Patterns

### Error Handling
- Use `anyhow::Result<()>` for command functions
- Use `anyhow::bail!()` for early returns with errors
- Use `.context()` to add context to errors

### Global Flags
- `--no-color` / `NO_COLOR` env: Disable colored output
- `--verbose` / `-v`: Enable verbose output
- `--output` / `-o`: Output format (text, json, quiet)

### Stdin Support
Many commands accept `-` to read from stdin:
```bash
echo "hello" | dx hash sha256 -
cat file.json | dx json fmt -
```

## Dependencies

Key crates used:
- `clap` - CLI argument parsing (derive macros)
- `clap_complete` - Shell completion generation
- `anyhow` - Error handling
- `colored` - Terminal colors
- `serde` / `serde_json` - Serialization
- `tokio` - Async runtime (for chat command)
- `tonic` - gRPC (for chat command)
- `dx-progress` - Our terminal progress library (OSC 9;4 support)
- `assert_cmd` / `predicates` - CLI testing

See **[DEPENDENCIES.md](./DEPENDENCIES.md)** for complete documentation of all dependencies including descriptions, links, and transitive dependencies.

## File Naming Conventions

- CLI args: `src/cli/commands/<command>.rs`
- Implementation: `src/commands/<command>.rs`
- Tests: `tests/cli.rs` (all CLI tests in one file)
