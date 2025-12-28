//! Fun terminal UI command arguments.

use clap::{Args, Subcommand};

/// Fun terminal UI effects
#[derive(Args, Debug)]
pub struct FunArgs {
    #[command(subcommand)]
    pub command: FunCommand,
}

#[derive(Subcommand, Debug)]
pub enum FunCommand {
    /// Fake progress bar with funny messages
    Progress {
        /// Total duration in seconds
        #[arg(short, long, default_value = "10")]
        duration: u64,
        /// Progress style: bar, spinner, or both
        #[arg(short, long, default_value = "both")]
        style: String,
    },
    /// Fake hacker terminal output
    Hacker {
        /// Duration in seconds
        #[arg(short, long, default_value = "15")]
        duration: u64,
        /// Output intensity (1=slow, 2=medium, 3=fast)
        #[arg(short, long, default_value = "2")]
        intensity: u8,
    },
    /// Countdown timer with visual effects
    Countdown {
        /// Countdown duration in seconds
        seconds: u64,
        /// Message to display when done
        #[arg(short, long)]
        message: Option<String>,
        /// Simple mode (no box art)
        #[arg(long)]
        simple: bool,
    },
    /// Showcase all available spinner styles
    Spinners {
        /// Duration per spinner in seconds
        #[arg(short, long, default_value = "2")]
        duration: u64,
        /// Show only a specific spinner by name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Simulate doing fake work with progress bars
    Work {
        /// Total duration in seconds
        #[arg(short, long, default_value = "30")]
        duration: u64,
        /// Number of fake tasks to complete
        #[arg(short, long, default_value = "8")]
        tasks: usize,
        /// Progress bar style (block, gradient, arrow, dots, emoji, classic)
        #[arg(short, long, default_value = "gradient")]
        style: String,
        /// List available progress bar styles
        #[arg(long)]
        list_styles: bool,
    },
    /// Show random programming wisdom with ASCII art
    Fortune {
        /// ASCII art animal (cow, tux, ghost, dragon, cat, dog)
        #[arg(short, long)]
        animal: Option<String>,
        /// Custom message instead of random fortune
        #[arg(short, long)]
        say: Option<String>,
        /// List available animals
        #[arg(short, long)]
        list: bool,
    },
}
