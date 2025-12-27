//! # Cross-Platform Paths with directories
//!
//! This example shows how to handle paths across platforms.
//!
//! Run with: `cargo run --example 0801_platform_paths`

#![allow(dead_code)]

use directories::{BaseDirs, ProjectDirs, UserDirs};

fn main() {
    println!("=== Cross-Platform Paths ===\n");

    // =========================================================================
    // BASE DIRECTORIES
    // =========================================================================

    println!("--- Base Directories ---");
    if let Some(base_dirs) = BaseDirs::new() {
        println!("  Home:       {:?}", base_dirs.home_dir());
        println!("  Cache:      {:?}", base_dirs.cache_dir());
        println!("  Config:     {:?}", base_dirs.config_dir());
        println!("  Data:       {:?}", base_dirs.data_dir());
        println!("  Data Local: {:?}", base_dirs.data_local_dir());
        #[cfg(not(windows))]
        println!("  Executable: {:?}", base_dirs.executable_dir());
        println!("  Preference: {:?}", base_dirs.preference_dir());
        #[cfg(not(windows))]
        println!("  Runtime:    {:?}", base_dirs.runtime_dir());
    }

    println!();

    // =========================================================================
    // PROJECT DIRECTORIES
    // =========================================================================

    println!("--- Project Directories ---");
    if let Some(proj_dirs) = ProjectDirs::from("com", "example", "dx") {
        println!("  Config:     {:?}", proj_dirs.config_dir());
        println!("  Data:       {:?}", proj_dirs.data_dir());
        println!("  Data Local: {:?}", proj_dirs.data_local_dir());
        println!("  Cache:      {:?}", proj_dirs.cache_dir());
        println!("  Preference: {:?}", proj_dirs.preference_dir());
        #[cfg(not(windows))]
        println!("  Runtime:    {:?}", proj_dirs.runtime_dir());
    }

    println!();

    // =========================================================================
    // USER DIRECTORIES
    // =========================================================================

    println!("--- User Directories ---");
    if let Some(user_dirs) = UserDirs::new() {
        println!("  Home:      {:?}", user_dirs.home_dir());
        println!("  Audio:     {:?}", user_dirs.audio_dir());
        println!("  Desktop:   {:?}", user_dirs.desktop_dir());
        println!("  Documents: {:?}", user_dirs.document_dir());
        println!("  Downloads: {:?}", user_dirs.download_dir());
        println!("  Pictures:  {:?}", user_dirs.picture_dir());
        println!("  Videos:    {:?}", user_dirs.video_dir());
    }

    println!();

    // =========================================================================
    // PLATFORM DIFFERENCES
    // =========================================================================

    println!("--- Platform Differences ---");
    println!(
        r#"
Project directories by platform:

Linux:
  Config: ~/.config/dx/
  Data:   ~/.local/share/dx/
  Cache:  ~/.cache/dx/

macOS:
  Config: ~/Library/Application Support/com.example.dx/
  Data:   ~/Library/Application Support/com.example.dx/
  Cache:  ~/Library/Caches/com.example.dx/

Windows:
  Config: C:\Users\X\AppData\Roaming\example\dx\config\
  Data:   C:\Users\X\AppData\Roaming\example\dx\data\
  Cache:  C:\Users\X\AppData\Local\example\dx\cache\
"#
    );

    println!();

    // =========================================================================
    // PATH BUILDING
    // =========================================================================

    println!("--- Path Building ---");
    println!(
        r#"
Build paths correctly:

use std::path::{{Path, PathBuf}};

// Good: Use PathBuf::join()
let config_path = config_dir.join("settings.toml");
let nested = base.join("subdir").join("file.txt");

// Good: Use Path::new() for literals
let path = Path::new("relative/path/file.txt");

// Bad: String concatenation
// let path = format!("{{}}/config.toml", dir);  // Wrong separator on Windows!

// Handle separators
#[cfg(windows)]
const PATH_SEP: char = '\\';
#[cfg(not(windows))]
const PATH_SEP: char = '/';
"#
    );

    // Demo path building
    let config_dir = std::path::Path::new("/example/config");
    let settings = config_dir.join("settings.toml");
    println!("  Example: {:?}", settings);

    println!();

    // =========================================================================
    // ENVIRONMENT OVERRIDES
    // =========================================================================

    println!("--- Environment Overrides ---");
    println!(
        r#"
Allow environment variable overrides:

use std::path::PathBuf;

fn get_config_dir() -> PathBuf {{
    // 1. Check environment variable first
    if let Ok(dir) = std::env::var("DX_CONFIG_DIR") {{
        return PathBuf::from(dir);
    }}

    // 2. Check XDG on Linux
    #[cfg(target_os = "linux")]
    if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {{
        return PathBuf::from(dir).join("dx");
    }}

    // 3. Use directories crate as fallback
    ProjectDirs::from("com", "example", "dx")
        .map(|d| d.config_dir().to_path_buf())
        .unwrap_or_else(|| {{
            // 4. Ultimate fallback: current directory
            PathBuf::from(".")
        }})
}}

Common environment variables:
  Linux:   XDG_CONFIG_HOME, XDG_DATA_HOME, XDG_CACHE_HOME
  macOS:   HOME
  Windows: APPDATA, LOCALAPPDATA, USERPROFILE
"#
    );

    println!();

    // =========================================================================
    // FILE OPERATIONS
    // =========================================================================

    println!("--- Cross-Platform File Operations ---");
    println!(
        r#"
Safe file operations:

use std::fs;

// Create directory with all parents
fs::create_dir_all(&config_dir)?;

// Write atomically (write to temp, then rename)
use tempfile::NamedTempFile;
let temp = NamedTempFile::new_in(config_dir.parent().unwrap())?;
write!(temp.as_file(), "content")?;
temp.persist(&config_path)?;

// Read with proper error context
use anyhow::Context;
let content = fs::read_to_string(&path)
    .with_context(|| format!("Failed to read {{}}", path.display()))?;

// Check existence
if path.exists() && path.is_file() {{
    // ...
}}

// Get metadata
let metadata = fs::metadata(&path)?;
println!("Size: {{}} bytes", metadata.len());
println!("Modified: {{:?}}", metadata.modified()?);
"#
    );

    println!();

    // =========================================================================
    // PATH DISPLAY
    // =========================================================================

    println!("--- Path Display ---");
    println!(
        r#"
Display paths correctly:

use std::path::Path;

let path = Path::new("/some/path/file.txt");

// For display (handles non-UTF8)
println!("Path: {{}}", path.display());

// For logging
println!("Path: {{:?}}", path);

// Convert to string (may fail on non-UTF8)
if let Some(s) = path.to_str() {{
    println!("As string: {{}}", s);
}}

// Convert lossy (replaces invalid UTF8)
let s = path.to_string_lossy();
println!("Lossy: {{}}", s);
"#
    );

    // Demo
    let path = std::path::Path::new("/example/path/file.txt");
    println!("  Display: {}", path.display());
    println!("  Debug:   {:?}", path);

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!();
    println!("=== Summary ===");
    println!("Cross-platform paths:");
    println!("  1. Use directories crate for standard paths");
    println!("  2. Use PathBuf::join() for building paths");
    println!("  3. Allow environment variable overrides");
    println!("  4. Use .display() for output");
    println!("  5. Handle non-UTF8 paths gracefully");
}
