//! # Interactive Prompts with dialoguer
//!
//! This example shows how to use dialoguer for user input.
//!
//! Run with: `cargo run --example 0403_interactive_prompts`

#![allow(dead_code)]

use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Password, Select};

fn main() {
    println!("=== Interactive Prompts with dialoguer ===\n");

    // =========================================================================
    // CONFIRMATION
    // =========================================================================

    println!("--- Confirm ---");
    println!(
        r#"
let confirmed = Confirm::new()
    .with_prompt("Do you want to continue?")
    .default(true)
    .interact()?;

if confirmed {{
    println!("Proceeding...");
}}
"#
    );

    // Demo (non-interactive for example)
    println!("  [Would show: Do you want to continue? (Y/n)]");

    println!();

    // =========================================================================
    // TEXT INPUT
    // =========================================================================

    println!("--- Text Input ---");
    println!(
        r#"
let name: String = Input::new()
    .with_prompt("Your name")
    .default("Anonymous".to_string())
    .interact_text()?;

// With validation
let port: u16 = Input::new()
    .with_prompt("Port number")
    .validate_with(|input: &String| {{
        input.parse::<u16>()
            .map(|_| ())
            .map_err(|_| "Invalid port number")
    }})
    .interact_text()?;
"#
    );

    println!("  [Would show: Your name: _]");

    println!();

    // =========================================================================
    // PASSWORD
    // =========================================================================

    println!("--- Password Input ---");
    println!(
        r#"
let password = Password::new()
    .with_prompt("Enter password")
    .interact()?;

// With confirmation
let password = Password::new()
    .with_prompt("New password")
    .with_confirmation("Confirm password", "Passwords don't match")
    .interact()?;
"#
    );

    println!("  [Would show: Enter password: ******]");

    println!();

    // =========================================================================
    // SELECT
    // =========================================================================

    println!("--- Select (single choice) ---");
    println!(
        r#"
let options = vec!["Option A", "Option B", "Option C"];

let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Choose an option")
    .items(&options)
    .default(0)
    .interact()?;

println!("You chose: {{}}", options[selection]);
"#
    );

    println!("  [Would show arrow-key selection menu]");

    println!();

    // =========================================================================
    // MULTI-SELECT
    // =========================================================================

    println!("--- MultiSelect (multiple choices) ---");
    println!(
        r#"
let features = vec!["Feature A", "Feature B", "Feature C", "Feature D"];

let selections = MultiSelect::with_theme(&ColorfulTheme::default())
    .with_prompt("Select features to enable")
    .items(&features)
    .defaults(&[true, false, true, false])  // Pre-selected
    .interact()?;

for i in selections {{
    println!("Enabled: {{}}", features[i]);
}}
"#
    );

    println!("  [Would show checkbox selection menu]");

    println!();

    // =========================================================================
    // THEMES
    // =========================================================================

    println!("--- Themes ---");
    println!(
        r#"
Available themes:

  ColorfulTheme::default()  // Colored prompts
  SimpleTheme               // No colors

Custom theme:
  let theme = ColorfulTheme {{
      prompt_prefix: style("?".to_string()).yellow(),
      success_prefix: style("✓".to_string()).green(),
      ...
  }};
"#
    );

    println!();

    // =========================================================================
    // PATTERNS
    // =========================================================================

    println!("--- Common Patterns ---");
    println!(
        r#"
1. DANGEROUS OPERATION:
   let confirm = Confirm::new()
       .with_prompt("Delete all files?")
       .default(false)
       .interact()?;

   if !confirm {{
       println!("Cancelled");
       return Ok(());
   }}

2. INITIAL SETUP:
   let name = Input::<String>::new()
       .with_prompt("Project name")
       .interact_text()?;

   let template = Select::new()
       .with_prompt("Template")
       .items(&["minimal", "full", "custom"])
       .interact()?;

3. SKIP IN NON-INTERACTIVE:
   if atty::is(atty::Stream::Stdin) {{
       // Interactive prompts
   }} else {{
       // Use defaults or error
   }}
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Interactive prompt types:");
    println!("  1. Confirm - Yes/No questions");
    println!("  2. Input - Text input with validation");
    println!("  3. Password - Hidden input");
    println!("  4. Select - Single choice from list");
    println!("  5. MultiSelect - Multiple choices");
}
