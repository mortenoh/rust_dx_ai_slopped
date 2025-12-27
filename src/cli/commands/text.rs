//! Text transformation command arguments.

use clap::{Args, Subcommand};

/// Text transformation utilities
#[derive(Args, Debug)]
pub struct TextArgs {
    #[command(subcommand)]
    pub command: TextCommand,
}

#[derive(Subcommand, Debug)]
pub enum TextCommand {
    /// Convert to UPPERCASE
    Upper {
        /// Text to transform (or use stdin)
        text: Option<String>,
    },
    /// Convert to lowercase
    Lower {
        /// Text to transform (or use stdin)
        text: Option<String>,
    },
    /// Convert to Title Case
    Title {
        /// Text to transform (or use stdin)
        text: Option<String>,
    },
    /// Convert to snake_case
    Snake {
        /// Text to transform (or use stdin)
        text: Option<String>,
    },
    /// Convert to camelCase
    Camel {
        /// Text to transform (or use stdin)
        text: Option<String>,
    },
    /// Convert to PascalCase
    Pascal {
        /// Text to transform (or use stdin)
        text: Option<String>,
    },
    /// Convert to kebab-case
    Kebab {
        /// Text to transform (or use stdin)
        text: Option<String>,
    },
    /// Convert to SCREAMING_SNAKE_CASE
    Scream {
        /// Text to transform (or use stdin)
        text: Option<String>,
    },
    /// Convert to url-safe-slug
    Slug {
        /// Text to transform (or use stdin)
        text: Option<String>,
    },
    /// Reverse text
    Reverse {
        /// Text to reverse (or use stdin)
        text: Option<String>,
    },
    /// Count characters, words, lines
    Count {
        /// Text to count (or use stdin)
        text: Option<String>,
    },
    /// Generate lorem ipsum text
    Lorem {
        /// Number of paragraphs
        #[arg(default_value = "1")]
        paragraphs: usize,
        /// Generate words instead of paragraphs
        #[arg(short, long)]
        words: Option<usize>,
    },
    /// Repeat text N times
    Repeat {
        /// Text to repeat
        text: String,
        /// Number of times
        times: usize,
        /// Separator between repetitions
        #[arg(short, long, default_value = "\n")]
        separator: String,
    },
    /// Trim whitespace
    Trim {
        /// Text to trim (or use stdin)
        text: Option<String>,
    },
}
