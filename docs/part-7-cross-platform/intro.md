# Introduction to Cross-Platform

Build CLI tools that work on Linux, macOS, and Windows.

## Why Cross-Platform?

- Wider user base
- Consistent behavior everywhere
- CI/CD on multiple platforms

## Platform Differences

| Aspect | Linux/macOS | Windows |
|--------|-------------|---------|
| Path separator | `/` | `\` |
| Line endings | `\n` | `\r\n` |
| Executable ext | none | `.exe` |
| Config location | `~/.config/` | `%APPDATA%\` |
| Home variable | `$HOME` | `%USERPROFILE%` |

## Rust's Cross-Platform Support

```rust
use std::path::PathBuf;

// Automatically uses correct separator
let path = PathBuf::from("config").join("settings.toml");

// Platform-agnostic home directory
fn config_dir() -> Option<PathBuf> {
    dirs::config_dir()
}
```

## Common Challenges

### File Paths
```rust
// Wrong: Hardcoded separator
let path = "config/app.toml";

// Right: Use PathBuf
let path = PathBuf::from("config").join("app.toml");
```

### Environment Variables
```rust
// Wrong: Unix-only
let home = std::env::var("HOME")?;

// Right: Use dirs crate
let home = dirs::home_dir().ok_or("No home directory")?;
```

### Process Execution
```rust
// Wrong: Unix-only
Command::new("ls").arg("-la").spawn()?;

// Right: Use Rust's std
for entry in std::fs::read_dir(".")? {
    println!("{}", entry?.path().display());
}
```

## What You'll Learn

| Chapter | Topic |
|---------|-------|
| 1 | Detecting the current platform |
| 2 | Conditional compilation with cfg |
| 3 | Cross-platform path handling |
| 4 | Testing on multiple platforms |
| 5 | Cross-compiling binaries |

## Helpful Crates

```toml
[dependencies]
dirs = "5"           # Standard directories
which = "6"          # Find executables
open = "5"           # Open files with default app
```
