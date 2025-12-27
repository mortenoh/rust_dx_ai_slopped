//! # Abstract Syntax Tree for Expressions
//!
//! Defines the AST nodes and evaluation logic.

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

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

impl Expr {
    /// Create a number expression
    pub fn number(value: f64) -> Self {
        Expr::Number { value }
    }

    /// Create a constant expression
    pub fn constant(name: impl Into<String>) -> Self {
        Expr::Constant { name: name.into() }
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

    /// Evaluate the expression and return the result
    pub fn eval(&self) -> Result<f64> {
        match self {
            Expr::Number { value } => Ok(*value),

            Expr::Constant { name } => match name.as_str() {
                "pi" => Ok(std::f64::consts::PI),
                "e" => Ok(std::f64::consts::E),
                "tau" => Ok(std::f64::consts::TAU),
                _ => bail!("Unknown constant: {}", name),
            },

            Expr::BinOp { op, left, right } => {
                let l = left.eval()?;
                let r = right.eval()?;

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
                let val = expr.eval()?;
                match op {
                    UnaryOp::Neg => Ok(-val),
                }
            }

            Expr::FuncCall { name, arg } => {
                let val = arg.eval()?;
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
                    _ => bail!("Unknown function: {}", name),
                }
            }
        }
    }
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
