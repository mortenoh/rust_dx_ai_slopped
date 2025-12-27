# Conditional Compilation

Include or exclude code based on platform.

## Basic cfg Attribute

```rust
#[cfg(unix)]
fn get_user() -> String {
    std::env::var("USER").unwrap_or_default()
}

#[cfg(windows)]
fn get_user() -> String {
    std::env::var("USERNAME").unwrap_or_default()
}
```

## cfg_attr

Apply attributes conditionally:

```rust
// Only derive Debug on non-release builds
#[cfg_attr(debug_assertions, derive(Debug))]
struct Config {
    api_key: String,
}

// Platform-specific derives
#[cfg_attr(windows, derive(Default))]
struct WindowsConfig {
    registry_key: String,
}
```

## Combining Conditions

### all (AND)

```rust
#[cfg(all(unix, target_arch = "x86_64"))]
fn unix_64bit_only() {
    println!("Unix on x86_64");
}
```

### any (OR)

```rust
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn linux_or_mac() {
    println!("Linux or macOS");
}
```

### not (NOT)

```rust
#[cfg(not(windows))]
fn not_windows() {
    println!("Any OS except Windows");
}
```

### Complex Conditions

```rust
#[cfg(all(
    unix,
    not(target_os = "macos"),
    target_pointer_width = "64"
))]
fn linux_64bit() {
    println!("64-bit Linux");
}
```

## Module-Level cfg

```rust
#[cfg(unix)]
mod unix_impl;

#[cfg(windows)]
mod windows_impl;

// Re-export the correct implementation
#[cfg(unix)]
pub use unix_impl::*;

#[cfg(windows)]
pub use windows_impl::*;
```

## Platform-Specific Modules

```
src/
├── main.rs
├── platform/
│   ├── mod.rs
│   ├── unix.rs
│   └── windows.rs
```

```rust
// src/platform/mod.rs
#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;
```

## Feature-Based Compilation

```toml
# Cargo.toml
[features]
default = []
tui = ["dep:ratatui"]
gui = ["dep:egui"]
```

```rust
#[cfg(feature = "tui")]
fn run_tui() {
    // TUI implementation
}

#[cfg(feature = "gui")]
fn run_gui() {
    // GUI implementation
}
```

## Testing with cfg

```rust
#[cfg(test)]
mod tests {
    #[test]
    #[cfg(unix)]
    fn test_unix_paths() {
        assert!(std::path::Path::new("/tmp").exists());
    }

    #[test]
    #[cfg(windows)]
    fn test_windows_paths() {
        assert!(std::path::Path::new("C:\\").exists());
    }
}
```

## Compile-Time Assertions

```rust
#[cfg(not(any(unix, windows)))]
compile_error!("This crate only supports Unix and Windows");

#[cfg(not(target_pointer_width = "64"))]
compile_error!("This crate requires 64-bit architecture");
```
