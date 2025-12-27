//! Expression evaluator command arguments.

use clap::{Args, Subcommand};

/// Expression evaluator - parse and evaluate math expressions
#[derive(Args, Debug)]
pub struct ExprArgs {
    #[command(subcommand)]
    pub command: ExprCommand,
}

#[derive(Subcommand, Debug)]
pub enum ExprCommand {
    /// Evaluate a math expression (reads from stdin if no expression given)
    #[command(visible_alias = "e")]
    Eval {
        /// Math expression (e.g., "2+2", "sqrt(16)", "2^10", "sin(pi/2)")
        /// If omitted, reads multi-line input from stdin (end with Ctrl+D)
        expression: Option<String>,
    },

    /// Run a program from file (supports variables and multi-line)
    #[command(visible_alias = "r")]
    Run {
        /// File path (use "-" for stdin)
        file: String,
    },

    /// Parse expression and show AST as JSON
    Ast {
        /// Math expression to parse
        expression: String,

        /// Pretty-print the JSON output
        #[arg(short, long)]
        pretty: bool,
    },

    /// Show available functions and constants
    #[command(visible_alias = "l")]
    List,
}
