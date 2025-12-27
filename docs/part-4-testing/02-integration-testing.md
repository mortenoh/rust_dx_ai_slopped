# Integration Testing

Integration tests verify modules work together.

## Directory Structure

```
my_project/
├── src/lib.rs
└── tests/
    ├── common/mod.rs
    └── hash_tests.rs
```

## Basic Integration Test

```rust
// tests/hash_tests.rs
use my_crate::hash::compute_hash;

#[test]
fn test_sha256_hash() {
    let result = compute_hash("hello", Algorithm::Sha256);
    assert!(result.is_ok());
}
```

## Shared Utilities

```rust
// tests/common/mod.rs
use tempfile::TempDir;

pub fn create_temp_file(content: &str) -> (TempDir, PathBuf) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, content).unwrap();
    (dir, path)
}
```

## Testing File Operations

```rust
use tempfile::tempdir;

#[test]
fn test_write_output() {
    let dir = tempdir().unwrap();
    let output = dir.path().join("out.txt");

    write_output(&output, "content").unwrap();

    assert!(output.exists());
}
```
