# Platform Detection

Detect the current operating system and architecture.

## Compile-Time Detection

```rust
#[cfg(target_os = "linux")]
fn platform() -> &'static str { "linux" }

#[cfg(target_os = "macos")]
fn platform() -> &'static str { "macos" }

#[cfg(target_os = "windows")]
fn platform() -> &'static str { "windows" }
```

## Common cfg Attributes

### Operating System

```rust
#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
#[cfg(target_os = "windows")]
#[cfg(target_os = "freebsd")]
#[cfg(target_os = "android")]
#[cfg(target_os = "ios")]
```

### OS Family

```rust
#[cfg(unix)]      // Linux, macOS, BSD, etc.
#[cfg(windows)]   // Windows only
```

### Architecture

```rust
#[cfg(target_arch = "x86_64")]
#[cfg(target_arch = "aarch64")]  // ARM64
#[cfg(target_arch = "x86")]
#[cfg(target_arch = "arm")]
```

### Pointer Width

```rust
#[cfg(target_pointer_width = "64")]
#[cfg(target_pointer_width = "32")]
```

## Runtime Detection

```rust
fn get_platform_info() -> String {
    format!(
        "OS: {} ({}), Arch: {}",
        std::env::consts::OS,
        std::env::consts::FAMILY,
        std::env::consts::ARCH,
    )
}

fn main() {
    println!("{}", get_platform_info());
    // OS: macos (unix), Arch: aarch64
}
```

## Available Constants

```rust
use std::env::consts::*;

OS           // "linux", "macos", "windows"
FAMILY       // "unix", "windows"
ARCH         // "x86_64", "aarch64"
DLL_PREFIX   // "lib" on Unix, "" on Windows
DLL_SUFFIX   // ".so", ".dylib", ".dll"
DLL_EXTENSION // "so", "dylib", "dll"
EXE_SUFFIX   // "" on Unix, ".exe" on Windows
EXE_EXTENSION // "", "exe"
```

## Practical Example

```rust
fn get_shell() -> &'static str {
    if cfg!(windows) {
        "cmd.exe"
    } else {
        "/bin/sh"
    }
}

fn get_config_path() -> PathBuf {
    if cfg!(target_os = "macos") {
        dirs::home_dir()
            .unwrap()
            .join("Library/Application Support/dx")
    } else if cfg!(windows) {
        dirs::config_dir().unwrap().join("dx")
    } else {
        dirs::config_dir().unwrap().join("dx")
    }
}
```

## Feature Flags by Platform

```toml
# Cargo.toml
[target.'cfg(unix)'.dependencies]
nix = "0.28"

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser"] }
```

## Using cfg! Macro

```rust
fn main() {
    if cfg!(debug_assertions) {
        println!("Debug build");
    }

    let separator = if cfg!(windows) { '\\' } else { '/' };
    println!("Path separator: {}", separator);
}
```
