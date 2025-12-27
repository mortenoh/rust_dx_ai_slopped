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
//! dx expr eval                   # Multi-line input mode (Ctrl+D to eval)
//! dx expr run script.dx          # Run a script file
//! dx expr run -                  # Run from stdin
//! dx expr ast "2 + 3"            # Show AST as JSON
//! dx expr list                   # Show available functions
//! ```

use crate::cli::commands::expr::{ExprArgs, ExprCommand};
use crate::expr;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::io::{self, Read};

pub fn run(args: ExprArgs) -> Result<()> {
    match args.command {
        ExprCommand::Eval { expression } => cmd_eval(expression),
        ExprCommand::Run { file } => cmd_run(&file),
        ExprCommand::Ast { expression, pretty } => cmd_ast(&expression, pretty),
        ExprCommand::List => cmd_list(),
    }
}

/// Evaluate a math expression
fn cmd_eval(expression: Option<String>) -> Result<()> {
    let input = match expression {
        Some(expr) => expr,
        None => {
            // Read from stdin until EOF
            eprintln!(
                "{}",
                "Enter expression(s), press Ctrl+D when done:".dimmed()
            );
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    // Use eval_program to support variables even in single expressions
    let result = expr::eval_program(&input)?;
    print_result(result);
    Ok(())
}

/// Run a program from a file
fn cmd_run(file: &str) -> Result<()> {
    let input = if file == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        fs::read_to_string(file)?
    };

    let result = expr::eval_program(&input)?;
    print_result(result);
    Ok(())
}

/// Print a result, formatting integers nicely
fn print_result(result: f64) {
    if result.fract() == 0.0 && result.abs() < 1e15 {
        println!("{}", result as i64);
    } else {
        println!("{}", result);
    }
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
    println!("  **   Power (alt)        2 ** 10 = 1024");
    println!("  -x   Unary minus        -5 = -5");
    println!("  ()   Parentheses        (2 + 3) * 4 = 20");
    println!();

    println!("{}", "CONSTANTS".yellow());
    println!("  pi   {:<18} Circle ratio", "3.14159...");
    println!("  e    {:<18} Euler's number", "2.71828...");
    println!("  tau  {:<18} 2 * pi", "6.28318...");
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
    println!("    print(x)   Print and return print(42) = 42");
    println!();

    println!("{}", "VARIABLES".yellow());
    println!("  Assign with '=' and use by name:");
    println!("    x = 5");
    println!("    y = x + 3");
    println!("    y * 2        # Returns 16");
    println!();
    println!("  Reserved names (cannot be assigned):");
    println!("    Constants: pi, e, tau");
    println!("    Functions: sin, cos, sqrt, etc.");
    println!();

    println!("{}", "MULTI-LINE".yellow());
    println!("  Statements separated by newlines or semicolons:");
    println!("    x = 5; y = 10; x + y");
    println!();
    println!("  Or use multi-line input (Ctrl+D to evaluate):");
    println!("    dx expr eval");
    println!("    > x = 5");
    println!("    > y = x * 2");
    println!("    > y + 10");
    println!("    > ^D");
    println!("    20");
    println!();

    println!("{}", "EXAMPLES".yellow());
    println!("  {}", "Basic expressions".cyan());
    println!("    dx expr eval \"2 + 3 * 4\"");
    println!("    dx expr eval \"sqrt(16) + pi\"");
    println!("    dx expr eval \"sin(pi/2)\"");
    println!("    dx expr eval \"2^3^2\"           # Right-associative: 512");
    println!();
    println!("  {}", "With variables".cyan());
    println!("    dx expr eval \"x = 5; y = x + 3; y * 2\"");
    println!("    echo 'r=5; pi*r^2' | dx expr run -");
    println!();
    println!("  {}", "Run script files".cyan());
    println!("    dx expr run script.dx");
    println!("    dx expr run -                  # From stdin");
    println!();
    println!("  {}", "Debug with print".cyan());
    println!("    dx expr eval \"x=5; print(x); x*2\"");
    println!();
    println!("  {}", "AST output".cyan());
    println!("    dx expr ast \"2 + 3\" --pretty");

    Ok(())
}
