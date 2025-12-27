# Introduction to Testing

Rust has first-class testing support built into the language. This part covers testing strategies for CLI applications.

## Testing Pyramid

```
         /\       End-to-End (CLI execution)
        /  \
       /----\     Integration (modules)
      /      \
     /--------\   Unit (functions)
```

## Test Types

| Type | Scope | Tools |
|------|-------|-------|
| Unit | Functions | Built-in |
| Integration | Modules | Built-in |
| CLI | Binary | assert_cmd |
| Snapshot | Output | insta |
| Property | Invariants | proptest |

## Running Tests

```bash
cargo test                    # All tests
cargo test unit_              # By name
cargo test --lib              # Library only
cargo test -- --nocapture     # Show output
```
