//! # Platform-Specific Features
//!
//! This example shows how to handle platform differences.
//!
//! Run with: `cargo run --example 0802_platform_features`

#![allow(dead_code)]

fn main() {
    println!("=== Platform-Specific Features ===\n");

    // =========================================================================
    // COMPILE-TIME DETECTION
    // =========================================================================

    println!("--- Compile-Time Detection ---");

    // cfg! macro for runtime branching
    let os_name = if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else {
        "Unknown"
    };
    println!("  Current OS: {}", os_name);

    // std::env::consts
    println!("  OS: {}", std::env::consts::OS);
    println!("  Arch: {}", std::env::consts::ARCH);
    println!("  Family: {}", std::env::consts::FAMILY);
    println!("  EXE suffix: {:?}", std::env::consts::EXE_SUFFIX);
    println!("  DLL suffix: {:?}", std::env::consts::DLL_SUFFIX);

    println!();

    // =========================================================================
    // CONDITIONAL COMPILATION
    // =========================================================================

    println!("--- Conditional Compilation ---");
    println!(
        r#"
Use #[cfg] for platform-specific code:

// Platform-specific modules
#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

// Platform-specific functions
#[cfg(unix)]
pub fn get_shell() -> &'static str {{
    std::env::var("SHELL").unwrap_or("/bin/sh".to_string())
}}

#[cfg(windows)]
pub fn get_shell() -> &'static str {{
    "cmd.exe"
}}

// Platform-specific implementations
impl Config {{
    #[cfg(unix)]
    pub fn default_editor(&self) -> &str {{
        "vim"
    }}

    #[cfg(windows)]
    pub fn default_editor(&self) -> &str {{
        "notepad"
    }}
}}

// Conditional dependencies
#[cfg(unix)]
use nix::unistd;

#[cfg(windows)]
use windows_sys::Win32;
"#
    );

    println!();

    // =========================================================================
    // PLATFORM SPECIFIC FEATURES
    // =========================================================================

    println!("--- Platform Features ---");
    println!(
        r#"
Common platform-specific features:

UNIX-ONLY:
  - File permissions (chmod)
  - Symlinks (always supported)
  - Signals (SIGTERM, SIGINT, etc.)
  - TTY detection
  - /dev/null

WINDOWS-ONLY:
  - Registry access
  - Windows paths (\\?\, UNC)
  - Console API
  - .exe extension
  - NUL device

Example:

#[cfg(unix)]
fn set_executable(path: &Path) -> std::io::Result<()> {{
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)
}}

#[cfg(windows)]
fn set_executable(_path: &Path) -> std::io::Result<()> {{
    // No-op on Windows (executability determined by extension)
    Ok(())
}}
"#
    );

    println!();

    // =========================================================================
    // TERMINAL HANDLING
    // =========================================================================

    println!("--- Terminal Handling ---");
    println!(
        r#"
Detect terminal capabilities:

// Check if stdin/stdout is a TTY
fn is_interactive() -> bool {{
    atty::is(atty::Stream::Stdout)
}}

// Or use std::io
fn is_tty() -> bool {{
    std::io::stdout().is_terminal()  // Rust 1.70+
}}

// Conditional output
if is_interactive() {{
    // Use colors, progress bars, etc.
    println!("{{}}", "Colored".green());
}} else {{
    // Plain output for pipes/redirects
    println!("Plain");
}}

// Terminal size
#[cfg(unix)]
fn get_terminal_width() -> Option<u16> {{
    use libc::{{ioctl, winsize, TIOCGWINSZ}};
    // ... ioctl call ...
}}

// Or use crossterm (cross-platform)
use crossterm::terminal;
let (width, height) = terminal::size().unwrap_or((80, 24));
"#
    );

    // Demo TTY detection
    let is_tty = std::io::IsTerminal::is_terminal(&std::io::stdout());
    println!("  stdout is TTY: {}", is_tty);

    println!();

    // =========================================================================
    // SIGNAL HANDLING
    // =========================================================================

    println!("--- Signal Handling ---");
    println!(
        r#"
Cross-platform signal handling with ctrlc:

use ctrlc;

fn setup_signal_handler() {{
    ctrlc::set_handler(move || {{
        println!("Received Ctrl+C, shutting down...");
        std::process::exit(0);
    }}).expect("Error setting Ctrl-C handler");
}}

// For more signals (Unix only):
#[cfg(unix)]
fn setup_unix_signals() {{
    use signal_hook::{{consts::*, iterator::Signals}};

    let mut signals = Signals::new(&[SIGTERM, SIGINT, SIGHUP]).unwrap();

    std::thread::spawn(move || {{
        for sig in signals.forever() {{
            match sig {{
                SIGTERM | SIGINT => {{
                    // Graceful shutdown
                    break;
                }}
                SIGHUP => {{
                    // Reload config
                }}
                _ => {{}}
            }}
        }}
    }});
}}
"#
    );

    println!();

    // =========================================================================
    // ENVIRONMENT VARIABLES
    // =========================================================================

    println!("--- Environment Variables ---");
    println!(
        r#"
Platform-specific environment:

// Home directory
#[cfg(unix)]
let home = std::env::var("HOME");

#[cfg(windows)]
let home = std::env::var("USERPROFILE");

// Or use directories crate (cross-platform)
use directories::BaseDirs;
let home = BaseDirs::new().map(|d| d.home_dir().to_path_buf());

// Path separator
#[cfg(unix)]
const PATH_VAR: &str = "PATH";
#[cfg(windows)]
const PATH_VAR: &str = "Path";  // Case-insensitive on Windows

#[cfg(unix)]
const PATH_SEP: char = ':';
#[cfg(windows)]
const PATH_SEP: char = ';';

// Standard streams
#[cfg(unix)]
const NULL_DEVICE: &str = "/dev/null";
#[cfg(windows)]
const NULL_DEVICE: &str = "NUL";
"#
    );

    println!();

    // =========================================================================
    // FEATURE FLAGS
    // =========================================================================

    println!("--- Feature Flags ---");
    println!(
        r#"
Use Cargo features for optional platform code:

# Cargo.toml
[features]
default = []
unix-extras = []
windows-extras = []

[target.'cfg(unix)'.dependencies]
nix = {{ version = "0.27", optional = true }}

[target.'cfg(windows)'.dependencies]
windows-sys = {{ version = "0.48", optional = true }}

# In code
#[cfg(all(unix, feature = "unix-extras"))]
mod advanced_unix {{
    use nix::unistd;
    // ...
}}

// Build with features
// cargo build --features unix-extras
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Platform-specific features:");
    println!("  1. Use cfg! and #[cfg] for conditionals");
    println!("  2. Check std::env::consts for platform info");
    println!("  3. Handle TTY detection for interactive features");
    println!("  4. Use cross-platform crates when possible");
    println!("  5. Test on all target platforms in CI");
}
