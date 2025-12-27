//! # Abstract Syntax Tree for Expressions
//!
//! Defines the AST nodes and evaluation logic.

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl BinOp {
    /// Get the operator symbol for display
    pub fn symbol(&self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Pow => "^",
        }
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnaryOp {
    Neg,
}

impl UnaryOp {
    /// Get the operator symbol for display
    pub fn symbol(&self) -> &'static str {
        match self {
            UnaryOp::Neg => "-",
        }
    }
}

/// Expression AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Expr {
    /// A numeric literal
    Number { value: f64 },

    /// A named constant (pi, e, tau)
    Constant { name: String },

    /// A variable reference
    Variable { name: String },

    /// A binary operation: left op right
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    /// A unary operation: op expr
    UnaryOp { op: UnaryOp, expr: Box<Expr> },

    /// A function call: name(arg)
    FuncCall { name: String, arg: Box<Expr> },
}

/// A statement in a program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Statement {
    /// Variable assignment: name = expr
    Assignment { name: String, value: Expr },
    /// An expression (last one's value is the program result)
    Expression(Expr),
}

/// A program is a sequence of statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Evaluation context holding variable bindings
#[derive(Debug, Clone, Default)]
pub struct Context {
    variables: HashMap<String, f64>,
}

impl Context {
    /// Create a new empty context
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a context with predefined variables
    pub fn with_vars(vars: HashMap<String, f64>) -> Self {
        Self { variables: vars }
    }

    /// Set a variable value
    pub fn set(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }

    /// Get a variable value
    pub fn get(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }
}

impl Expr {
    /// Create a number expression
    pub fn number(value: f64) -> Self {
        Expr::Number { value }
    }

    /// Create a constant expression
    pub fn constant(name: impl Into<String>) -> Self {
        Expr::Constant { name: name.into() }
    }

    /// Create a variable expression
    pub fn variable(name: impl Into<String>) -> Self {
        Expr::Variable { name: name.into() }
    }

    /// Create a binary operation expression
    pub fn binop(op: BinOp, left: Expr, right: Expr) -> Self {
        Expr::BinOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a unary operation expression
    pub fn unary(op: UnaryOp, expr: Expr) -> Self {
        Expr::UnaryOp {
            op,
            expr: Box::new(expr),
        }
    }

    /// Create a function call expression
    pub fn func_call(name: impl Into<String>, arg: Expr) -> Self {
        Expr::FuncCall {
            name: name.into(),
            arg: Box::new(arg),
        }
    }

    /// Evaluate the expression and return the result (no variables)
    pub fn eval(&self) -> Result<f64> {
        self.eval_with_context(&Context::new())
    }

    /// Evaluate the expression with a variable context
    pub fn eval_with_context(&self, ctx: &Context) -> Result<f64> {
        match self {
            Expr::Number { value } => Ok(*value),

            Expr::Constant { name } => match name.as_str() {
                "pi" => Ok(std::f64::consts::PI),
                "e" => Ok(std::f64::consts::E),
                "tau" => Ok(std::f64::consts::TAU),
                _ => bail!("Unknown constant: {}", name),
            },

            Expr::Variable { name } => ctx
                .get(name)
                .ok_or_else(|| anyhow::anyhow!("Undefined variable: {}", name)),

            Expr::BinOp { op, left, right } => {
                let l = left.eval_with_context(ctx)?;
                let r = right.eval_with_context(ctx)?;

                match op {
                    BinOp::Add => Ok(l + r),
                    BinOp::Sub => Ok(l - r),
                    BinOp::Mul => Ok(l * r),
                    BinOp::Div => {
                        if r == 0.0 {
                            bail!("Division by zero");
                        }
                        Ok(l / r)
                    }
                    BinOp::Mod => {
                        if r == 0.0 {
                            bail!("Modulo by zero");
                        }
                        Ok(l % r)
                    }
                    BinOp::Pow => Ok(l.powf(r)),
                }
            }

            Expr::UnaryOp { op, expr } => {
                let val = expr.eval_with_context(ctx)?;
                match op {
                    UnaryOp::Neg => Ok(-val),
                }
            }

            Expr::FuncCall { name, arg } => {
                let val = arg.eval_with_context(ctx)?;
                match name.as_str() {
                    "sin" => Ok(val.sin()),
                    "cos" => Ok(val.cos()),
                    "tan" => Ok(val.tan()),
                    "asin" => Ok(val.asin()),
                    "acos" => Ok(val.acos()),
                    "atan" => Ok(val.atan()),
                    "sinh" => Ok(val.sinh()),
                    "cosh" => Ok(val.cosh()),
                    "tanh" => Ok(val.tanh()),
                    "sqrt" => {
                        if val < 0.0 {
                            bail!("Square root of negative number");
                        }
                        Ok(val.sqrt())
                    }
                    "cbrt" => Ok(val.cbrt()),
                    "abs" => Ok(val.abs()),
                    "floor" => Ok(val.floor()),
                    "ceil" => Ok(val.ceil()),
                    "round" => Ok(val.round()),
                    "trunc" => Ok(val.trunc()),
                    "exp" => Ok(val.exp()),
                    "ln" => {
                        if val <= 0.0 {
                            bail!("Logarithm of non-positive number");
                        }
                        Ok(val.ln())
                    }
                    "log2" => {
                        if val <= 0.0 {
                            bail!("Logarithm of non-positive number");
                        }
                        Ok(val.log2())
                    }
                    "log10" => {
                        if val <= 0.0 {
                            bail!("Logarithm of non-positive number");
                        }
                        Ok(val.log10())
                    }
                    "print" => {
                        println!("{}", val);
                        Ok(val)
                    }
                    _ => bail!("Unknown function: {}", name),
                }
            }
        }
    }
}

impl Statement {
    /// Evaluate the statement, potentially modifying context
    /// Returns the value of the expression (for assignments, the assigned value)
    pub fn eval(&self, ctx: &mut Context) -> Result<f64> {
        match self {
            Statement::Assignment { name, value } => {
                // Check for reserved names
                if matches!(name.as_str(), "pi" | "e" | "tau") {
                    bail!("Cannot assign to constant: {}", name);
                }
                if is_function_name(name) {
                    bail!("Cannot assign to function name: {}", name);
                }
                let result = value.eval_with_context(ctx)?;
                ctx.set(name, result);
                Ok(result)
            }
            Statement::Expression(expr) => expr.eval_with_context(ctx),
        }
    }
}

impl Program {
    /// Evaluate all statements and return the last expression's value
    pub fn eval(&self) -> Result<f64> {
        self.eval_with_context(&mut Context::new())
    }

    /// Evaluate all statements with a given context
    pub fn eval_with_context(&self, ctx: &mut Context) -> Result<f64> {
        if self.statements.is_empty() {
            bail!("Empty program");
        }

        let mut result = 0.0;
        for stmt in &self.statements {
            result = stmt.eval(ctx)?;
        }
        Ok(result)
    }
}

/// Check if a name is a built-in function
fn is_function_name(name: &str) -> bool {
    matches!(
        name,
        "sin"
            | "cos"
            | "tan"
            | "asin"
            | "acos"
            | "atan"
            | "sinh"
            | "cosh"
            | "tanh"
            | "sqrt"
            | "cbrt"
            | "abs"
            | "floor"
            | "ceil"
            | "round"
            | "trunc"
            | "exp"
            | "ln"
            | "log2"
            | "log10"
            | "print"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let expr = Expr::number(42.0);
        assert_eq!(expr.eval().unwrap(), 42.0);
    }

    #[test]
    fn test_binop() {
        // 2 + 3
        let expr = Expr::binop(BinOp::Add, Expr::number(2.0), Expr::number(3.0));
        assert_eq!(expr.eval().unwrap(), 5.0);

        // 10 - 4
        let expr = Expr::binop(BinOp::Sub, Expr::number(10.0), Expr::number(4.0));
        assert_eq!(expr.eval().unwrap(), 6.0);

        // 3 * 4
        let expr = Expr::binop(BinOp::Mul, Expr::number(3.0), Expr::number(4.0));
        assert_eq!(expr.eval().unwrap(), 12.0);

        // 15 / 3
        let expr = Expr::binop(BinOp::Div, Expr::number(15.0), Expr::number(3.0));
        assert_eq!(expr.eval().unwrap(), 5.0);
    }

    #[test]
    fn test_unary() {
        // -5
        let expr = Expr::unary(UnaryOp::Neg, Expr::number(5.0));
        assert_eq!(expr.eval().unwrap(), -5.0);

        // --5 (double negation)
        let expr = Expr::unary(UnaryOp::Neg, Expr::unary(UnaryOp::Neg, Expr::number(5.0)));
        assert_eq!(expr.eval().unwrap(), 5.0);
    }

    #[test]
    fn test_nested() {
        // (2 + 3) * 4
        let expr = Expr::binop(
            BinOp::Mul,
            Expr::binop(BinOp::Add, Expr::number(2.0), Expr::number(3.0)),
            Expr::number(4.0),
        );
        assert_eq!(expr.eval().unwrap(), 20.0);
    }

    #[test]
    fn test_division_by_zero() {
        let expr = Expr::binop(BinOp::Div, Expr::number(1.0), Expr::number(0.0));
        assert!(expr.eval().is_err());
    }
}
