//! # Text Transformation Command
//!
//! Transform text between different cases and formats.
//!
//! ## Examples
//! ```bash
//! dx text upper "hello world"     # HELLO WORLD
//! dx text snake "HelloWorld"      # hello_world
//! dx text slug "Hello World!"     # hello-world
//! dx text lorem 3                 # 3 paragraphs of lorem ipsum
//! echo "hello" | dx text upper    # HELLO (from stdin)
//! ```

use crate::cli::commands::text::{TextArgs, TextCommand};
use anyhow::Result;
use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
};
use std::io::{self, Read};

pub fn run(args: TextArgs) -> Result<()> {
    match args.command {
        TextCommand::Upper { text } => cmd_transform(text, |s| s.to_uppercase()),
        TextCommand::Lower { text } => cmd_transform(text, |s| s.to_lowercase()),
        TextCommand::Title { text } => cmd_transform(text, |s| s.to_title_case()),
        TextCommand::Snake { text } => cmd_transform(text, |s| s.to_snake_case()),
        TextCommand::Camel { text } => cmd_transform(text, |s| s.to_lower_camel_case()),
        TextCommand::Pascal { text } => cmd_transform(text, |s| s.to_pascal_case()),
        TextCommand::Kebab { text } => cmd_transform(text, |s| s.to_kebab_case()),
        TextCommand::Scream { text } => cmd_transform(text, |s| s.to_shouty_snake_case()),
        TextCommand::Slug { text } => cmd_slug(text),
        TextCommand::Reverse { text } => cmd_transform(text, |s| s.chars().rev().collect()),
        TextCommand::Count { text } => cmd_count(text),
        TextCommand::Lorem { paragraphs, words } => cmd_lorem(paragraphs, words),
        TextCommand::Repeat {
            text,
            times,
            separator,
        } => cmd_repeat(&text, times, &separator),
        TextCommand::Trim { text } => cmd_transform(text, |s| s.trim().to_string()),
    }
}

/// Get text from argument or stdin
fn get_text(text: Option<String>) -> Result<String> {
    match text {
        Some(t) => Ok(t),
        None => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            Ok(input.trim_end().to_string())
        }
    }
}

/// Generic transform function
fn cmd_transform<F>(text: Option<String>, transform: F) -> Result<()>
where
    F: Fn(&str) -> String,
{
    let input = get_text(text)?;
    println!("{}", transform(&input));
    Ok(())
}

/// Create URL-safe slug
fn cmd_slug(text: Option<String>) -> Result<()> {
    let input = get_text(text)?;
    // Convert to kebab-case and remove non-alphanumeric except hyphens
    let slug: String = input
        .to_kebab_case()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    println!("{}", slug);
    Ok(())
}

/// Count characters, words, and lines
fn cmd_count(text: Option<String>) -> Result<()> {
    let input = get_text(text)?;
    let chars = input.chars().count();
    let words = input.split_whitespace().count();
    let lines = input.lines().count();

    println!("chars: {}", chars);
    println!("words: {}", words);
    println!("lines: {}", lines);
    Ok(())
}

/// Generate lorem ipsum text
fn cmd_lorem(paragraphs: usize, words: Option<usize>) -> Result<()> {
    if let Some(word_count) = words {
        // Generate specific number of words
        println!("{}", lipsum::lipsum(word_count));
    } else {
        // Generate paragraphs
        for i in 0..paragraphs {
            if i > 0 {
                println!();
            }
            println!("{}", lipsum::lipsum(50 + (i % 3) * 20));
        }
    }
    Ok(())
}

/// Repeat text N times
fn cmd_repeat(text: &str, times: usize, separator: &str) -> Result<()> {
    let repeated: Vec<&str> = (0..times).map(|_| text).collect();
    println!("{}", repeated.join(separator));
    Ok(())
}
