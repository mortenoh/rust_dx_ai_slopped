//! # Man Page Generation with clap_mangen
//!
//! This example shows how to generate Unix man pages.
//!
//! Run with: `cargo run --example 0704_man_pages`

#![allow(dead_code)]

fn main() {
    println!("=== Man Page Generation ===\n");

    // =========================================================================
    // SETUP
    // =========================================================================

    println!("--- Setup ---");
    println!(
        r#"
Add clap_mangen to build-dependencies:

[build-dependencies]
clap = {{ version = "4", features = ["derive"] }}
clap_mangen = "0.2"

Create build.rs:

use clap::CommandFactory;
use clap_mangen::Man;
use std::fs;
use std::path::PathBuf;

// Include CLI definition
include!("src/cli/args.rs");

fn main() {{
    let out_dir = PathBuf::from(
        std::env::var_os("OUT_DIR").unwrap_or("target".into())
    ).join("man");

    fs::create_dir_all(&out_dir).unwrap();

    let cmd = Cli::command();
    generate_manpages(&cmd, &out_dir);
}}

fn generate_manpages(cmd: &clap::Command, out_dir: &PathBuf) {{
    let man = Man::new(cmd.clone());
    let mut buffer = Vec::new();
    man.render(&mut buffer).unwrap();

    let name = cmd.get_name();
    let path = out_dir.join(format!("{{}}.1", name));
    fs::write(&path, buffer).unwrap();

    // Generate for subcommands
    for sub in cmd.get_subcommands() {{
        if sub.get_name() == "help" {{
            continue;
        }}
        let sub_name = format!("{{}}-{{}}", name, sub.get_name());
        let man = Man::new(sub.clone().name(&sub_name));
        let mut buffer = Vec::new();
        man.render(&mut buffer).unwrap();
        fs::write(out_dir.join(format!("{{}}.1", sub_name)), buffer).unwrap();
    }}
}}
"#
    );

    println!();

    // =========================================================================
    // MAN PAGE STRUCTURE
    // =========================================================================

    println!("--- Man Page Structure ---");
    println!(
        r#"
Generated man page sections:

DX(1)                     General Commands Manual                    DX(1)

NAME
       dx - Developer toolkit CLI

SYNOPSIS
       dx [OPTIONS] <COMMAND>

DESCRIPTION
       A comprehensive developer toolkit for common tasks.

OPTIONS
       -h, --help
              Print help information

       -V, --version
              Print version information

       --no-color
              Disable colored output

COMMANDS
       hash   Calculate file hashes
       encode Base64/hex encoding
       uuid   Generate UUIDs

AUTHOR
       Written by Your Name.

REPORTING BUGS
       Report bugs at https://github.com/user/dx/issues

COPYRIGHT
       Copyright (c) 2024 Your Name. License MIT.

SEE ALSO
       sha256sum(1), base64(1), uuidgen(1)
"#
    );

    println!();

    // =========================================================================
    // ENHANCED DESCRIPTIONS
    // =========================================================================

    println!("--- Enhanced Descriptions ---");
    println!(
        r#"
Add rich descriptions in clap for better man pages:

/// The main CLI
#[derive(Parser)]
#[command(
    name = "dx",
    author = "Your Name <you@example.com>",
    version,
    about = "Developer toolkit CLI",
    long_about = "A comprehensive developer toolkit for common tasks.\n\n\
        dx provides utilities for hashing, encoding, UUID generation,\n\
        timestamp conversion, and more.",
    after_help = "EXAMPLES:\n    \
        dx hash myfile.txt\n    \
        dx encode base64 'hello world'\n    \
        dx uuid --type v7",
    after_long_help = "EXAMPLES:\n\n    \
        Hash a file:\n        \
        $ dx hash myfile.txt\n\n    \
        Encode to base64:\n        \
        $ dx encode base64 'hello world'\n\n    \
        Generate UUID v7:\n        \
        $ dx uuid --type v7\n\n\
        ENVIRONMENT:\n    \
        DX_CONFIG_DIR - Override config directory\n    \
        NO_COLOR - Disable colored output",
)]
pub struct Cli {{ ... }}

/// Hash subcommand
#[derive(Args)]
#[command(
    about = "Calculate file hashes",
    long_about = "Calculate cryptographic hashes of files or stdin.\n\n\
        Supports MD5, SHA-256, and SHA-512 algorithms.",
    after_help = "See 'dx help hash' for examples.",
)]
pub struct HashArgs {{ ... }}
"#
    );

    println!();

    // =========================================================================
    // INSTALLATION
    // =========================================================================

    println!("--- Installing Man Pages ---");
    println!(
        r#"
Install man pages:

# Build to generate man pages
cargo build --release

# Man pages are in target/release/build/dx-*/out/man/

# Install manually
sudo mkdir -p /usr/local/share/man/man1
sudo cp target/release/build/dx-*/out/man/*.1 /usr/local/share/man/man1/
sudo mandb  # Update man database (Linux)

# Or in Makefile:
install: build
    install -d $(DESTDIR)/usr/local/bin
    install -m 755 target/release/dx $(DESTDIR)/usr/local/bin/
    install -d $(DESTDIR)/usr/local/share/man/man1
    install -m 644 target/release/build/dx-*/out/man/*.1 \
        $(DESTDIR)/usr/local/share/man/man1/

# View man page
man dx
man dx-hash
"#
    );

    println!();

    // =========================================================================
    // CUSTOM SECTIONS
    // =========================================================================

    println!("--- Custom Man Page Sections ---");
    println!(
        r#"
Add custom sections to man pages:

use clap_mangen::Man;

fn generate_with_sections(cmd: &clap::Command) -> Vec<u8> {{
    let man = Man::new(cmd.clone())
        .section("ENVIRONMENT")
        .section("FILES")
        .section("EXIT STATUS")
        .section("EXAMPLES")
        .section("SEE ALSO");

    let mut buffer = Vec::new();
    man.render(&mut buffer).unwrap();
    buffer
}}

// Or manually append to the roff output:
fn append_section(buffer: &mut Vec<u8>, title: &str, content: &str) {{
    buffer.extend_from_slice(format!(
        ".SH {{}}\n{{}}\n",
        title.to_uppercase(),
        content
    ).as_bytes());
}}

let mut man_content = generate_man(cmd);
append_section(&mut man_content, "EXIT STATUS", "\
.TP
.B 0
Success
.TP
.B 1
General error
.TP
.B 2
Usage error");
"#
    );

    println!();

    // =========================================================================
    // DISTRIBUTION
    // =========================================================================

    println!("--- Including in Distribution ---");
    println!(
        r#"
Include man pages in releases:

# In release workflow
- name: Build
  run: cargo build --release

- name: Package man pages
  run: |
    mkdir -p dist/man
    cp target/release/build/dx-*/out/man/*.1 dist/man/

- name: Create tarball
  run: |
    tar -czvf dx-linux-x64.tar.gz \
      target/release/dx \
      dist/man/*.1 \
      README.md \
      LICENSE

# Homebrew formula:
class Dx < Formula
  # ...
  def install
    bin.install "dx"
    man1.install Dir["man/*.1"]
  end
end

# Debian package:
# debian/dx.manpages
debian/dx.1
debian/dx-hash.1
debian/dx-encode.1
"#
    );

    println!();

    // =========================================================================
    // TESTING MAN PAGES
    // =========================================================================

    println!("--- Testing Man Pages ---");
    println!(
        r#"
Verify man pages are correct:

# Check syntax
mandoc -T lint dx.1

# Preview without installing
man ./dx.1

# Convert to text
man ./dx.1 | col -b > dx.txt

# Convert to HTML
mandoc -T html dx.1 > dx.html
# or
groff -man -Thtml dx.1 > dx.html

# Test in CI
- name: Verify man pages
  run: |
    for f in target/release/build/dx-*/out/man/*.1; do
      mandoc -T lint "$f" || exit 1
    done
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Man page generation:");
    println!("  1. Add clap_mangen to build-dependencies");
    println!("  2. Generate in build.rs");
    println!("  3. Add rich descriptions to clap commands");
    println!("  4. Install to /usr/share/man/man1/");
    println!("  5. Include in release packages");
}
