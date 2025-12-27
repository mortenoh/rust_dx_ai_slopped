# Platform Testing

Test your CLI on multiple platforms.

## GitHub Actions Matrix

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Run tests
        run: cargo test --all-features

      - name: Run clippy
        run: cargo clippy -- -D warnings
```

## Platform-Specific Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_common() {
        // Runs on all platforms
        assert!(true);
    }

    #[test]
    #[cfg(unix)]
    fn test_unix_permissions() {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::metadata("Cargo.toml")
            .unwrap()
            .permissions();
        assert!(perms.mode() & 0o400 != 0); // readable
    }

    #[test]
    #[cfg(windows)]
    fn test_windows_paths() {
        let path = std::path::Path::new("C:\\Windows");
        assert!(path.is_absolute());
    }
}
```

## Ignore on Platform

```rust
#[test]
#[cfg_attr(windows, ignore)]
fn test_requires_unix() {
    // Ignored on Windows
}

#[test]
#[cfg_attr(not(target_os = "linux"), ignore)]
fn test_linux_only() {
    // Only runs on Linux
}
```

## Testing Environment Variables

```rust
#[test]
fn test_config_dir() {
    let config = crate::get_config_dir();

    if cfg!(target_os = "linux") {
        assert!(config.to_string_lossy().contains(".config"));
    } else if cfg!(target_os = "macos") {
        assert!(config.to_string_lossy().contains("Library"));
    } else if cfg!(windows) {
        assert!(config.to_string_lossy().contains("AppData"));
    }
}
```

## Testing with Docker

Test Linux from macOS/Windows:

```dockerfile
# Dockerfile.test
FROM rust:1.75

WORKDIR /app
COPY . .

RUN cargo test
```

```yaml
# docker-compose.yml
services:
  test:
    build:
      context: .
      dockerfile: Dockerfile.test
```

```bash
docker-compose run test
```

## Cross-Platform Test Utilities

```rust
#[cfg(test)]
mod test_utils {
    use std::path::PathBuf;

    pub fn test_file_path(name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("fixtures");
        path.push(name);
        path
    }

    pub fn normalize_line_endings(s: &str) -> String {
        s.replace("\r\n", "\n")
    }
}

#[test]
fn test_output() {
    let output = run_command();
    let normalized = test_utils::normalize_line_endings(&output);
    assert_eq!(normalized, "expected\n");
}
```

## Extended CI Matrix

```yaml
jobs:
  test:
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Install target
        run: rustup target add ${{ matrix.target }}

      - name: Test
        run: cargo test --target ${{ matrix.target }}
```
