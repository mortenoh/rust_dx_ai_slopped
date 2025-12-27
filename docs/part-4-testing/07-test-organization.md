# Test Organization

Best practices for organizing tests.

## Directory Structure

```
my_cli/
├── src/
│   ├── lib.rs
│   └── commands/
│       └── hash.rs
├── tests/
│   ├── common/mod.rs
│   ├── cli_tests.rs
│   └── integration/
└── benches/
```

## Unit Tests in Source

```rust
// src/commands/hash.rs
pub fn compute_hash(input: &str) -> Result<String> {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() { }

    mod edge_cases {
        use super::*;

        #[test]
        fn test_empty() { }
    }
}
```

## Naming Conventions

```rust
// test_<function>_<scenario>_<expected>
#[test]
fn test_parse_valid_input_returns_config() { }

#[test]
fn test_parse_empty_input_returns_error() { }
```

## Running Specific Tests

```bash
cargo test hash                 # By name
cargo test commands::hash       # By module
cargo test --test cli_tests     # By file
cargo test -- --ignored         # Ignored tests
```

## Test Attributes

```rust
#[test]
fn normal_test() { }

#[test]
#[ignore]
fn slow_test() { }

#[test]
#[cfg(target_os = "linux")]
fn linux_only() { }
```
