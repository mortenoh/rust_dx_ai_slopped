# Path Handling

Handle file paths correctly across platforms.

## PathBuf and Path

```rust
use std::path::{Path, PathBuf};

// Create path using join (handles separators)
let config = PathBuf::from("config").join("app.toml");

// Convert to string (display)
println!("Config: {}", config.display());

// Path operations
let parent = config.parent();
let filename = config.file_name();
let extension = config.extension();
```

## Standard Directories

Use the `dirs` crate:

```toml
[dependencies]
dirs = "5"
```

```rust
use dirs;

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .expect("No config directory")
        .join("dx")
        .join("config.toml")
}

fn get_cache_path() -> PathBuf {
    dirs::cache_dir()
        .expect("No cache directory")
        .join("dx")
}

fn get_data_path() -> PathBuf {
    dirs::data_dir()
        .expect("No data directory")
        .join("dx")
}
```

### Directory Locations

| Function | Linux | macOS | Windows |
|----------|-------|-------|---------|
| `home_dir()` | `$HOME` | `$HOME` | `%USERPROFILE%` |
| `config_dir()` | `~/.config` | `~/Library/Application Support` | `%APPDATA%` |
| `cache_dir()` | `~/.cache` | `~/Library/Caches` | `%LOCALAPPDATA%` |
| `data_dir()` | `~/.local/share` | `~/Library/Application Support` | `%APPDATA%` |

## Path Manipulation

```rust
let path = PathBuf::from("/home/user/documents/file.txt");

// Components
path.parent()              // /home/user/documents
path.file_name()           // file.txt
path.file_stem()           // file
path.extension()           // txt

// Joining
path.join("subdir")        // /home/user/documents/file.txt/subdir
path.with_file_name("new") // /home/user/documents/new
path.with_extension("md")  // /home/user/documents/file.md

// Checking
path.exists()
path.is_file()
path.is_dir()
path.is_absolute()
```

## Canonicalization

```rust
use std::fs;

// Get absolute path (resolves symlinks)
let canonical = fs::canonicalize("./relative/path")?;

// On Windows, returns \\?\C:\... prefix
// Use dunce crate to get normal paths on Windows
```

### Using dunce

```toml
[dependencies]
dunce = "1"
```

```rust
// Normal canonicalize on Windows
let path = dunce::canonicalize("./file")?;
// Returns C:\path\file instead of \\?\C:\path\file
```

## Handling Separators

```rust
// Never hardcode separators
let wrong = "config/app.toml";  // Won't work on Windows

// Use Path::new or PathBuf::from
let right = Path::new("config").join("app.toml");

// Get separator
std::path::MAIN_SEPARATOR      // '/' or '\\'
std::path::MAIN_SEPARATOR_STR  // "/" or "\\"
```

## Relative and Absolute Paths

```rust
use std::path::Path;
use std::env;

fn make_absolute(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .expect("No current dir")
            .join(path)
    }
}
```

## Path Display

```rust
let path = PathBuf::from("/home/user");

// For display (handles non-UTF8)
println!("{}", path.display());

// Try to convert to string
if let Some(s) = path.to_str() {
    println!("String: {}", s);
}

// Lossy conversion
let s = path.to_string_lossy();
```

## Temporary Paths

```rust
use std::env;

let tmp = env::temp_dir();
println!("Temp: {}", tmp.display());
// Linux: /tmp
// macOS: /var/folders/...
// Windows: C:\Users\...\AppData\Local\Temp
```
