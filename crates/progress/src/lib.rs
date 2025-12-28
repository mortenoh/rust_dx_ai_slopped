//! # dx-progress
//!
//! A lightweight terminal progress library with native terminal integration.
//!
//! This crate provides progress reporting that integrates with modern terminals
//! via OSC 9;4 escape sequences, supported by:
//! - Ghostty (1.2+)
//! - Windows Terminal
//! - ConEmu
//! - iTerm2 (partial)
//!
//! ## Features
//!
//! - **OSC 9;4 Progress Reporting**: Native terminal progress indicators
//! - **Visual Progress Bars**: ANSI-based progress bars
//! - **Spinners**: Animated spinner support
//! - **Zero Dependencies**: No external crates required
//!
//! ## Quick Start
//!
//! ```no_run
//! use dx_progress::{TerminalProgress, ProgressState};
//!
//! // Simple progress
//! let mut progress = TerminalProgress::new(100);
//! for i in 0..=100 {
//!     progress.set(i);
//!     progress.draw(Some("Processing..."));
//!     std::thread::sleep(std::time::Duration::from_millis(50));
//! }
//! progress.finish_with_message("Done!");
//! ```
//!
//! ## Low-Level API
//!
//! ```no_run
//! use dx_progress::{osc_progress, osc_progress_clear, ProgressState};
//!
//! // Report 50% progress
//! osc_progress(50, ProgressState::Normal);
//!
//! // Show error state
//! osc_progress(50, ProgressState::Error);
//!
//! // Clear when done
//! osc_progress_clear();
//! ```

use std::io::{self, Write};

/// Progress state for OSC 9;4 reporting.
///
/// Different states affect how the terminal displays the progress indicator:
/// - `Normal`: Standard progress bar (usually blue/green)
/// - `Error`: Error state (usually red)
/// - `Indeterminate`: Pulsing/spinning state for unknown duration
/// - `Warning`: Warning state (usually yellow)
/// - `Hidden`: Clear/hide the progress indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressState {
    /// Hide/clear progress indicator
    Hidden = 0,
    /// Normal progress (default)
    #[default]
    Normal = 1,
    /// Error state (red)
    Error = 2,
    /// Indeterminate/paused (pulsing)
    Indeterminate = 3,
    /// Warning state (yellow)
    Warning = 4,
}

/// Report progress to terminal emulator via OSC 9;4.
///
/// This works with terminals that support the ConEmu-style progress reporting:
/// - Ghostty (1.2+)
/// - Windows Terminal
/// - ConEmu
/// - iTerm2 (partial)
///
/// # Arguments
/// * `percent` - Progress percentage (0-100, clamped)
/// * `state` - Progress state (Normal, Error, Indeterminate, etc.)
///
/// # Example
/// ```no_run
/// use dx_progress::{osc_progress, ProgressState};
///
/// osc_progress(50, ProgressState::Normal);  // 50% progress
/// osc_progress(75, ProgressState::Warning); // 75% with warning
/// osc_progress(0, ProgressState::Hidden);   // Clear progress
/// ```
pub fn osc_progress(percent: u64, state: ProgressState) {
    if state == ProgressState::Hidden {
        print!("\x1b]9;4;0\x1b\\");
    } else {
        print!("\x1b]9;4;{};{}\x1b\\", state as u8, percent.min(100));
    }
    io::stdout().flush().ok();
}

/// Clear the terminal progress indicator.
///
/// This is a convenience function equivalent to `osc_progress(0, ProgressState::Hidden)`.
#[inline]
pub fn osc_progress_clear() {
    osc_progress(0, ProgressState::Hidden);
}

/// Terminal progress reporter with automatic OSC 9;4 integration.
///
/// Provides both visual (ANSI) progress bar and terminal-native progress
/// reporting for supported terminals. Automatically clears OSC progress
/// on drop to prevent stuck indicators.
///
/// # Example
/// ```no_run
/// use dx_progress::TerminalProgress;
///
/// let mut progress = TerminalProgress::new(100);
/// for i in 0..=100 {
///     progress.set(i);
///     progress.draw(Some("Working..."));
///     std::thread::sleep(std::time::Duration::from_millis(20));
/// }
/// progress.finish_with_message("Complete!");
/// ```
pub struct TerminalProgress {
    total: u64,
    current: u64,
    state: ProgressState,
    show_bar: bool,
    bar_width: usize,
}

impl TerminalProgress {
    /// Create a new progress reporter.
    ///
    /// # Arguments
    /// * `total` - The total value representing 100%
    pub fn new(total: u64) -> Self {
        Self {
            total,
            current: 0,
            state: ProgressState::Normal,
            show_bar: true,
            bar_width: 30,
        }
    }

    /// Create an indeterminate (spinner-style) progress reporter.
    ///
    /// Use this when the total amount of work is unknown.
    pub fn indeterminate() -> Self {
        let mut p = Self::new(0);
        p.state = ProgressState::Indeterminate;
        osc_progress(0, ProgressState::Indeterminate);
        p
    }

    /// Set whether to show the visual progress bar (default: true).
    pub fn show_bar(mut self, show: bool) -> Self {
        self.show_bar = show;
        self
    }

    /// Set the width of the visual progress bar (default: 30).
    pub fn bar_width(mut self, width: usize) -> Self {
        self.bar_width = width;
        self
    }

    /// Get the current percentage (0-100).
    pub fn percent(&self) -> u64 {
        if self.total == 0 {
            0
        } else {
            (self.current * 100) / self.total
        }
    }

    /// Get the current progress value.
    pub fn current(&self) -> u64 {
        self.current
    }

    /// Get the total value.
    pub fn total(&self) -> u64 {
        self.total
    }

    /// Set the current progress value.
    pub fn set(&mut self, value: u64) {
        self.current = value.min(self.total);
        self.update();
    }

    /// Increment the progress by a given amount.
    pub fn inc(&mut self, delta: u64) {
        self.set(self.current.saturating_add(delta));
    }

    /// Set the progress state (Normal, Error, Warning, etc.).
    pub fn set_state(&mut self, state: ProgressState) {
        self.state = state;
        self.update();
    }

    /// Update the terminal with current progress.
    fn update(&self) {
        osc_progress(self.percent(), self.state);
    }

    /// Draw a visual progress bar to stderr.
    ///
    /// # Arguments
    /// * `msg` - Optional message to display after the bar
    pub fn draw(&self, msg: Option<&str>) {
        if !self.show_bar {
            return;
        }

        let percent = self.percent() as usize;
        let filled = (percent * self.bar_width) / 100;
        let empty = self.bar_width - filled;

        let bar = format!(
            "\x1b[36m{}\x1b[34m{}\x1b[0m", // cyan filled, blue empty
            "█".repeat(filled),
            "░".repeat(empty)
        );

        if let Some(m) = msg {
            eprint!("\r\x1b[K[{}] {}% {}", bar, percent, m);
        } else {
            eprint!("\r\x1b[K[{}] {}%", bar, percent);
        }
        io::stderr().flush().ok();
    }

    /// Finish and clear the progress indicator.
    pub fn finish(&self) {
        osc_progress_clear();
        if self.show_bar {
            eprint!("\r\x1b[K");
            io::stderr().flush().ok();
        }
    }

    /// Finish with a completion message.
    pub fn finish_with_message(&self, msg: &str) {
        osc_progress_clear();
        eprintln!("\r\x1b[K{}", msg);
    }
}

impl Drop for TerminalProgress {
    fn drop(&mut self) {
        // Always clear OSC progress on drop to avoid stuck indicators
        osc_progress_clear();
    }
}

/// Bouncing progress bar for indeterminate operations.
///
/// Shows a bar segment that bounces back and forth, useful when
/// the total amount of work is unknown. Sends OSC 9;4;3 for
/// terminal-native indeterminate progress in supported terminals.
///
/// # Example
/// ```no_run
/// use dx_progress::BouncingBar;
/// use std::time::Duration;
///
/// let mut bar = BouncingBar::new();
/// for _ in 0..100 {
///     bar.tick();
///     bar.draw(Some("Loading..."));
///     std::thread::sleep(Duration::from_millis(50));
/// }
/// bar.finish_with_message("Done!");
/// ```
pub struct BouncingBar {
    position: usize,
    direction: i8, // 1 = right, -1 = left
    bar_width: usize,
    ball_width: usize,
}

impl BouncingBar {
    /// Create a new bouncing progress bar.
    pub fn new() -> Self {
        // Send indeterminate state to terminal
        osc_progress(0, ProgressState::Indeterminate);
        Self {
            position: 0,
            direction: 1,
            bar_width: 30,
            ball_width: 3,
        }
    }

    /// Set the width of the progress bar (default: 30).
    pub fn bar_width(mut self, width: usize) -> Self {
        self.bar_width = width;
        self
    }

    /// Set the width of the bouncing segment (default: 3).
    pub fn ball_width(mut self, width: usize) -> Self {
        self.ball_width = width.max(1);
        self
    }

    /// Advance the animation by one frame.
    pub fn tick(&mut self) {
        let max_pos = self.bar_width.saturating_sub(self.ball_width);

        if self.direction > 0 {
            if self.position >= max_pos {
                self.direction = -1;
                self.position = self.position.saturating_sub(1);
            } else {
                self.position += 1;
            }
        } else {
            if self.position == 0 {
                self.direction = 1;
                self.position = 1;
            } else {
                self.position = self.position.saturating_sub(1);
            }
        }
    }

    /// Draw the bouncing bar to stderr.
    pub fn draw(&self, msg: Option<&str>) {
        let before = self.position;
        let after = self
            .bar_width
            .saturating_sub(self.position + self.ball_width);

        let bar = format!(
            "\x1b[34m{}\x1b[36m{}\x1b[34m{}\x1b[0m",
            "░".repeat(before),
            "█".repeat(self.ball_width),
            "░".repeat(after)
        );

        if let Some(m) = msg {
            eprint!("\r\x1b[K[{}] {}", bar, m);
        } else {
            eprint!("\r\x1b[K[{}]", bar);
        }
        io::stderr().flush().ok();
    }

    /// Finish and clear the progress indicator.
    pub fn finish(&self) {
        osc_progress_clear();
        eprint!("\r\x1b[K");
        io::stderr().flush().ok();
    }

    /// Finish with a completion message.
    pub fn finish_with_message(&self, msg: &str) {
        osc_progress_clear();
        eprintln!("\r\x1b[K{}", msg);
    }
}

impl Default for BouncingBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BouncingBar {
    fn drop(&mut self) {
        osc_progress_clear();
    }
}

/// Spinner frames for terminal animation.
///
/// Braille-based spinner that works well in most terminals.
pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Draw a spinner frame to stderr.
///
/// # Arguments
/// * `frame` - Frame index (will wrap around automatically)
/// * `msg` - Message to display next to spinner
///
/// # Example
/// ```no_run
/// use dx_progress::draw_spinner;
///
/// for frame in 0..50 {
///     draw_spinner(frame, "Loading...");
///     std::thread::sleep(std::time::Duration::from_millis(80));
/// }
/// eprintln!("\r\x1b[KDone!");
/// ```
pub fn draw_spinner(frame: usize, msg: &str) {
    let spinner = SPINNER_FRAMES[frame % SPINNER_FRAMES.len()];
    eprint!("\r\x1b[K\x1b[32m{}\x1b[0m {}", spinner, msg); // green spinner
    io::stderr().flush().ok();
}

/// Clear the current line on stderr.
///
/// Useful for cleaning up after spinners or progress bars.
pub fn clear_line() {
    eprint!("\r\x1b[K");
    io::stderr().flush().ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_percent() {
        let mut p = TerminalProgress::new(100);
        assert_eq!(p.percent(), 0);

        p.set(50);
        assert_eq!(p.percent(), 50);

        p.set(100);
        assert_eq!(p.percent(), 100);
    }

    #[test]
    fn test_progress_percent_rounding() {
        let mut p = TerminalProgress::new(3);
        p.set(1);
        assert_eq!(p.percent(), 33);

        p.set(2);
        assert_eq!(p.percent(), 66);
    }

    #[test]
    fn test_progress_increment() {
        let mut p = TerminalProgress::new(100);
        p.inc(10);
        assert_eq!(p.percent(), 10);
        p.inc(90);
        assert_eq!(p.percent(), 100);
    }

    #[test]
    fn test_progress_overflow() {
        let mut p = TerminalProgress::new(100);
        p.set(200); // Over 100%
        assert_eq!(p.percent(), 100);
    }

    #[test]
    fn test_zero_total() {
        let p = TerminalProgress::new(0);
        assert_eq!(p.percent(), 0);
    }

    #[test]
    fn test_progress_state_values() {
        assert_eq!(ProgressState::Hidden as u8, 0);
        assert_eq!(ProgressState::Normal as u8, 1);
        assert_eq!(ProgressState::Error as u8, 2);
        assert_eq!(ProgressState::Indeterminate as u8, 3);
        assert_eq!(ProgressState::Warning as u8, 4);
    }
}
