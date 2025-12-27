//! # Documenting CLI Commands
//!
//! This example shows how to document CLI commands effectively.
//!
//! Run with: `cargo run --example 0702_command_docs`

#![allow(dead_code)]

fn main() {
    println!("=== Documenting CLI Commands ===\n");

    // =========================================================================
    // COMMAND DOCUMENTATION TEMPLATE
    // =========================================================================

    println!("--- Command Documentation Template ---");
    println!(
        r##"
# docs/src/commands/hash.md

# hash

Calculate file hashes using various algorithms.

## Synopsis

```
dx hash [OPTIONS] <FILE>
dx hash [OPTIONS] -
```

## Description

The `hash` command calculates cryptographic hashes of files or stdin.
Supported algorithms include MD5, SHA-256, and SHA-512.

## Arguments

| Argument | Description |
|----------|-------------|
| `<FILE>` | Path to file to hash, or `-` for stdin |

## Options

| Option | Default | Description |
|--------|---------|-------------|
| `-a, --algorithm <ALG>` | `sha256` | Hash algorithm: `md5`, `sha256`, `sha512` |
| `-o, --output <FMT>` | `hex` | Output format: `hex`, `base64` |
| `--verify <HASH>` | | Verify file matches expected hash |
| `-q, --quiet` | | Only output the hash value |

## Examples

### Hash a file

```bash
$ dx hash myfile.txt
sha256: 2cf24dba5fb0a30e26e83b2ac5b9e29e...

$ dx hash -a md5 myfile.txt
md5: 5d41402abc4b2a76b9719d911017c592
```

### Hash stdin

```bash
$ echo "hello" | dx hash -
sha256: 2cf24dba5fb0a30e26e83b2ac5b9e29e...

$ cat largefile.bin | dx hash -a sha512 -
sha512: 9b71d224bd62f3785d96d46ad3ea3d73...
```

### Verify a hash

```bash
$ dx hash --verify 2cf24dba... myfile.txt
✓ Hash matches

$ dx hash --verify wronghash myfile.txt
✗ Hash mismatch
  Expected: wronghash
  Got:      2cf24dba...
```

### Output formats

```bash
# Hex output (default)
$ dx hash myfile.txt
2cf24dba5fb0a30e26e83b2ac5b9e29e...

# Base64 output
$ dx hash -o base64 myfile.txt
LPJNul+wow4m6DsqxbninhsWHowMuJGqU...

# Quiet mode (hash only)
$ dx hash -q myfile.txt
2cf24dba5fb0a30e26e83b2ac5b9e29e
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Hash mismatch (with --verify) |
| 2 | File not found or read error |

## See Also

- [encode](./encode.md) - Base64/hex encoding
- [Environment Variables](../reference/environment.md)
"##
    );

    println!();

    // =========================================================================
    // OVERVIEW PAGE
    // =========================================================================

    println!("--- Commands Overview Page ---");
    println!(
        r#"
# docs/src/commands/overview.md

# Commands Overview

dx provides the following commands:

| Command | Description |
|---------|-------------|
| [hash](./hash.md) | Calculate file hashes |
| [encode](./encode.md) | Base64/hex encoding |
| [uuid](./uuid.md) | Generate UUIDs |
| [time](./time.md) | Timestamp conversion |
| [json](./json.md) | JSON formatting |
| [env](./env.md) | Environment variables |
| [config](./config.md) | Configuration management |

## Global Options

These options work with all commands:

| Option | Description |
|--------|-------------|
| `-h, --help` | Show help for command |
| `-V, --version` | Show version |
| `--no-color` | Disable colored output |
| `-v, --verbose` | Increase verbosity |
| `-q, --quiet` | Suppress non-error output |

## Getting Help

```bash
# General help
dx --help

# Command-specific help
dx hash --help
dx encode --help
```
"#
    );

    println!();

    // =========================================================================
    // GENERATED HELP
    // =========================================================================

    println!("--- Including Generated Help ---");
    println!(
        r##"
Auto-include CLI help output in docs:

# build.rs or script
fn generate_help_docs() {{
    use std::process::Command;

    let commands = ["hash", "encode", "uuid", "time", "json"];

    for cmd in commands {{
        let output = Command::new("./target/release/dx")
            .args([cmd, "--help"])
            .output()
            .unwrap();

        let help = String::from_utf8_lossy(&output.stdout);
        let path = format!("docs/src/generated/{{}}_help.txt", cmd);
        std::fs::write(&path, help.as_ref()).unwrap();
    }}
}}

# In markdown:
```
{{{{#include ../generated/hash_help.txt}}}}
```

This keeps docs in sync with actual CLI output!
"##
    );

    println!();

    // =========================================================================
    // EXAMPLES WITH OUTPUT
    // =========================================================================

    println!("--- Examples with Output ---");
    println!(
        r#"
Show command examples with expected output:

## Examples

### Basic Usage

```console
$ dx uuid
550e8400-e29b-41d4-a716-446655440000
```

### With Options

```console
$ dx uuid --type v7 --count 3
01893c8b-d7a9-7000-8000-000000000001
01893c8b-d7a9-7000-8000-000000000002
01893c8b-d7a9-7000-8000-000000000003
```

### Error Case

```console
$ dx hash /nonexistent/file
error: File not found: /nonexistent/file

Tip: Check that the path exists and you have read permission.
```

Use `console` language for shell examples to get proper highlighting.
"#
    );

    println!();

    // =========================================================================
    // ADMONITIONS
    // =========================================================================

    println!("--- Admonitions ---");
    println!(
        r#"
Add warnings and tips:

> **Note:** This command requires network access.

> **Warning:** This operation cannot be undone!

> **Tip:** Use `-q` flag for scripting.

Or with mdbook-admonish preprocessor:

```admonish warning
This will delete all data!
```

```admonish tip
You can pipe output to other commands.
```

```admonish note
Available since version 1.2.0
```
"#
    );

    println!();

    // =========================================================================
    // VERSIONING
    // =========================================================================

    println!("--- Version-Specific Documentation ---");
    println!(
        r#"
Document version differences:

## Options

| Option | Since | Description |
|--------|-------|-------------|
| `-a, --algorithm` | 1.0 | Hash algorithm |
| `--verify` | 1.2 | Verify hash |
| `--parallel` | 2.0 | Parallel processing |

## Changelog

### v2.0.0

- Added `--parallel` flag for faster multi-file hashing
- Breaking: Changed default algorithm from MD5 to SHA-256

### v1.2.0

- Added `--verify` flag
- Added base64 output format

### v1.0.0

- Initial release
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Command documentation:");
    println!("  1. Use consistent template for all commands");
    println!("  2. Include synopsis, options, examples");
    println!("  3. Show both success and error cases");
    println!("  4. Auto-generate from --help when possible");
    println!("  5. Document exit codes and version history");
}
