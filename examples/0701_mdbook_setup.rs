//! # mdbook Setup
//!
//! This example shows how to set up mdbook for CLI documentation.
//!
//! Run with: `cargo run --example 0701_mdbook_setup`

#![allow(dead_code)]

fn main() {
    println!("=== mdbook Setup ===\n");

    // =========================================================================
    // INSTALLATION
    // =========================================================================

    println!("--- Installation ---");
    println!(
        r#"
Install mdbook:

  cargo install mdbook

Optional preprocessors:
  cargo install mdbook-toc       # Table of contents
  cargo install mdbook-mermaid   # Diagrams
  cargo install mdbook-linkcheck # Check broken links
"#
    );

    println!();

    // =========================================================================
    // PROJECT STRUCTURE
    // =========================================================================

    println!("--- Project Structure ---");
    println!(
        r#"
Initialize in your project:

  cd my-cli-project
  mdbook init docs

Creates:
  docs/
  ├── book.toml          # Configuration
  └── src/
      ├── SUMMARY.md     # Table of contents
      └── chapter_1.md   # First chapter

Recommended structure for CLI docs:
  docs/
  ├── book.toml
  └── src/
      ├── SUMMARY.md
      ├── introduction.md
      ├── installation.md
      ├── getting-started.md
      ├── commands/
      │   ├── overview.md
      │   ├── hash.md
      │   ├── encode.md
      │   └── ...
      ├── configuration.md
      ├── development/
      │   ├── building.md
      │   ├── testing.md
      │   └── contributing.md
      └── faq.md
"#
    );

    println!();

    // =========================================================================
    // BOOK.TOML
    // =========================================================================

    println!("--- book.toml Configuration ---");
    println!(
        r##"
# docs/book.toml

[book]
title = "dx - Developer Toolkit"
authors = ["Your Name"]
description = "A comprehensive developer toolkit CLI"
language = "en"
src = "src"

[build]
build-dir = "book"            # Output directory
create-missing = true         # Create missing files from SUMMARY

[preprocessor.toc]
command = "mdbook-toc"
renderer = ["html"]

[preprocessor.mermaid]
command = "mdbook-mermaid"

[output.html]
default-theme = "rust"        # rust, coal, light, navy, ayu
preferred-dark-theme = "coal"
git-repository-url = "https://github.com/user/dx"
edit-url-template = "https://github.com/user/dx/edit/main/docs/{{{{path}}}}"
site-url = "/dx/"
no-section-label = false
additional-css = ["custom.css"]
additional-js = ["custom.js"]

[output.html.fold]
enable = true
level = 1

[output.html.playground]
editable = false
copyable = true
copy-js = true
line-numbers = true

[output.html.search]
enable = true
limit-results = 30
use-hierarchical = true
"##
    );

    println!();

    // =========================================================================
    // SUMMARY.MD
    // =========================================================================

    println!("--- SUMMARY.md ---");
    println!(
        r#"
# docs/src/SUMMARY.md

# Summary

[Introduction](./introduction.md)

# Getting Started

- [Installation](./installation.md)
- [Quick Start](./getting-started.md)
- [Configuration](./configuration.md)

# Commands

- [Overview](./commands/overview.md)
- [hash](./commands/hash.md)
- [encode](./commands/encode.md)
- [uuid](./commands/uuid.md)
- [time](./commands/time.md)
- [json](./commands/json.md)
- [env](./commands/env.md)
- [config](./commands/config.md)

# Development

- [Building from Source](./development/building.md)
- [Testing](./development/testing.md)
- [Cross-Compilation](./development/cross-compilation.md)
- [Contributing](./development/contributing.md)

# Reference

- [Exit Codes](./reference/exit-codes.md)
- [Environment Variables](./reference/environment.md)

---

[FAQ](./faq.md)
[Changelog](./changelog.md)
"#
    );

    println!();

    // =========================================================================
    // BUILDING AND SERVING
    // =========================================================================

    println!("--- Building and Serving ---");
    println!(
        r#"
Build the book:

  cd docs
  mdbook build

  # Output in docs/book/

Serve with live reload:

  mdbook serve
  # Open http://localhost:3000

Serve on different port:

  mdbook serve -p 8080

Watch for changes:

  mdbook watch

Clean build:

  mdbook clean
"#
    );

    println!();

    // =========================================================================
    // CONTENT FEATURES
    // =========================================================================

    println!("--- Content Features ---");
    println!(
        r##"
Markdown features in mdbook:

# Code blocks with syntax highlighting
```rust
fn main() {{
    println!("Hello!");
}}
```

# Hide lines in code (starts with #)
```rust
# fn main() {{
let x = 42;
# }}
```

# Include files
{{{{#include ../src/main.rs}}}}

# Include specific lines
{{{{#include ../src/main.rs:5:10}}}}

# Include with anchors
// In source file:
// ANCHOR: my_function
fn my_function() {{ }}
// ANCHOR_END: my_function

// In markdown:
{{{{#include ../src/lib.rs:my_function}}}}

# Runnable Rust code (playground)
```rust,editable
fn main() {{
    println!("Try editing me!");
}}
```

# Hide code in playground
```rust,editable
# #![allow(unused)]
fn main() {{
    let visible = true;
}}
```
"##
    );

    println!();

    // =========================================================================
    // LINKING
    // =========================================================================

    println!("--- Linking ---");
    println!(
        r#"
Link syntax:

# Relative links
[Next Chapter](./next.md)
[Subfolder](./commands/hash.md)
[Parent](../index.md)

# Anchors within page
## My Section {{#my-section}}

[Link to section](#my-section)

# Cross-page anchors
[Hash command](./commands/hash.md#examples)

# External links
[GitHub](https://github.com/user/dx)

# Link checking
Run: mdbook-linkcheck
Catches broken internal and external links
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("mdbook setup:");
    println!("  1. cargo install mdbook");
    println!("  2. mdbook init docs");
    println!("  3. Edit book.toml and SUMMARY.md");
    println!("  4. mdbook serve for development");
    println!("  5. mdbook build for production");
}
