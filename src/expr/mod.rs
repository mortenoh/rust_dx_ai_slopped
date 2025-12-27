//! # Expression Evaluation Library
//!
//! A full-featured arithmetic expression evaluator using recursive descent parsing
//! and an Abstract Syntax Tree (AST).
//!
//! ## Architecture
//!
//! ```text
//! Input String  →  Parser  →  AST (Expr)  →  Evaluator  →  Result
//! "2 + 3 * 4"      parse()    BinOp(...)     eval()        14.0
//! ```
//!
//! ## Supported Operations
//!
//! - Addition: `+`
//! - Subtraction: `-`
//! - Multiplication: `*`
//! - Division: `/`
//! - Modulo: `%`
//! - Power: `^` (right-associative)
//! - Parentheses: `(` `)`
//! - Unary minus: `-5`
//!
//! ## Constants
//!
//! - `pi` - π (3.14159...)
//! - `e` - Euler's number (2.71828...)
//! - `tau` - τ = 2π (6.28318...)
//!
//! ## Functions
//!
//! - Trigonometric: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
//! - Hyperbolic: `sinh`, `cosh`, `tanh`
//! - Roots: `sqrt`, `cbrt`
//! - Rounding: `floor`, `ceil`, `round`, `trunc`
//! - Other: `abs`, `exp`, `ln`, `log2`, `log10`, `print`
//!
//! ## Operator Precedence (lowest to highest)
//!
//! 1. Addition, Subtraction (`+`, `-`)
//! 2. Multiplication, Division, Modulo (`*`, `/`, `%`)
//! 3. Power (`^`) - right-associative
//! 4. Unary minus (`-`)
//! 5. Function calls, parentheses
//!
//! ## Variables and Multi-line Programs
//!
//! The evaluator supports variables and multi-line programs:
//!
//! ```
//! use rust_cli_complete::expr::eval_program;
//!
//! let program = r#"
//!     x = 5
//!     y = x + 3
//!     y * 2
//! "#;
//! assert_eq!(eval_program(program).unwrap(), 16.0);
//! ```
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
//! // Constants and functions
//! let result = eval("sin(pi / 2)").unwrap();
//! assert!((result - 1.0).abs() < 1e-10);
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

mod ast;
mod parser;

pub use ast::{BinOp, Context, Expr, Program, Statement, UnaryOp};

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
/// Statements can be variable assignments (`x = 5`) or expressions.
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
}
