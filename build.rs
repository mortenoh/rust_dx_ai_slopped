//! # Cargo Build Script (build.rs)
//!
//! This file is a **build script** that Cargo executes **before** compiling
//! your crate. It runs as a separate binary and can perform compile-time tasks.
//!
//! ## When Does build.rs Run?
//!
//! Cargo runs build.rs when:
//! 1. The build script itself changes
//! 2. Files specified in `cargo:rerun-if-changed` change
//! 3. Environment variables in `cargo:rerun-if-env-changed` change
//! 4. First build or `cargo clean` was run
//!
//! ## What Can build.rs Do?
//!
//! | Capability | Example |
//! |------------|---------|
//! | Generate code | Shell completions, bindings |
//! | Set env vars | `cargo:rustc-env=VAR=value` |
//! | Link libraries | `cargo:rustc-link-lib=ssl` |
//! | Add search paths | `cargo:rustc-link-search=/path` |
//! | Compile C/C++ | Via the `cc` crate |
//! | Emit warnings | `cargo:warning=message` |
//!
//! ## cargo: Directives
//!
//! Build scripts communicate with Cargo by printing special lines to stdout:
//!
//! ```text
//! cargo:rustc-env=NAME=VALUE     - Set environment variable for rustc
//! cargo:rustc-cfg=feature        - Enable a cfg flag
//! cargo:rerun-if-changed=PATH    - Rebuild if this file changes
//! cargo:rerun-if-env-changed=VAR - Rebuild if this env var changes
//! cargo:warning=MESSAGE          - Print a warning during build
//! ```
//!
//! ## External Documentation
//! - Build Scripts: <https://doc.rust-lang.org/cargo/reference/build-scripts.html>
//! - clap_complete: <https://docs.rs/clap_complete>
//! - clap_mangen: <https://docs.rs/clap_mangen>

use std::env;
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

// =============================================================================
// PROTOCOL BUFFER COMPILATION
// =============================================================================
//
// tonic_build compiles .proto files into Rust code for gRPC.
// The generated code is placed in OUT_DIR and included via tonic::include_proto!

fn compile_protos() -> Result<(), Box<dyn std::error::Error>> {
    // Only compile if the proto file exists
    let proto_path = "proto/chat.proto";
    if Path::new(proto_path).exists() {
        tonic_prost_build::compile_protos(proto_path)?;
        println!("cargo:rerun-if-changed={}", proto_path);
    }
    Ok(())
}

// =============================================================================
// CLAP IMPORTS FOR COMPLETIONS AND MAN PAGES
// =============================================================================
//
// Note: build.rs has its own dependencies specified in [build-dependencies]
// in Cargo.toml. These are separate from [dependencies].

use clap::{Arg, Command as ClapCommand, ValueHint};
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;

fn main() {
    // =========================================================================
    // STEP 0: COMPILE PROTOCOL BUFFERS
    // =========================================================================
    //
    // Compile .proto files to Rust before the rest of the build.
    // This generates gRPC service traits and message types.

    if let Err(e) = compile_protos() {
        println!("cargo:warning=Failed to compile protos: {}", e);
    }

    // =========================================================================
    // STEP 1: DETERMINE OUTPUT DIRECTORY
    // =========================================================================
    //
    // OUT_DIR is set by Cargo and points to a build-specific directory:
    // target/<profile>/build/<crate>-<hash>/out/
    //
    // This is where build scripts should write generated files.
    // It's unique per build configuration (debug/release, features, etc.)

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // For easier access, we also create directories in target/ directly
    // These are more convenient for users to find and copy
    let target_dir = out_dir
        .ancestors() // Walk up the directory tree
        .nth(3) // Go up 3 levels: out/ -> <hash>/ -> build/ -> <profile>/
        .expect("Could not find target directory")
        .to_path_buf();

    // =========================================================================
    // STEP 2: SET BUILD-TIME ENVIRONMENT VARIABLES
    // =========================================================================
    //
    // These become available in code via env!("VAR_NAME") macro.
    // Useful for embedding version info, git commit, build timestamp.

    set_build_metadata();

    // =========================================================================
    // STEP 3: BUILD CLI COMMAND FOR CODE GENERATION
    // =========================================================================
    //
    // To generate completions and man pages, we need the CLI command structure.
    //
    // IMPORTANT: We can't import from our own crate here (it's not built yet!)
    // So we reconstruct the CLI using clap's builder API.
    //
    // Alternative approaches:
    // 1. Use include!() to share code (complex with dependencies)
    // 2. Create a shared crate for CLI definitions (more crates)
    // 3. Manual reconstruction (what we do here - explicit and educational)

    let cmd = build_cli_command();

    // =========================================================================
    // STEP 4: GENERATE SHELL COMPLETIONS
    // =========================================================================
    //
    // Shell completions allow users to press <TAB> to autocomplete commands.
    // Each shell has its own completion format.

    if let Err(e) = generate_completions(&cmd, &target_dir) {
        // Use cargo:warning to show errors without failing the build
        // This makes completions optional - build succeeds even if generation fails
        println!("cargo:warning=Failed to generate completions: {}", e);
    }

    // =========================================================================
    // STEP 5: GENERATE MAN PAGES
    // =========================================================================
    //
    // Man pages are Unix documentation accessible via `man dx`.
    // They're written in roff format and installed to /usr/share/man/

    if let Err(e) = generate_man_pages(&cmd, &target_dir) {
        println!("cargo:warning=Failed to generate man pages: {}", e);
    }

    // =========================================================================
    // STEP 6: DECLARE REBUILD TRIGGERS
    // =========================================================================
    //
    // Tell Cargo when to re-run this build script.
    // By default, it runs when any file in the package changes.
    // Being explicit improves build times.

    // Rebuild if the build script itself changes
    println!("cargo:rerun-if-changed=build.rs");

    // Rebuild if CLI definitions change
    println!("cargo:rerun-if-changed=src/cli/args.rs");

    // Rebuild if git HEAD changes (for commit hash)
    println!("cargo:rerun-if-changed=.git/HEAD");

    // Rebuild if an environment variable changes
    println!("cargo:rerun-if-env-changed=DX_BUILD_EXTRA");
}

// =============================================================================
// BUILD METADATA
// =============================================================================
//
// Set environment variables that can be accessed at compile time via env!()
// This embeds build information directly into the binary.

fn set_build_metadata() {
    // -------------------------------------------------------------------------
    // Git Commit Hash
    // -------------------------------------------------------------------------
    //
    // Run `git rev-parse HEAD` to get the current commit.
    // If not in a git repo, fall back to "unknown".
    //
    // Usage in code: env!("DX_GIT_HASH") or option_env!("DX_GIT_HASH")

    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // cargo:rustc-env sets an environment variable for the Rust compiler
    // This is different from regular env vars - it's baked into the binary!
    println!("cargo:rustc-env=DX_GIT_HASH={}", git_hash);

    // -------------------------------------------------------------------------
    // Build Timestamp
    // -------------------------------------------------------------------------
    //
    // ISO 8601 formatted build time in UTC.
    // Note: This causes rebuilds every time since timestamp always changes.
    // For reproducible builds, consider using SOURCE_DATE_EPOCH instead.

    let build_time = chrono_lite_now();
    println!("cargo:rustc-env=DX_BUILD_TIME={}", build_time);

    // -------------------------------------------------------------------------
    // Target Triple
    // -------------------------------------------------------------------------
    //
    // The target platform we're building for.
    // Cargo sets this automatically: x86_64-apple-darwin, etc.

    if let Ok(target) = env::var("TARGET") {
        println!("cargo:rustc-env=DX_TARGET={}", target);
    }

    // -------------------------------------------------------------------------
    // Build Profile
    // -------------------------------------------------------------------------
    //
    // "debug" or "release" - useful for conditional behavior

    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-env=DX_PROFILE={}", profile);
    }
}

/// Get current UTC time in ISO 8601 format.
///
/// We implement this manually to avoid adding chrono as a build dependency.
/// For build scripts, fewer dependencies = faster builds.
fn chrono_lite_now() -> String {
    // On Unix, we can use the `date` command
    #[cfg(unix)]
    {
        Command::new("date")
            .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    // On Windows, use PowerShell
    #[cfg(windows)]
    {
        Command::new("powershell")
            .args(["-Command", "Get-Date -Format 'yyyy-MM-ddTHH:mm:ssZ'"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    #[cfg(not(any(unix, windows)))]
    {
        "unknown".to_string()
    }
}

// =============================================================================
// CLI COMMAND BUILDER
// =============================================================================
//
// Reconstruct the CLI structure using clap's builder API.
// This mirrors src/cli/args.rs but without needing to import it.

fn build_cli_command() -> ClapCommand {
    // The builder API constructs commands programmatically
    // Compare to the derive API used in args.rs: #[derive(Parser)]
    ClapCommand::new("dx")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Developer")
        .about("Developer Experience CLI - A toolkit for common developer tasks")
        // Global arguments apply to all subcommands
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .help("Disable colored output")
                .global(true)
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .global(true)
                .action(clap::ArgAction::SetTrue),
        )
        // Subcommands - each is a separate ClapCommand
        .subcommand(
            ClapCommand::new("hash")
                .visible_alias("h")
                .about("Compute file or string hashes (MD5, SHA256, SHA512)")
                .arg(
                    Arg::new("input")
                        .help("File to hash")
                        .value_hint(ValueHint::FilePath),
                ),
        )
        .subcommand(
            ClapCommand::new("encode")
                .visible_alias("e")
                .about("Encode or decode data (base64, hex)"),
        )
        .subcommand(
            ClapCommand::new("uuid")
                .visible_alias("u")
                .about("Generate UUIDs (v4, v7)"),
        )
        .subcommand(
            ClapCommand::new("time")
                .visible_alias("t")
                .about("Convert and format timestamps"),
        )
        .subcommand(
            ClapCommand::new("json")
                .visible_alias("j")
                .about("Format and validate JSON")
                .arg(
                    Arg::new("input")
                        .help("JSON file to process")
                        .value_hint(ValueHint::FilePath),
                ),
        )
        .subcommand(ClapCommand::new("env").about("Manage environment variables"))
        .subcommand(
            ClapCommand::new("config")
                .visible_alias("cfg")
                .about("Manage application configuration"),
        )
        .subcommand(
            ClapCommand::new("rand")
                .visible_alias("r")
                .about("Generate random data (numbers, strings, passwords)"),
        )
        .subcommand(
            ClapCommand::new("text").about("Transform text (case conversion, slugify, etc.)"),
        )
        .subcommand(
            ClapCommand::new("calc")
                .visible_alias("c")
                .about("Calculator and unit conversions"),
        )
        .subcommand(ClapCommand::new("net").about("Network utilities (IP, DNS, ports)"))
        .subcommand(
            ClapCommand::new("chat")
                .about("Real-time chat using gRPC")
                .subcommand(
                    ClapCommand::new("server").about("Start chat server").arg(
                        Arg::new("port")
                            .short('p')
                            .long("port")
                            .help("Port to listen on")
                            .default_value("50051"),
                    ),
                )
                .subcommand(
                    ClapCommand::new("client")
                        .about("Connect as chat client")
                        .arg(Arg::new("name").help("Your display name").required(true))
                        .arg(
                            Arg::new("server")
                                .short('s')
                                .long("server")
                                .help("Server address")
                                .default_value("http://[::1]:50051"),
                        ),
                ),
        )
}

// =============================================================================
// SHELL COMPLETION GENERATION
// =============================================================================
//
// Generate completion scripts for various shells.
// Users source these in their shell config for tab-completion.

fn generate_completions(cmd: &ClapCommand, target_dir: &Path) -> Result<(), Error> {
    // Create completions directory: target/completions/
    let completions_dir = target_dir.join("completions");
    fs::create_dir_all(&completions_dir)?;

    // Clone the command for each shell (generate_to consumes it)
    let mut cmd = cmd.clone();

    // -------------------------------------------------------------------------
    // Generate for each shell
    // -------------------------------------------------------------------------
    //
    // Shell::variants() returns all supported shells:
    // - Bash: Most common on Linux
    // - Zsh: Default on macOS, popular on Linux
    // - Fish: Modern shell with great defaults
    // - PowerShell: Windows and cross-platform
    // - Elvish: Expressive shell (less common)

    for shell in [
        Shell::Bash,
        Shell::Zsh,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Elvish,
    ] {
        // generate_to writes the completion script to a file
        // Returns the path to the generated file
        let path = generate_to(shell, &mut cmd, "dx", &completions_dir)?;

        // Emit a note about what was generated
        // These show up in `cargo build -vv` (very verbose)
        println!(
            "cargo:note=Generated {} completions: {}",
            shell,
            path.display()
        );
    }

    Ok(())
}

// =============================================================================
// MAN PAGE GENERATION
// =============================================================================
//
// Generate Unix manual pages in roff format.
// These can be installed to /usr/share/man/man1/ for `man dx` access.

fn generate_man_pages(cmd: &ClapCommand, target_dir: &Path) -> Result<(), Error> {
    // Create man directory: target/man/
    let man_dir = target_dir.join("man");
    fs::create_dir_all(&man_dir)?;

    // -------------------------------------------------------------------------
    // Generate main command man page
    // -------------------------------------------------------------------------
    //
    // Man pages have sections (1 = user commands, 8 = admin commands, etc.)
    // CLI tools typically go in section 1.

    let man = Man::new(cmd.clone());
    let mut buffer = Vec::new();

    // Render the man page to our buffer
    man.render(&mut buffer)?;

    // Write to dx.1 (the .1 indicates section 1)
    let man_path = man_dir.join("dx.1");
    fs::write(&man_path, buffer)?;

    println!("cargo:note=Generated man page: {}", man_path.display());

    // -------------------------------------------------------------------------
    // Generate subcommand man pages
    // -------------------------------------------------------------------------
    //
    // Each subcommand can have its own man page: dx-hash.1, dx-encode.1, etc.

    for subcommand in cmd.get_subcommands() {
        let name = subcommand.get_name();

        // Skip internal/hidden subcommands
        if subcommand.is_hide_set() {
            continue;
        }

        let mut buffer = Vec::new();
        let sub_man = Man::new(subcommand.clone());
        sub_man.render(&mut buffer)?;

        // Convention: parent-subcommand.1
        let sub_path = man_dir.join(format!("dx-{}.1", name));
        fs::write(&sub_path, buffer)?;

        println!("cargo:note=Generated man page: {}", sub_path.display());
    }

    Ok(())
}

// =============================================================================
// ADDITIONAL EXAMPLES (COMMENTED OUT)
// =============================================================================
//
// Here are other common build.rs patterns for reference:

/*
// Example: Link to a system library
// This tells the linker to link against libssl
fn link_openssl() {
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
    // On some systems you need to specify the search path:
    // println!("cargo:rustc-link-search=/usr/local/lib");
}

// Example: Compile C code using the `cc` crate
// Requires: cc = "1" in [build-dependencies]
fn compile_c_code() {
    cc::Build::new()
        .file("src/helper.c")
        .compile("helper");
}

// Example: Generate code from a schema
// Common for protobuf, flatbuffers, etc.
fn generate_from_schema() {
    let schema = std::fs::read_to_string("schema.proto").unwrap();
    let generated = process_schema(&schema);
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("generated.rs");
    std::fs::write(out_path, generated).unwrap();
    // In your code, use: include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

// Example: Conditional compilation based on features
fn set_feature_flags() {
    // You can emit cfg flags that code can check
    if std::env::var("CARGO_FEATURE_EXPERIMENTAL").is_ok() {
        println!("cargo:rustc-cfg=experimental");
    }
    // In code: #[cfg(experimental)] fn experimental_feature() { ... }
}

// Example: Fail the build with an error
fn require_something() {
    if !Path::new("/required/file").exists() {
        // Print error and exit with non-zero status
        eprintln!("error: /required/file is missing!");
        std::process::exit(1);
    }
}
*/
