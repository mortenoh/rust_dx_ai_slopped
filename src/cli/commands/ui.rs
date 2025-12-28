//! TUI dashboard command arguments.

use clap::Args;

/// Interactive TUI dashboard
#[derive(Args, Debug)]
pub struct UiArgs {
    /// Refresh rate in milliseconds
    #[arg(short, long, default_value = "250")]
    pub tick: u64,
}
