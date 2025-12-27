//! # Table Output with comfy-table
//!
//! This example shows how to format data as tables.
//!
//! Run with: `cargo run --example 0404_table_output`

#![allow(dead_code)]

use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};

fn main() {
    println!("=== Table Output with comfy-table ===\n");

    // =========================================================================
    // BASIC TABLE
    // =========================================================================

    println!("--- Basic Table ---");
    {
        let mut table = Table::new();
        table
            .set_header(vec!["Name", "Age", "Status"])
            .add_row(vec!["Alice", "30", "Active"])
            .add_row(vec!["Bob", "25", "Inactive"])
            .add_row(vec!["Charlie", "35", "Active"]);

        println!("{}", table);
    }

    println!();

    // =========================================================================
    // STYLED TABLE
    // =========================================================================

    println!("--- Styled Table ---");
    {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_content_arrangement(ContentArrangement::Dynamic);

        table.set_header(vec![
            Cell::new("ID").add_attribute(Attribute::Bold),
            Cell::new("Name").add_attribute(Attribute::Bold),
            Cell::new("Status").add_attribute(Attribute::Bold),
        ]);

        table.add_row(vec![
            Cell::new("1"),
            Cell::new("Production"),
            Cell::new("✓ Running").fg(Color::Green),
        ]);

        table.add_row(vec![
            Cell::new("2"),
            Cell::new("Staging"),
            Cell::new("✓ Running").fg(Color::Green),
        ]);

        table.add_row(vec![
            Cell::new("3"),
            Cell::new("Development"),
            Cell::new("✗ Stopped").fg(Color::Red),
        ]);

        println!("{}", table);
    }

    println!();

    // =========================================================================
    // DATA TABLE
    // =========================================================================

    println!("--- Data Table ---");
    {
        #[derive(Debug)]
        struct FileInfo {
            name: String,
            size: u64,
            modified: String,
        }

        let files = vec![
            FileInfo {
                name: "config.toml".into(),
                size: 1024,
                modified: "2024-01-15".into(),
            },
            FileInfo {
                name: "data.json".into(),
                size: 51200,
                modified: "2024-01-14".into(),
            },
            FileInfo {
                name: "README.md".into(),
                size: 2048,
                modified: "2024-01-10".into(),
            },
        ];

        let mut table = Table::new();
        table.set_header(vec!["Filename", "Size", "Modified"]);

        for file in &files {
            table.add_row(vec![&file.name, &format_size(file.size), &file.modified]);
        }

        println!("{}", table);
    }

    println!();

    // =========================================================================
    // PRESETS
    // =========================================================================

    println!("--- Table Presets ---");
    println!(
        r#"
Available presets:

  ASCII_FULL          +-----+-----+
                      | A   | B   |
                      +-----+-----+

  UTF8_FULL           ┌─────┬─────┐
                      │ A   │ B   │
                      └─────┴─────┘

  UTF8_BORDERS_ONLY   ┌───────────┐
                      │ A     B   │
                      └───────────┘

  NOTHING             A     B
                      1     2

Usage:
  table.load_preset(UTF8_FULL);
"#
    );

    println!();

    // =========================================================================
    // COLUMN CONFIGURATION
    // =========================================================================

    println!("--- Column Configuration ---");
    println!(
        r#"
// Set column widths
table.set_width(80);  // Total width

// Per-column settings
use comfy_table::ColumnConstraint::*;
table.set_constraints(vec![
    Absolute(10),      // Fixed width
    LowerBoundary(5),  // Minimum width
    UpperBoundary(20), // Maximum width
    ContentWidth,      // Fit content
]);

// Content arrangement
table.set_content_arrangement(ContentArrangement::Dynamic);
// or: Disabled, DynamicFullWidth
"#
    );

    println!();

    // =========================================================================
    // CONDITIONAL FORMATTING
    // =========================================================================

    println!("--- Conditional Formatting ---");
    {
        fn status_cell(status: &str) -> Cell {
            match status {
                "success" => Cell::new("✓ Success").fg(Color::Green),
                "warning" => Cell::new("⚠ Warning").fg(Color::Yellow),
                "error" => Cell::new("✗ Error").fg(Color::Red),
                _ => Cell::new(status),
            }
        }

        let mut table = Table::new();
        table.set_header(vec!["Task", "Status"]);
        table.add_row(vec![Cell::new("Build"), status_cell("success")]);
        table.add_row(vec![Cell::new("Test"), status_cell("warning")]);
        table.add_row(vec![Cell::new("Deploy"), status_cell("error")]);

        println!("{}", table);
    }

    println!();

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Table output features:");
    println!("  1. Headers and rows");
    println!("  2. Cell colors and attributes");
    println!("  3. Multiple presets (ASCII, UTF8)");
    println!("  4. Column width constraints");
    println!("  5. Dynamic content arrangement");
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
