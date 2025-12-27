//! # Colored Terminal Output
//!
//! This example shows how to use the colored crate for terminal output.
//!
//! Run with: `cargo run --example 0401_colored_output`

#![allow(dead_code)]

use colored::*;

fn main() {
    println!("=== Colored Terminal Output ===\n");

    // =========================================================================
    // BASIC COLORS
    // =========================================================================

    println!("--- Basic Colors ---");
    println!("  {}", "Red text".red());
    println!("  {}", "Green text".green());
    println!("  {}", "Blue text".blue());
    println!("  {}", "Yellow text".yellow());
    println!("  {}", "Cyan text".cyan());
    println!("  {}", "Magenta text".magenta());
    println!("  {}", "White text".white());

    println!();

    // =========================================================================
    // STYLES
    // =========================================================================

    println!("--- Styles ---");
    println!("  {}", "Bold text".bold());
    println!("  {}", "Italic text".italic());
    println!("  {}", "Underline text".underline());
    println!("  {}", "Dimmed text".dimmed());
    println!("  {}", "Strikethrough".strikethrough());

    println!();

    // =========================================================================
    // COMBINATIONS
    // =========================================================================

    println!("--- Combinations ---");
    println!("  {}", "Bold red".red().bold());
    println!("  {}", "Bold green underline".green().bold().underline());
    println!("  {}", "Yellow on blue".yellow().on_blue());
    println!("  {}", "White on red".white().on_red());

    println!();

    // =========================================================================
    // CLI OUTPUT PATTERNS
    // =========================================================================

    println!("--- CLI Output Patterns ---");

    // Success
    println!("  {} Operation completed", "✓".green().bold());

    // Warning
    println!("  {} Deprecated feature used", "⚠".yellow().bold());

    // Error
    println!("  {} File not found", "✗".red().bold());

    // Info
    println!("  {} Processing...", "→".cyan());

    // Key-value
    println!("  {}: {}", "Version".cyan(), "1.0.0");
    println!("  {}: {}", "Status".cyan(), "OK".green());

    println!();

    // =========================================================================
    // CONDITIONAL COLORING
    // =========================================================================

    println!("--- Conditional Coloring ---");

    fn status_color(status: &str) -> ColoredString {
        match status {
            "success" => status.green(),
            "warning" => status.yellow(),
            "error" => status.red(),
            _ => status.normal(),
        }
    }

    println!("  Status: {}", status_color("success"));
    println!("  Status: {}", status_color("warning"));
    println!("  Status: {}", status_color("error"));

    println!();

    // =========================================================================
    // RESPECTING NO_COLOR
    // =========================================================================

    println!("--- Respecting NO_COLOR ---");
    println!(
        r#"
The colored crate respects the NO_COLOR env var:

  NO_COLOR=1 dx hash file.txt

You can also control it programmatically:

  // Disable colors
  colored::control::set_override(false);

  // Enable colors
  colored::control::set_override(true);

  // Check if colors are enabled
  if colored::control::SHOULD_COLORIZE.should_colorize() {{
      // Use colors
  }}

In CLI args:
  #[arg(long, global = true, env = "NO_COLOR")]
  no_color: bool,

  if cli.no_color {{
      colored::control::set_override(false);
  }}
"#
    );

    println!();

    // =========================================================================
    // OUTPUT HELPERS
    // =========================================================================

    println!("--- Output Helper Functions ---");
    println!(
        r#"
Create helper functions for consistent output:

pub fn print_success(msg: &str) {{
    println!("{{}} {{}}", "✓".green().bold(), msg);
}}

pub fn print_error(msg: &str) {{
    eprintln!("{{}} {{}}", "✗".red().bold(), msg);
}}

pub fn print_warning(msg: &str) {{
    eprintln!("{{}} {{}}", "⚠".yellow().bold(), msg);
}}

pub fn print_info(msg: &str) {{
    println!("{{}} {{}}", "→".cyan(), msg);
}}

pub fn print_key_value(key: &str, value: &str) {{
    println!("{{}}: {{}}", key.cyan(), value);
}}
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Colored output features:");
    println!("  1. Basic colors: red, green, blue, yellow, etc.");
    println!("  2. Styles: bold, italic, underline, dimmed");
    println!("  3. Backgrounds: on_red, on_blue, etc.");
    println!("  4. Chainable: \"text\".red().bold()");
    println!("  5. Respects NO_COLOR environment variable");
}
