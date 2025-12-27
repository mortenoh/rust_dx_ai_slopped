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
├── expr/                   # Expression evaluator library
│   ├── mod.rs              # Public API: parse(), parse_to_ast()
│   ├── ast.rs              # Expr, BinOp, UnaryOp enums
│   └── parser.rs           # Recursive descent parser
└── utils/
    ├── mod.rs
    └── output.rs
```

## Available Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `hash`  | `h`   | Compute file/string hashes (MD5, SHA256, SHA512) |
| `encode`| `e`   | Base64, hex encoding/decoding |
| `uuid`  | `u`   | Generate UUIDs (v4, v7) |
| `time`  | `t`   | Parse, format, convert timestamps |
| `json`  | `j`   | Format, validate, query JSON |
| `env`   | -     | Inspect/export environment variables |
| `config`| `cfg` | Manage application configuration |
| `rand`  | `r`   | Generate random data (numbers, strings, passwords) |
| `text`  | -     | Text transformations (case, slugify) |
| `calc`  | `c`   | Unit conversions (bytes, time, base, percent) |
| `expr`  | `x`   | Expression evaluator (math, functions, constants) |
| `net`   | -     | Network utilities (IP, DNS, ports) |
| `chat`  | -     | gRPC-based real-time chat (async) |
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

A reusable recursive descent parser with AST support:

```rust
use rust_cli_complete::expr;

// Evaluate expression
let result = expr::parse("2 + 3 * 4")?;  // 14.0

// Get AST (serializable with serde)
let ast = expr::parse_to_ast("sin(pi/2)")?;
let json = serde_json::to_string(&ast)?;
```

**Supported features:**
- Operators: `+`, `-`, `*`, `/`, `%` (modulo), `^` (power, right-associative)
- Constants: `pi`, `e`, `tau`
- Functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, sqrt, cbrt, abs, floor, ceil, round, trunc, exp, ln, log2, log10

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

# Cross-compile (requires cargo-zigbuild)
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
- `assert_cmd` / `predicates` - CLI testing

## File Naming Conventions

- CLI args: `src/cli/commands/<command>.rs`
- Implementation: `src/commands/<command>.rs`
- Tests: `tests/cli.rs` (all CLI tests in one file)
- Examples: `examples/PPNN_topic_name.rs` (PP = phase, NN = number)
