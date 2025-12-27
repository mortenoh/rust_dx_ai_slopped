//! Watch command - watch files for changes and run commands.

use crate::cli::commands::watch::WatchArgs;
use anyhow::{Context, Result};
use colored::Colorize;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebounceEventResult};
use regex::Regex;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

/// Run the watch command
pub fn run(args: WatchArgs) -> Result<()> {
    // Build the command string for display
    let cmd_display = args.command.join(" ");
    println!(
        "{} {} {}",
        "Watching".cyan(),
        args.paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", "),
        format!("→ {}", cmd_display).dimmed()
    );

    // Run initial command if requested
    if args.initial {
        run_command(&args.command, args.clear)?;
    }

    // Set up file watcher
    let (tx, rx) = channel::<DebounceEventResult>();

    let debounce_duration = Duration::from_millis(args.debounce);
    let mut debouncer =
        new_debouncer(debounce_duration, tx).context("Failed to create file watcher")?;

    // Watch all specified paths
    let mode = if args.recursive {
        RecursiveMode::Recursive
    } else {
        RecursiveMode::NonRecursive
    };

    for path in &args.paths {
        debouncer
            .watcher()
            .watch(path, mode)
            .context(format!("Failed to watch path: {}", path.display()))?;
    }

    // Build include/exclude patterns
    let include_pattern = args.include.as_ref().map(|p| build_glob_regex(p));
    let exclude_pattern = args.exclude.as_ref().map(|p| build_glob_regex(p));

    println!("{}", "Press Ctrl+C to stop".dimmed());
    println!();

    // Main event loop
    loop {
        match rx.recv() {
            Ok(Ok(events)) => {
                // Check if any event matches our filters
                let should_run = events.iter().any(|event| {
                    let path = &event.path;
                    should_include_path(path, &include_pattern, &exclude_pattern)
                });

                if should_run {
                    // Show which files changed
                    for event in &events {
                        println!(
                            "{} {}",
                            "Changed:".yellow(),
                            event.path.display().to_string().dimmed()
                        );
                    }

                    run_command(&args.command, args.clear)?;
                }
            }
            Ok(Err(error)) => {
                eprintln!("{} {}", "Watch error:".red(), error);
            }
            Err(e) => {
                eprintln!("{} {}", "Channel error:".red(), e);
                break;
            }
        }
    }

    Ok(())
}

fn build_glob_regex(pattern: &str) -> Regex {
    let regex_pattern = pattern
        .replace('.', r"\.")
        .replace('*', ".*")
        .replace('?', ".");

    Regex::new(&format!("(?i){}", regex_pattern)).unwrap_or_else(|_| Regex::new(".*").unwrap())
}

fn should_include_path(path: &Path, include: &Option<Regex>, exclude: &Option<Regex>) -> bool {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    // Skip hidden files
    if name.starts_with('.') {
        return false;
    }

    // Check exclude pattern
    if let Some(exclude_re) = exclude {
        if exclude_re.is_match(&name) {
            return false;
        }
    }

    // Check include pattern
    if let Some(include_re) = include {
        return include_re.is_match(&name);
    }

    true
}

fn run_command(command: &[String], clear: bool) -> Result<()> {
    if clear {
        // Clear screen using ANSI escape codes
        print!("\x1b[2J\x1b[H");
    }

    println!("{}", "─".repeat(60).dimmed());

    if command.is_empty() {
        return Ok(());
    }

    let (program, args) = command.split_first().unwrap();

    let status = Command::new(program)
        .args(args)
        .status()
        .context(format!("Failed to run command: {}", program))?;

    println!();
    if status.success() {
        println!("{}", "✓ Command completed successfully".green());
    } else {
        let code = status.code().unwrap_or(-1);
        println!(
            "{}",
            format!("✗ Command failed (exit code: {})", code).red()
        );
    }
    println!("{}", "─".repeat(60).dimmed());
    println!();

    Ok(())
}
