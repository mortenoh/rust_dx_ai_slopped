# Testing with assert_cmd

Test CLI binaries by running the actual binary.

## Setup

```bash
cargo add --dev assert_cmd predicates
```

## Basic Test

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help() {
    Command::cargo_bin("dx")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}
```

## Testing Arguments

```rust
#[test]
fn test_hash_file() {
    Command::cargo_bin("dx")
        .unwrap()
        .args(["hash", "Cargo.toml"])
        .assert()
        .success();
}
```

## Testing stdin

```rust
#[test]
fn test_stdin() {
    Command::cargo_bin("dx")
        .unwrap()
        .args(["hash", "-"])
        .write_stdin("hello")
        .assert()
        .success();
}
```

## Testing Exit Codes

```rust
#[test]
fn test_missing_file() {
    Command::cargo_bin("dx")
        .unwrap()
        .args(["hash", "/nonexistent"])
        .assert()
        .failure()
        .code(1);
}
```

## Environment Variables

```rust
#[test]
fn test_env() {
    Command::cargo_bin("dx")
        .unwrap()
        .env("DX_ALGORITHM", "sha512")
        .args(["hash", "file.txt"])
        .assert()
        .success();
}
```
