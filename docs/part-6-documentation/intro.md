# Introduction to Documentation

Good documentation is essential for CLI tools. This part covers all documentation aspects.

## Documentation Layers

### User Documentation
- **README** - Quick start and overview
- **Man pages** - Traditional Unix help
- **mdbook** - Comprehensive guides
- **`--help`** - Inline command help

### Developer Documentation
- **rustdoc** - API documentation
- **Doc tests** - Verified examples
- **CONTRIBUTING** - Contribution guide

## Documentation Philosophy

### Write for Your Audience

```
Users want:        Developers want:
- Quick examples   - API details
- Common tasks     - Implementation notes
- Error solutions  - Architecture docs
```

### Keep Examples Working

```rust
/// Compute hash of data
///
/// ```
/// use dx::hash;
/// let result = hash("hello");
/// assert!(!result.is_empty());
/// ```
pub fn hash(data: &str) -> String {
    // Doc test ensures this example always works
}
```

## What You'll Learn

| Chapter | Topic |
|---------|-------|
| 1 | mdbook setup and configuration |
| 2 | mdbook features (code, search, themes) |
| 3 | Preprocessors for extended functionality |
| 4 | Custom themes and styling |
| 5 | rustdoc for API documentation |
| 6 | Doc tests for verified examples |
| 7 | Man page generation |

## Tools Overview

```bash
# mdbook - User documentation
cargo install mdbook
mdbook build docs/

# rustdoc - API documentation
cargo doc --open

# clap_mangen - Man pages
cargo run --example generate-man
```
