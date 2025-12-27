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
    /// Evaluate a math expression
    #[command(visible_alias = "e")]
    Eval {
        /// Math expression (e.g., "2+2", "sqrt(16)", "2^10", "sin(pi/2)")
        expression: String,
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
