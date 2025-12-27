//! # Progress Bars with indicatif
//!
//! This example shows how to use progress bars for long operations.
//!
//! Run with: `cargo run --example 0402_progress_bars`

#![allow(dead_code)]

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Progress Bars with indicatif ===\n");

    // =========================================================================
    // SIMPLE SPINNER
    // =========================================================================

    println!("--- Simple Spinner ---");
    {
        let spinner = ProgressBar::new_spinner();
        spinner.set_message("Processing...");
        spinner.enable_steady_tick(Duration::from_millis(100));

        for _ in 0..10 {
            thread::sleep(Duration::from_millis(100));
        }

        spinner.finish_with_message("Done!");
    }

    println!();

    // =========================================================================
    // PROGRESS BAR
    // =========================================================================

    println!("--- Progress Bar ---");
    {
        let total = 50;
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        for _ in 0..total {
            pb.inc(1);
            thread::sleep(Duration::from_millis(50));
        }

        pb.finish_with_message("Complete");
    }

    println!();

    // =========================================================================
    // DOWNLOAD STYLE
    // =========================================================================

    println!("--- Download Style ---");
    {
        let total = 1024 * 1024 * 10; // 10 MB
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec})")
                .unwrap()
                .progress_chars("█▓░"),
        );

        let chunk_size = total / 20;
        for _ in 0..20 {
            pb.inc(chunk_size);
            thread::sleep(Duration::from_millis(100));
        }

        pb.finish_with_message("Downloaded");
    }

    println!();

    // =========================================================================
    // MULTI PROGRESS
    // =========================================================================

    println!("--- Multi Progress (parallel tasks) ---");
    {
        let m = MultiProgress::new();

        let pb1 = m.add(ProgressBar::new(30));
        pb1.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.bold} [{bar:30.green/white}] {pos}/{len}")
                .unwrap(),
        );
        pb1.set_prefix("Task 1");

        let pb2 = m.add(ProgressBar::new(20));
        pb2.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.bold} [{bar:30.yellow/white}] {pos}/{len}")
                .unwrap(),
        );
        pb2.set_prefix("Task 2");

        let pb3 = m.add(ProgressBar::new(40));
        pb3.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.bold} [{bar:30.cyan/white}] {pos}/{len}")
                .unwrap(),
        );
        pb3.set_prefix("Task 3");

        // Simulate parallel progress
        for i in 0..40 {
            if i < 30 {
                pb1.inc(1);
            }
            if i < 20 {
                pb2.inc(1);
            }
            pb3.inc(1);
            thread::sleep(Duration::from_millis(50));
        }

        pb1.finish();
        pb2.finish();
        pb3.finish();
    }

    println!();

    // =========================================================================
    // TEMPLATES
    // =========================================================================

    println!("--- Template Variables ---");
    println!(
        r#"
Available template variables:

  {{spinner}}      - Animated spinner
  {{bar}}          - Progress bar
  {{pos}}          - Current position
  {{len}}          - Total length
  {{percent}}      - Percentage
  {{elapsed}}      - Elapsed time (short)
  {{elapsed_precise}} - Elapsed time (full)
  {{eta}}          - Estimated time remaining
  {{eta_precise}}  - ETA (full)
  {{bytes}}        - Bytes processed
  {{total_bytes}}  - Total bytes
  {{bytes_per_sec}} - Speed
  {{msg}}          - Custom message
  {{prefix}}       - Prefix text

Example styles:
  "[{{bar:40}}] {{pos}}/{{len}}"
  "{{spinner}} {{msg}}"
  "[{{elapsed}}] {{bar:20}} {{percent}}%"
"#
    );

    println!();

    // =========================================================================
    // USAGE PATTERNS
    // =========================================================================

    println!("--- Usage Patterns ---");
    println!(
        r#"
1. FILE PROCESSING:
   let pb = ProgressBar::new(file_count);
   for file in files {{
       process(file);
       pb.inc(1);
   }}
   pb.finish();

2. DOWNLOADS:
   let pb = ProgressBar::new(total_bytes);
   while let Some(chunk) = download_chunk() {{
       pb.inc(chunk.len() as u64);
   }}

3. INDEFINITE (spinner):
   let pb = ProgressBar::new_spinner();
   pb.enable_steady_tick(Duration::from_millis(100));
   // ... work ...
   pb.finish_with_message("Done");

4. HIDDEN (quiet mode):
   let pb = if quiet {{
       ProgressBar::hidden()
   }} else {{
       ProgressBar::new(total)
   }};
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Progress bar features:");
    println!("  1. Spinners for indefinite work");
    println!("  2. Progress bars for known totals");
    println!("  3. Custom templates and styles");
    println!("  4. Multi-progress for parallel tasks");
    println!("  5. Hidden mode for quiet output");
}
