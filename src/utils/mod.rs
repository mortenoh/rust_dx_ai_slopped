//! Shared utilities.

mod output;
pub mod progress;

pub use output::{print_error, print_success, print_warning};
pub use progress::{osc_progress, osc_progress_clear, ProgressState, TerminalProgress};
