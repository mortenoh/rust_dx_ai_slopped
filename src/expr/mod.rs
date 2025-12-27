//! # Expression Evaluation Library
//!
//! A full-featured arithmetic expression evaluator with support for variables,
//! user-defined functions, lambdas, closures, conditionals, and much more.
//!
//! ## Architecture
//!
//! ```text
//! Input String  →  Parser  →  AST (Expr)  →  Evaluator  →  Result
//! "2 + 3 * 4"      parse()    BinOp(...)     eval()        14.0
//! ```
//!
//! ## Features
//!
//! - **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `^`, `**`
//! - **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
//! - **Logical**: `and`, `or`, `not`, `&&`, `||`, `!`
//! - **Conditionals**: `if cond then a else b`
//! - **Variables**: `x = 5`
//! - **Functions**: `def f(x) = x * 2`
//! - **Lambdas**: `f = x => x * 2` or `f = (a, b) => a + b`
//! - **Closures**: Functions capture their environment
//! - **Comments**: `# comment to end of line`
//!
//! ## Constants
//!
//! - `pi` - π (3.14159...)
//! - `e` - Euler's number (2.71828...)
//! - `tau` - τ = 2π (6.28318...)
//! - `true` - 1.0
//! - `false` - 0.0
//!
//! ## Built-in Functions
//!
//! ### Single Argument
//! - Trigonometric: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
//! - Hyperbolic: `sinh`, `cosh`, `tanh`
//! - Roots: `sqrt`, `cbrt`
//! - Rounding: `floor`, `ceil`, `round`, `trunc`
//! - Other: `abs`, `sign`, `fract`, `exp`, `ln`, `log2`, `log10`, `print`
//!
//! ### Multiple Arguments
//! - Two args: `max`, `min`, `pow`, `atan2`, `hypot`, `log`, `mod`
//! - Three args: `clamp`, `lerp`
//! - Variadic: `sum`, `avg`
//!
//! ## Operator Precedence (lowest to highest)
//!
//! 1. Logical OR (`or`, `||`)
//! 2. Logical AND (`and`, `&&`)
//! 3. Equality (`==`, `!=`)
//! 4. Comparison (`<`, `>`, `<=`, `>=`)
//! 5. Addition, Subtraction (`+`, `-`)
//! 6. Multiplication, Division, Modulo (`*`, `/`, `%`)
//! 7. Power (`^`, `**`) - right-associative
//! 8. Unary (`-`, `not`, `!`)
//! 9. Function calls, parentheses
//!
//! ## Examples
//!
//! ```
//! use rust_cli_complete::expr::{eval, parse, eval_program, Context};
//!
//! // Basic arithmetic
//! assert_eq!(eval("2 + 3 * 4").unwrap(), 14.0);
//!
//! // Power and modulo
//! assert_eq!(eval("2 ^ 10").unwrap(), 1024.0);
//! assert_eq!(eval("17 % 5").unwrap(), 2.0);
//!
//! // Comparison and logical operators
//! assert_eq!(eval("5 > 3 and 2 < 4").unwrap(), 1.0);
//! assert_eq!(eval("not false").unwrap(), 1.0);
//!
//! // Conditional expression
//! assert_eq!(eval("if 5 > 3 then 100 else 200").unwrap(), 100.0);
//!
//! // Constants and functions
//! let result = eval("sin(pi / 2)").unwrap();
//! assert!((result - 1.0).abs() < 1e-10);
//!
//! // Multi-argument functions
//! assert_eq!(eval("max(3, 7)").unwrap(), 7.0);
//! assert_eq!(eval("clamp(15, 0, 10)").unwrap(), 10.0);
//!
//! // Get the AST for inspection
//! let ast = parse("sqrt(16) + pi").unwrap();
//! println!("{:?}", ast);
//!
//! // Serialize AST to JSON
//! let json = serde_json::to_string_pretty(&ast).unwrap();
//! println!("{}", json);
//!
//! // Multi-line programs with variables
//! let result = eval_program("x = 10; y = x * 2; y + 5").unwrap();
//! assert_eq!(result, 25.0);
//!
//! // Using a context with predefined variables
//! let mut ctx = Context::new();
//! ctx.set("radius", 5.0);
//! let area = rust_cli_complete::expr::eval_with_context("pi * radius ^ 2", &mut ctx).unwrap();
//! assert!((area - 78.53981633974483).abs() < 1e-10);
//! ```
//!
//! ## User-Defined Functions
//!
//! ```
//! use rust_cli_complete::expr::eval_program;
//!
//! // Define and use a function
//! let result = eval_program(r#"
//!     def square(x) = x * x
//!     square(5)
//! "#).unwrap();
//! assert_eq!(result, 25.0);
//!
//! // Recursive function
//! let result = eval_program(r#"
//!     def factorial(n) = if n <= 1 then 1 else n * factorial(n - 1)
//!     factorial(5)
//! "#).unwrap();
//! assert_eq!(result, 120.0);
//! ```
//!
//! ## Lambda Expressions
//!
//! ```
//! use rust_cli_complete::expr::eval_program;
//!
//! // Single parameter lambda
//! let result = eval_program(r#"
//!     double = x => x * 2
//!     double(5)
//! "#).unwrap();
//! assert_eq!(result, 10.0);
//!
//! // Multi-parameter lambda
//! let result = eval_program(r#"
//!     add = (a, b) => a + b
//!     add(3, 4)
//! "#).unwrap();
//! assert_eq!(result, 7.0);
//! ```
//!
//! ## Closures
//!
//! ```
//! use rust_cli_complete::expr::eval_program;
//!
//! // Lambda captures outer variable
//! let result = eval_program(r#"
//!     multiplier = 10
//!     scale = x => x * multiplier
//!     scale(5)
//! "#).unwrap();
//! assert_eq!(result, 50.0);
//! ```

mod ast;
mod parser;

pub use ast::{
    is_builtin_function_name, is_keyword, BinOp, Callable, Context, Expr, FuncBody, FuncDef,
    Program, Statement, UnaryOp,
};

use anyhow::Result;
use parser::Parser;

/// Parse an expression string into an AST.
///
/// This separates parsing from evaluation, allowing you to inspect
/// or transform the AST before evaluating.
///
/// # Examples
///
/// ```
/// use rust_cli_complete::expr::{parse, Expr, BinOp};
///
/// let ast = parse("1 + 2").unwrap();
/// match ast {
///     Expr::BinOp { op: BinOp::Add, .. } => println!("It's an addition!"),
///     _ => {}
/// }
/// ```
pub fn parse(input: &str) -> Result<Expr> {
    let mut parser = Parser::new(input);
    parser.parse()
}

/// Parse and evaluate an arithmetic expression string.
///
/// This is a convenience function that parses and evaluates in one step.
///
/// # Arguments
///
/// * `input` - An arithmetic expression string
///
/// # Returns
///
/// The evaluated result as `f64`, or an error if parsing/evaluation fails.
///
/// # Errors
///
/// Returns an error if:
/// - The expression is empty or malformed
/// - Division by zero occurs
/// - Parentheses are unbalanced
/// - Unknown characters are encountered
///
/// # Examples
///
/// ```
/// use rust_cli_complete::expr::eval;
///
/// let result = eval("(1 + 2) * 3").unwrap();
/// assert_eq!(result, 9.0);
/// ```
pub fn eval(input: &str) -> Result<f64> {
    parse(input)?.eval()
}

/// Parse a multi-line program into a Program AST.
///
/// Programs consist of statements separated by newlines or semicolons.
/// Statements can be variable assignments (`x = 5`), function definitions
/// (`def f(x) = x * 2`), or expressions.
///
/// # Examples
///
/// ```
/// use rust_cli_complete::expr::parse_program;
///
/// let program = parse_program("x = 5; y = x + 3; y * 2").unwrap();
/// assert_eq!(program.statements.len(), 3);
/// ```
pub fn parse_program(input: &str) -> Result<Program> {
    let mut parser = Parser::new(input);
    parser.parse_program()
}

/// Parse and evaluate a multi-line program.
///
/// Programs consist of statements separated by newlines or semicolons.
/// The result is the value of the last expression.
///
/// # Examples
///
/// ```
/// use rust_cli_complete::expr::eval_program;
///
/// // Using semicolons
/// assert_eq!(eval_program("x = 5; y = x + 3; y * 2").unwrap(), 16.0);
///
/// // Using newlines
/// let program = r#"
///     radius = 5
///     pi * radius ^ 2
/// "#;
/// let area = eval_program(program).unwrap();
/// assert!((area - 78.53981633974483).abs() < 1e-10);
/// ```
pub fn eval_program(input: &str) -> Result<f64> {
    parse_program(input)?.eval()
}

/// Evaluate a program with a given context.
///
/// This allows you to pass in predefined variables and retrieve
/// variables set during evaluation.
///
/// # Examples
///
/// ```
/// use rust_cli_complete::expr::{eval_with_context, Context};
///
/// let mut ctx = Context::new();
/// ctx.set("x", 10.0);
///
/// // Use predefined variable
/// let result = eval_with_context("y = x * 2; y + 5", &mut ctx).unwrap();
/// assert_eq!(result, 25.0);
///
/// // Context now has 'y' set
/// assert_eq!(ctx.get("y"), Some(20.0));
/// ```
pub fn eval_with_context(input: &str, ctx: &mut Context) -> Result<f64> {
    parse_program(input)?.eval_with_context(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_api() {
        assert_eq!(eval("1 + 1").unwrap(), 2.0);
        assert_eq!(eval("2 * 3 + 4").unwrap(), 10.0);
        assert_eq!(eval("(5 - 3) * 2").unwrap(), 4.0);
    }

    #[test]
    fn test_parse_api() {
        let ast = parse("1 + 2").unwrap();
        assert!(matches!(ast, Expr::BinOp { op: BinOp::Add, .. }));
    }

    #[test]
    fn test_eval_errors() {
        assert!(eval("").is_err());
        assert!(eval("1 / 0").is_err());
        assert!(eval("hello").is_err());
    }

    #[test]
    fn test_ast_serialization() {
        let ast = parse("2 + 3 * 4").unwrap();
        let json = serde_json::to_string(&ast).unwrap();

        // Verify it's valid JSON
        let _: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Can deserialize back
        let parsed: Expr = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.eval().unwrap(), 14.0);
    }

    #[test]
    fn test_comparison_operators() {
        assert_eq!(eval("5 > 3").unwrap(), 1.0);
        assert_eq!(eval("5 < 3").unwrap(), 0.0);
        assert_eq!(eval("5 == 5").unwrap(), 1.0);
        assert_eq!(eval("5 != 5").unwrap(), 0.0);
    }

    #[test]
    fn test_logical_operators() {
        assert_eq!(eval("1 and 1").unwrap(), 1.0);
        assert_eq!(eval("1 or 0").unwrap(), 1.0);
        assert_eq!(eval("not 0").unwrap(), 1.0);
    }

    #[test]
    fn test_conditional() {
        assert_eq!(eval("if 1 then 10 else 20").unwrap(), 10.0);
        assert_eq!(eval("if 0 then 10 else 20").unwrap(), 20.0);
    }

    #[test]
    fn test_multi_arg_functions() {
        assert_eq!(eval("max(3, 7)").unwrap(), 7.0);
        assert_eq!(eval("min(3, 7)").unwrap(), 3.0);
        assert_eq!(eval("clamp(15, 0, 10)").unwrap(), 10.0);
    }

    #[test]
    fn test_user_defined_function() {
        let result = eval_program("def double(x) = x * 2\ndouble(5)").unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_lambda() {
        let result = eval_program("f = x => x * 2\nf(5)").unwrap();
        assert_eq!(result, 10.0);
    }
}
