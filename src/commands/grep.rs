//! Grep command - search for patterns in files.

use crate::cli::commands::grep::GrepArgs;
use anyhow::{Context, Result};
use colored::Colorize;
use regex::{Regex, RegexBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

/// Run the grep command
pub fn run(args: GrepArgs) -> Result<()> {
    let regex = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .context("Invalid regex pattern")?;

    let paths = if args.paths.is_empty() {
        vec![std::path::PathBuf::from(".")]
    } else {
        args.paths.clone()
    };

    let mut total_matches = 0;

    for path in &paths {
        if path.is_file() {
            total_matches += search_file(path, &regex, &args)?;
        } else if path.is_dir() {
            if args.recursive {
                total_matches += search_dir(path, &regex, &args)?;
            } else {
                // Non-recursive: only search files in this directory
                for entry in std::fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() && should_include(&path, &args) {
                        total_matches += search_file(&path, &regex, &args)?;
                    }
                }
            }
        } else {
            eprintln!("{}: No such file or directory", path.display());
        }
    }

    if args.count && paths.len() == 1 && paths[0].is_file() {
        // Already printed in search_file
    } else if total_matches == 0 && !args.files_only && !args.count {
        // No matches found, exit with non-zero status
        std::process::exit(1);
    }

    Ok(())
}

fn search_dir(dir: &Path, regex: &Regex, args: &GrepArgs) -> Result<usize> {
    let mut total = 0;
    let walker = WalkDir::new(dir).follow_links(true);

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        // Skip hidden files/dirs unless --hidden
        if !args.hidden {
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }
        }

        if path.is_file() && should_include(path, args) {
            total += search_file(path, regex, args)?;
        }
    }

    Ok(total)
}

fn should_include(path: &Path, args: &GrepArgs) -> bool {
    let name = match path.file_name() {
        Some(n) => n.to_string_lossy(),
        None => return false,
    };

    // Check exclude pattern
    if let Some(exclude) = &args.exclude {
        if glob_match(exclude, &name) {
            return false;
        }
    }

    // Check include pattern
    if let Some(include) = &args.include {
        return glob_match(include, &name);
    }

    // Skip binary files (simple heuristic: check extension)
    let binary_extensions = ["exe", "bin", "o", "so", "dylib", "dll", "a", "png", "jpg", "gif", "pdf", "zip", "tar", "gz"];
    if let Some(ext) = path.extension() {
        if binary_extensions.contains(&ext.to_string_lossy().to_lowercase().as_str()) {
            return false;
        }
    }

    true
}

fn glob_match(pattern: &str, name: &str) -> bool {
    // Simple glob matching: * matches anything
    let regex_pattern = pattern
        .replace('.', r"\.")
        .replace('*', ".*")
        .replace('?', ".");

    if let Ok(re) = Regex::new(&format!("^{}$", regex_pattern)) {
        re.is_match(name)
    } else {
        false
    }
}

fn search_file(path: &Path, regex: &Regex, args: &GrepArgs) -> Result<usize> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {}", path.display(), e);
            return Ok(0);
        }
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    let mut match_count = 0;
    let mut matched_lines: Vec<(usize, String)> = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        let is_match = regex.is_match(line);
        let should_show = if args.invert { !is_match } else { is_match };

        if should_show {
            match_count += 1;
            matched_lines.push((line_num + 1, line.clone()));
        }
    }

    if match_count == 0 {
        return Ok(0);
    }

    // Output based on mode
    if args.files_only {
        println!("{}", path.display());
        return Ok(match_count);
    }

    if args.count {
        println!("{}:{}", path.display(), match_count);
        return Ok(match_count);
    }

    // Determine context lines
    let before = args.context.or(args.before).unwrap_or(0);
    let after = args.context.or(args.after).unwrap_or(0);

    let show_filename = args.recursive || args.paths.len() > 1;

    for (line_num, line) in &matched_lines {
        // Show context before
        if before > 0 {
            let start = line_num.saturating_sub(before + 1);
            for i in start..(*line_num - 1) {
                if i < lines.len() {
                    print_context_line(path, i + 1, &lines[i], show_filename, args.line_number);
                }
            }
        }

        // Show matching line
        print_match_line(path, *line_num, line, regex, show_filename, args.line_number);

        // Show context after
        if after > 0 {
            for i in *line_num..(*line_num + after).min(lines.len()) {
                print_context_line(path, i + 1, &lines[i], show_filename, args.line_number);
            }
        }
    }

    Ok(match_count)
}

fn print_match_line(path: &Path, line_num: usize, line: &str, regex: &Regex, show_filename: bool, show_line_num: bool) {
    let mut output = String::new();

    if show_filename {
        output.push_str(&format!("{}:", path.display().to_string().magenta()));
    }

    if show_line_num {
        output.push_str(&format!("{}:", line_num.to_string().green()));
    }

    // Highlight matches in the line
    let highlighted = regex.replace_all(line, |caps: &regex::Captures| {
        caps[0].red().bold().to_string()
    });

    output.push_str(&highlighted);
    println!("{}", output);
}

fn print_context_line(path: &Path, line_num: usize, line: &str, show_filename: bool, show_line_num: bool) {
    let mut output = String::new();

    if show_filename {
        output.push_str(&format!("{}-", path.display().to_string().magenta()));
    }

    if show_line_num {
        output.push_str(&format!("{}-", line_num.to_string().green()));
    }

    output.push_str(line);
    println!("{}", output.dimmed());
}
