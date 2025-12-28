//! Diff command - text diffing utilities.

use crate::cli::commands::diff::{DiffArgs, DiffFormat};
use anyhow::{Context, Result};
use colored::Colorize;
use similar::{ChangeTag, TextDiff};
use std::fs;

/// Run the diff command
pub fn run(args: DiffArgs) -> Result<()> {
    let text1 = fs::read_to_string(&args.file1)
        .with_context(|| format!("Failed to read file: {}", args.file1.display()))?;
    let text2 = fs::read_to_string(&args.file2)
        .with_context(|| format!("Failed to read file: {}", args.file2.display()))?;

    let diff = TextDiff::from_lines(&text1, &text2);

    match args.format {
        DiffFormat::Unified => {
            println!("{}", format!("--- {}", args.file1.display()).red());
            println!("{}", format!("+++ {}", args.file2.display()).green());

            for hunk in diff
                .unified_diff()
                .context_radius(args.context)
                .iter_hunks()
            {
                println!("{}", format!("{}", hunk.header()).cyan());
                for change in hunk.iter_changes() {
                    let line = match change.tag() {
                        ChangeTag::Delete => format!("-{}", change).red().to_string(),
                        ChangeTag::Insert => format!("+{}", change).green().to_string(),
                        ChangeTag::Equal => format!(" {}", change).to_string(),
                    };
                    print!("{}", line);
                }
            }
        }
        DiffFormat::Inline => {
            for change in diff.iter_all_changes() {
                match change.tag() {
                    ChangeTag::Delete => {
                        print!("{}", format!("[-{}]", change.value().trim_end()).red());
                    }
                    ChangeTag::Insert => {
                        print!("{}", format!("[+{}]", change.value().trim_end()).green());
                    }
                    ChangeTag::Equal => {
                        print!("{}", change.value());
                    }
                }
            }
        }
        DiffFormat::Compact => {
            let mut line_num_old = 0usize;
            let mut line_num_new = 0usize;

            for change in diff.iter_all_changes() {
                match change.tag() {
                    ChangeTag::Delete => {
                        line_num_old += 1;
                        println!(
                            "{}",
                            format!("{:4} -    | {}", line_num_old, change.value().trim_end())
                                .red()
                        );
                    }
                    ChangeTag::Insert => {
                        line_num_new += 1;
                        println!(
                            "{}",
                            format!("     + {:3}| {}", line_num_new, change.value().trim_end())
                                .green()
                        );
                    }
                    ChangeTag::Equal => {
                        line_num_old += 1;
                        line_num_new += 1;
                    }
                }
            }
        }
    }

    Ok(())
}
