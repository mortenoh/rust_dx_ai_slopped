//! # Expression Evaluator Command
//!
//! Parse and evaluate mathematical expressions.
//!
//! ## Examples
//! ```bash
//! dx expr eval "2 + 3 * 4"       # 14
//! dx expr eval "2^10"            # 1024
//! dx expr eval "sqrt(16) + pi"   # 7.14159...
//! dx expr eval "sin(pi/2)"       # 1
//! dx expr ast "2 + 3"            # Show AST as JSON
//! dx expr help                   # Show available functions
//! ```

use crate::cli::commands::expr::{ExprArgs, ExprCommand};
use crate::expr;
use anyhow::Result;
use colored::Colorize;

pub fn run(args: ExprArgs) -> Result<()> {
    match args.command {
        ExprCommand::Eval { expression } => cmd_eval(&expression),
        ExprCommand::Ast { expression, pretty } => cmd_ast(&expression, pretty),
        ExprCommand::List => cmd_list(),
    }
}

/// Evaluate a math expression
fn cmd_eval(expression: &str) -> Result<()> {
    let result = expr::eval(expression)?;

    // Format nicely: show as integer if whole number
    if result.fract() == 0.0 && result.abs() < 1e15 {
        println!("{}", result as i64);
    } else {
        println!("{}", result);
    }
    Ok(())
}

/// Parse expression and show AST as JSON
fn cmd_ast(expression: &str, pretty: bool) -> Result<()> {
    let ast = expr::parse(expression)?;

    let json = if pretty {
        serde_json::to_string_pretty(&ast)?
    } else {
        serde_json::to_string(&ast)?
    };

    println!("{}", json);
    Ok(())
}

/// Show available functions and constants
fn cmd_list() -> Result<()> {
    println!("{}", "Expression Evaluator".cyan().bold());
    println!();

    println!("{}", "OPERATORS".yellow());
    println!("  +    Addition           2 + 3 = 5");
    println!("  -    Subtraction        5 - 2 = 3");
    println!("  *    Multiplication     3 * 4 = 12");
    println!("  /    Division           15 / 3 = 5");
    println!("  %    Modulo             17 % 5 = 2");
    println!("  ^    Power              2 ^ 10 = 1024");
    println!("  -x   Unary minus        -5 = -5");
    println!("  ()   Parentheses        (2 + 3) * 4 = 20");
    println!();

    println!("{}", "CONSTANTS".yellow());
    println!("  pi   {:<18} {}", "3.14159...", "Circle ratio");
    println!("  e    {:<18} {}", "2.71828...", "Euler's number");
    println!("  tau  {:<18} {}", "6.28318...", "2 * pi");
    println!();

    println!("{}", "FUNCTIONS".yellow());
    println!();
    println!("  {}", "Trigonometric (radians)".cyan());
    println!("    sin(x)   cos(x)   tan(x)");
    println!("    asin(x)  acos(x)  atan(x)");
    println!();
    println!("  {}", "Hyperbolic".cyan());
    println!("    sinh(x)  cosh(x)  tanh(x)");
    println!();
    println!("  {}", "Roots".cyan());
    println!("    sqrt(x)  Square root     sqrt(16) = 4");
    println!("    cbrt(x)  Cube root       cbrt(27) = 3");
    println!();
    println!("  {}", "Rounding".cyan());
    println!("    floor(x)   Round down    floor(3.7) = 3");
    println!("    ceil(x)    Round up      ceil(3.2) = 4");
    println!("    round(x)   Round nearest round(3.5) = 4");
    println!("    trunc(x)   Truncate      trunc(-3.7) = -3");
    println!();
    println!("  {}", "Logarithms & Exponential".cyan());
    println!("    ln(x)      Natural log   ln(e) = 1");
    println!("    log2(x)    Log base 2    log2(8) = 3");
    println!("    log10(x)   Log base 10   log10(100) = 2");
    println!("    exp(x)     e^x           exp(1) = e");
    println!();
    println!("  {}", "Other".cyan());
    println!("    abs(x)     Absolute value  abs(-5) = 5");
    println!();

    println!("{}", "EXAMPLES".yellow());
    println!("  dx expr eval \"2 + 3 * 4\"");
    println!("  dx expr eval \"sqrt(16) + pi\"");
    println!("  dx expr eval \"sin(pi/2)\"");
    println!("  dx expr eval \"2^3^2\"           # Right-associative: 512");
    println!("  dx expr ast \"2 + 3\" --pretty");

    Ok(())
}
