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

    // ========== OPERATORS ==========
    println!("{}", "ARITHMETIC OPERATORS".yellow());
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

    println!("{}", "COMPARISON OPERATORS".yellow());
    println!("  ==   Equal              5 == 5 → 1");
    println!("  !=   Not equal          5 != 3 → 1");
    println!("  <    Less than          3 < 5 → 1");
    println!("  >    Greater than       5 > 3 → 1");
    println!("  <=   Less or equal      5 <= 5 → 1");
    println!("  >=   Greater or equal   5 >= 3 → 1");
    println!();

    println!("{}", "LOGICAL OPERATORS".yellow());
    println!("  and  Logical AND        1 and 1 → 1");
    println!("  &&   Logical AND        1 && 0 → 0");
    println!("  or   Logical OR         0 or 1 → 1");
    println!("  ||   Logical OR         0 || 0 → 0");
    println!("  not  Logical NOT        not 0 → 1");
    println!("  !    Logical NOT        !1 → 0");
    println!();

    // ========== CONSTANTS ==========
    println!("{}", "CONSTANTS".yellow());
    println!("  pi     {:<18} Circle ratio", "3.14159...");
    println!("  e      {:<18} Euler's number", "2.71828...");
    println!("  tau    {:<18} 2 * pi", "6.28318...");
    println!("  true   {:<18} Boolean true", "1");
    println!("  false  {:<18} Boolean false", "0");
    println!();

    // ========== FUNCTIONS ==========
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
    println!("    log(x, b)  Log base b    log(8, 2) = 3");
    println!("    exp(x)     e^x           exp(1) = e");
    println!();

    println!("  {}", "Multi-Argument".cyan());
    println!("    max(a, b)      Maximum       max(3, 7) = 7");
    println!("    min(a, b)      Minimum       min(3, 7) = 3");
    println!("    pow(x, y)      Power         pow(2, 10) = 1024");
    println!("    atan2(y, x)    Two-arg atan  atan2(1, 1) = pi/4");
    println!("    hypot(a, b)    Hypotenuse    hypot(3, 4) = 5");
    println!("    mod(a, b)      Modulo        mod(10, 3) = 1");
    println!("    clamp(x, lo, hi) Constrain   clamp(15, 0, 10) = 10");
    println!("    lerp(a, b, t)  Interpolate   lerp(0, 10, 0.5) = 5");
    println!();

    println!("  {}", "Variadic".cyan());
    println!("    sum(...)   Sum all args    sum(1, 2, 3) = 6");
    println!("    avg(...)   Average         avg(2, 4, 6) = 4");
    println!();

    println!("  {}", "Other".cyan());
    println!("    abs(x)     Absolute value  abs(-5) = 5");
    println!("    sign(x)    Sign of number  sign(-5) = -1");
    println!("    fract(x)   Fractional part fract(3.75) = 0.75");
    println!("    print(x)   Print and return print(42) = 42");
    println!();

    // ========== CONDITIONALS ==========
    println!("{}", "CONDITIONALS".yellow());
    println!("  if-then-else expression:");
    println!("    if condition then expr else expr");
    println!();
    println!("  Examples:");
    println!("    if x > 0 then x else -x           # absolute value");
    println!("    if a > b then a else b            # max");
    println!("    if n <= 1 then 1 else n * f(n-1)  # factorial");
    println!();

    // ========== VARIABLES ==========
    println!("{}", "VARIABLES".yellow());
    println!("  Assign with '=' and use by name:");
    println!("    x = 5");
    println!("    y = x + 3");
    println!("    y * 2        # Returns 16");
    println!();
    println!("  Reserved names (cannot be assigned):");
    println!("    Constants: pi, e, tau, true, false");
    println!("    Functions: sin, cos, sqrt, etc.");
    println!();

    // ========== USER FUNCTIONS ==========
    println!("{}", "USER-DEFINED FUNCTIONS".yellow());
    println!("  Define with 'def':");
    println!("    def square(x) = x * x");
    println!("    def add(a, b) = a + b");
    println!("    def factorial(n) = if n <= 1 then 1 else n * factorial(n-1)");
    println!();
    println!("  Call like built-ins:");
    println!("    square(5)     # 25");
    println!("    add(3, 4)     # 7");
    println!("    factorial(5)  # 120");
    println!();

    // ========== LAMBDA FUNCTIONS ==========
    println!("{}", "LAMBDA EXPRESSIONS".yellow());
    println!("  Single parameter:");
    println!("    double = x => x * 2");
    println!("    double(5)              # 10");
    println!();
    println!("  Multiple parameters:");
    println!("    add = (a, b) => a + b");
    println!("    add(3, 4)              # 7");
    println!();
    println!("  No parameters:");
    println!("    answer = () => 42");
    println!("    answer()               # 42");
    println!();

    // ========== CLOSURES ==========
    println!("{}", "CLOSURES".yellow());
    println!("  Lambdas capture their environment:");
    println!("    multiplier = 10");
    println!("    scale = x => x * multiplier");
    println!("    scale(5)               # 50");
    println!();

    // ========== COMMENTS ==========
    println!("{}", "COMMENTS".yellow());
    println!("  # starts a comment to end of line:");
    println!("    x = 5    # set x");
    println!("    # this line is ignored");
    println!("    y = x * 2  # calculate y");
    println!();

    // ========== MULTI-LINE ==========
    println!("{}", "MULTI-LINE PROGRAMS".yellow());
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

    // ========== EXAMPLES ==========
    println!("{}", "EXAMPLES".yellow());
    println!();

    println!("  {}", "Basic expressions".cyan());
    println!("    dx expr eval \"2 + 3 * 4\"");
    println!("    dx expr eval \"sqrt(16) + pi\"");
    println!("    dx expr eval \"sin(pi/2)\"");
    println!("    dx expr eval \"2^3^2\"           # Right-associative: 512");
    println!();

    println!("  {}", "Comparisons and logic".cyan());
    println!("    dx expr eval \"5 > 3 and 2 < 4\"");
    println!("    dx expr eval \"if 5 > 3 then 100 else 200\"");
    println!();

    println!("  {}", "With variables".cyan());
    println!("    dx expr eval \"x = 5; y = x + 3; y * 2\"");
    println!("    echo 'r=5; pi*r^2' | dx expr run -");
    println!();

    println!("  {}", "User functions".cyan());
    println!("    dx expr eval \"def f(x) = x*x; f(5)\"");
    println!(
        "    dx expr eval \"def fib(n) = if n <= 1 then n else fib(n-1) + fib(n-2); fib(10)\""
    );
    println!();

    println!("  {}", "Lambdas".cyan());
    println!("    dx expr eval \"f = x => x * 2; f(5)\"");
    println!("    dx expr eval \"add = (a,b) => a+b; add(3,4)\"");
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
