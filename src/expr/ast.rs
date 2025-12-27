//! # Abstract Syntax Tree for Expressions
//!
//! Defines the AST nodes and evaluation logic.

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    // Comparison
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    // Logical
    And,
    Or,
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
            BinOp::Eq => "==",
            BinOp::Ne => "!=",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Le => "<=",
            BinOp::Ge => ">=",
            BinOp::And => "and",
            BinOp::Or => "or",
        }
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnaryOp {
    Neg,
    Not,
}

impl UnaryOp {
    /// Get the operator symbol for display
    pub fn symbol(&self) -> &'static str {
        match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "not",
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

    /// A function call: name(args...)
    FuncCall { name: String, args: Vec<Expr> },

    /// A conditional expression: if cond then a else b
    Conditional {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },

    /// A lambda expression: (params) => body
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
}

/// A user-defined function
#[derive(Debug, Clone)]
pub struct FuncDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: FuncBody,
}

/// Function body - either a single expression or multiple statements
#[derive(Debug, Clone)]
pub enum FuncBody {
    /// Single expression: def f(x) = x * 2
    Expr(Expr),
    /// Block with statements: def f(x) { ... }
    Block(Vec<Statement>),
}

/// A callable value (user function or lambda)
#[derive(Debug, Clone)]
pub struct Callable {
    pub params: Vec<String>,
    pub body: FuncBody,
    /// Captured environment for closures
    pub captures: HashMap<String, f64>,
}

/// A statement in a program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Statement {
    /// Variable assignment: name = expr
    Assignment { name: String, value: Expr },
    /// An expression (last one's value is the program result)
    Expression(Expr),
    /// Function definition: def name(params) = expr or def name(params) { ... }
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
}

/// A program is a sequence of statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Evaluation context holding variable bindings and user-defined functions
#[derive(Debug, Clone, Default)]
pub struct Context {
    variables: HashMap<String, f64>,
    functions: HashMap<String, Rc<Callable>>,
}

impl Context {
    /// Create a new empty context
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a context with predefined variables
    pub fn with_vars(vars: HashMap<String, f64>) -> Self {
        Self {
            variables: vars,
            functions: HashMap::new(),
        }
    }

    /// Set a variable value
    pub fn set(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }

    /// Get a variable value
    pub fn get(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }

    /// Define a user function
    pub fn define_function(&mut self, name: &str, callable: Callable) {
        self.functions.insert(name.to_string(), Rc::new(callable));
    }

    /// Get a user function
    pub fn get_function(&self, name: &str) -> Option<Rc<Callable>> {
        self.functions.get(name).cloned()
    }

    /// Store a lambda as a variable (internally stored as function)
    pub fn set_lambda(&mut self, name: &str, callable: Callable) {
        self.functions.insert(name.to_string(), Rc::new(callable));
    }

    /// Check if a name refers to a lambda/function
    pub fn is_callable(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Get all current variable values (for capturing closures)
    pub fn capture_env(&self) -> HashMap<String, f64> {
        self.variables.clone()
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

    /// Create a function call expression (single arg for backward compat)
    pub fn func_call(name: impl Into<String>, arg: Expr) -> Self {
        Expr::FuncCall {
            name: name.into(),
            args: vec![arg],
        }
    }

    /// Create a function call expression with multiple arguments
    pub fn func_call_multi(name: impl Into<String>, args: Vec<Expr>) -> Self {
        Expr::FuncCall {
            name: name.into(),
            args,
        }
    }

    /// Create a conditional expression
    pub fn conditional(condition: Expr, then_branch: Expr, else_branch: Expr) -> Self {
        Expr::Conditional {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }

    /// Create a lambda expression
    pub fn lambda(params: Vec<String>, body: Expr) -> Self {
        Expr::Lambda {
            params,
            body: Box::new(body),
        }
    }

    /// Evaluate the expression and return the result (no variables)
    pub fn eval(&self) -> Result<f64> {
        self.eval_with_context(&mut Context::new())
    }

    /// Evaluate the expression with a variable context
    pub fn eval_with_context(&self, ctx: &mut Context) -> Result<f64> {
        match self {
            Expr::Number { value } => Ok(*value),

            Expr::Constant { name } => match name.as_str() {
                "pi" => Ok(std::f64::consts::PI),
                "e" => Ok(std::f64::consts::E),
                "tau" => Ok(std::f64::consts::TAU),
                "true" => Ok(1.0),
                "false" => Ok(0.0),
                _ => bail!("Unknown constant: {}", name),
            },

            Expr::Variable { name } => {
                // Check if it's a callable (function/lambda stored as variable)
                if ctx.is_callable(name) {
                    bail!(
                        "'{}' is a function, not a value. Use {}() to call it.",
                        name,
                        name
                    );
                }
                ctx.get(name)
                    .ok_or_else(|| anyhow::anyhow!("Undefined variable: {}", name))
            }

            Expr::BinOp { op, left, right } => {
                // Short-circuit evaluation for logical operators
                match op {
                    BinOp::And => {
                        let l = left.eval_with_context(ctx)?;
                        if l == 0.0 {
                            return Ok(0.0);
                        }
                        let r = right.eval_with_context(ctx)?;
                        Ok(if r != 0.0 { 1.0 } else { 0.0 })
                    }
                    BinOp::Or => {
                        let l = left.eval_with_context(ctx)?;
                        if l != 0.0 {
                            return Ok(1.0);
                        }
                        let r = right.eval_with_context(ctx)?;
                        Ok(if r != 0.0 { 1.0 } else { 0.0 })
                    }
                    _ => {
                        let l = left.eval_with_context(ctx)?;
                        let r = right.eval_with_context(ctx)?;
                        eval_binop(*op, l, r)
                    }
                }
            }

            Expr::UnaryOp { op, expr } => {
                let val = expr.eval_with_context(ctx)?;
                match op {
                    UnaryOp::Neg => Ok(-val),
                    UnaryOp::Not => Ok(if val == 0.0 { 1.0 } else { 0.0 }),
                }
            }

            Expr::FuncCall { name, args } => {
                // First check for user-defined function or lambda
                if let Some(callable) = ctx.get_function(name) {
                    return call_user_function(&callable, args, ctx);
                }

                // Evaluate all arguments
                let mut vals = Vec::with_capacity(args.len());
                for arg in args {
                    vals.push(arg.eval_with_context(ctx)?);
                }

                eval_builtin_function(name, &vals)
            }

            Expr::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_val = condition.eval_with_context(ctx)?;
                if cond_val != 0.0 {
                    then_branch.eval_with_context(ctx)
                } else {
                    else_branch.eval_with_context(ctx)
                }
            }

            Expr::Lambda { params, body: _ } => {
                // Lambdas themselves don't evaluate to a number
                // They should be assigned to a variable or called immediately
                bail!(
                    "Lambda expression cannot be evaluated directly as a number. \
                     Assign it to a variable first: f = ({}) => ...",
                    params.join(", ")
                );
            }
        }
    }
}

/// Evaluate a binary operation
fn eval_binop(op: BinOp, l: f64, r: f64) -> Result<f64> {
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
        // Comparison operators return 1.0 (true) or 0.0 (false)
        BinOp::Eq => Ok(if (l - r).abs() < f64::EPSILON {
            1.0
        } else {
            0.0
        }),
        BinOp::Ne => Ok(if (l - r).abs() >= f64::EPSILON {
            1.0
        } else {
            0.0
        }),
        BinOp::Lt => Ok(if l < r { 1.0 } else { 0.0 }),
        BinOp::Gt => Ok(if l > r { 1.0 } else { 0.0 }),
        BinOp::Le => Ok(if l <= r { 1.0 } else { 0.0 }),
        BinOp::Ge => Ok(if l >= r { 1.0 } else { 0.0 }),
        // And/Or handled with short-circuit in eval_with_context
        BinOp::And | BinOp::Or => unreachable!(),
    }
}

/// Call a user-defined function or lambda
fn call_user_function(callable: &Callable, args: &[Expr], ctx: &mut Context) -> Result<f64> {
    if args.len() != callable.params.len() {
        bail!(
            "Function expects {} argument(s), got {}",
            callable.params.len(),
            args.len()
        );
    }

    // Evaluate arguments in the current context
    let mut arg_vals = Vec::with_capacity(args.len());
    for arg in args {
        arg_vals.push(arg.eval_with_context(ctx)?);
    }

    // Create a new context for function execution with captured environment
    let mut func_ctx = Context::new();

    // Copy captured variables (closure support)
    for (name, value) in &callable.captures {
        func_ctx.set(name, *value);
    }

    // Copy functions from parent context (for recursion)
    for (name, func) in &ctx.functions {
        func_ctx.functions.insert(name.clone(), func.clone());
    }

    // Bind parameters to arguments
    for (param, val) in callable.params.iter().zip(arg_vals.iter()) {
        func_ctx.set(param, *val);
    }

    // Evaluate the function body
    match &callable.body {
        FuncBody::Expr(expr) => {
            let result = expr.eval_with_context(&mut func_ctx)?;
            // Update captured variables in the original context (for mutable closures)
            for name in callable.captures.keys() {
                if let Some(new_val) = func_ctx.get(name) {
                    ctx.set(name, new_val);
                }
            }
            Ok(result)
        }
        FuncBody::Block(statements) => {
            let mut result = 0.0;
            for stmt in statements {
                result = stmt.eval(&mut func_ctx)?;
            }
            // Update captured variables
            for name in callable.captures.keys() {
                if let Some(new_val) = func_ctx.get(name) {
                    ctx.set(name, new_val);
                }
            }
            Ok(result)
        }
    }
}

/// Evaluate a built-in function
fn eval_builtin_function(name: &str, args: &[f64]) -> Result<f64> {
    match (name, args.len()) {
        // Single-argument functions
        ("sin", 1) => Ok(args[0].sin()),
        ("cos", 1) => Ok(args[0].cos()),
        ("tan", 1) => Ok(args[0].tan()),
        ("asin", 1) => Ok(args[0].asin()),
        ("acos", 1) => Ok(args[0].acos()),
        ("atan", 1) => Ok(args[0].atan()),
        ("sinh", 1) => Ok(args[0].sinh()),
        ("cosh", 1) => Ok(args[0].cosh()),
        ("tanh", 1) => Ok(args[0].tanh()),
        ("sqrt", 1) => {
            if args[0] < 0.0 {
                bail!("Square root of negative number");
            }
            Ok(args[0].sqrt())
        }
        ("cbrt", 1) => Ok(args[0].cbrt()),
        ("abs", 1) => Ok(args[0].abs()),
        ("floor", 1) => Ok(args[0].floor()),
        ("ceil", 1) => Ok(args[0].ceil()),
        ("round", 1) => Ok(args[0].round()),
        ("trunc", 1) => Ok(args[0].trunc()),
        ("exp", 1) => Ok(args[0].exp()),
        ("ln", 1) => {
            if args[0] <= 0.0 {
                bail!("Logarithm of non-positive number");
            }
            Ok(args[0].ln())
        }
        ("log2", 1) => {
            if args[0] <= 0.0 {
                bail!("Logarithm of non-positive number");
            }
            Ok(args[0].log2())
        }
        ("log10", 1) => {
            if args[0] <= 0.0 {
                bail!("Logarithm of non-positive number");
            }
            Ok(args[0].log10())
        }
        ("print", 1) => {
            println!("{}", args[0]);
            Ok(args[0])
        }
        ("sign", 1) => Ok(args[0].signum()),
        ("fract", 1) => Ok(args[0].fract()),

        // Two-argument functions
        ("max", 2) => Ok(args[0].max(args[1])),
        ("min", 2) => Ok(args[0].min(args[1])),
        ("pow", 2) => Ok(args[0].powf(args[1])),
        ("atan2", 2) => Ok(args[0].atan2(args[1])),
        ("hypot", 2) => Ok(args[0].hypot(args[1])),
        ("log", 2) => {
            // log(x, base)
            if args[0] <= 0.0 || args[1] <= 0.0 || args[1] == 1.0 {
                bail!("Invalid arguments for log(x, base)");
            }
            Ok(args[0].log(args[1]))
        }
        ("mod", 2) => {
            if args[1] == 0.0 {
                bail!("Modulo by zero");
            }
            Ok(args[0] % args[1])
        }

        // Three-argument functions
        ("clamp", 3) => {
            // clamp(x, min, max)
            Ok(args[0].clamp(args[1], args[2]))
        }
        ("lerp", 3) => {
            // lerp(a, b, t) = a + (b - a) * t
            Ok(args[0] + (args[1] - args[0]) * args[2])
        }

        // Variadic functions (work with any number of args)
        ("sum", _) => Ok(args.iter().sum()),
        ("avg", n) if n > 0 => Ok(args.iter().sum::<f64>() / n as f64),
        ("avg", 0) => bail!("avg() requires at least one argument"),

        // Wrong number of arguments for known functions
        (
            "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sinh" | "cosh" | "tanh" | "sqrt"
            | "cbrt" | "abs" | "floor" | "ceil" | "round" | "trunc" | "exp" | "ln" | "log2"
            | "log10" | "print" | "sign" | "fract",
            n,
        ) => {
            bail!("{}() expects 1 argument, got {}", name, n)
        }
        ("max" | "min" | "pow" | "atan2" | "hypot" | "log" | "mod", n) => {
            bail!("{}() expects 2 arguments, got {}", name, n)
        }
        ("clamp" | "lerp", n) => {
            bail!("{}() expects 3 arguments, got {}", name, n)
        }

        _ => bail!("Unknown function: {}", name),
    }
}

impl Statement {
    /// Evaluate the statement, potentially modifying context
    /// Returns the value of the expression (for assignments, the assigned value)
    pub fn eval(&self, ctx: &mut Context) -> Result<f64> {
        match self {
            Statement::Assignment { name, value } => {
                // Check for reserved names
                if matches!(name.as_str(), "pi" | "e" | "tau" | "true" | "false") {
                    bail!("Cannot assign to constant: {}", name);
                }
                if is_builtin_function_name(name) {
                    bail!("Cannot assign to built-in function name: {}", name);
                }

                // Check if we're assigning a lambda
                if let Expr::Lambda { params, body } = value {
                    let callable = Callable {
                        params: params.clone(),
                        body: FuncBody::Expr((**body).clone()),
                        captures: ctx.capture_env(),
                    };
                    ctx.set_lambda(name, callable);
                    return Ok(0.0); // Lambda assignment returns 0
                }

                let result = value.eval_with_context(ctx)?;
                ctx.set(name, result);
                Ok(result)
            }
            Statement::Expression(expr) => expr.eval_with_context(ctx),
            Statement::FuncDef { name, params, body } => {
                // Check reserved names
                if matches!(name.as_str(), "pi" | "e" | "tau" | "true" | "false") {
                    bail!("Cannot define function with reserved name: {}", name);
                }
                if is_builtin_function_name(name) {
                    bail!("Cannot redefine built-in function: {}", name);
                }

                let callable = Callable {
                    params: params.clone(),
                    body: FuncBody::Expr((**body).clone()),
                    captures: ctx.capture_env(),
                };
                ctx.define_function(name, callable);
                Ok(0.0) // Function definition returns 0
            }
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
pub fn is_builtin_function_name(name: &str) -> bool {
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
            | "log"
            | "log2"
            | "log10"
            | "print"
            | "sign"
            | "fract"
            | "max"
            | "min"
            | "pow"
            | "atan2"
            | "hypot"
            | "mod"
            | "clamp"
            | "lerp"
            | "sum"
            | "avg"
    )
}

/// Check if a name is a reserved keyword
pub fn is_keyword(name: &str) -> bool {
    matches!(
        name,
        "if" | "then" | "else" | "def" | "and" | "or" | "not" | "true" | "false"
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
    fn test_comparison_operators() {
        // 5 == 5 -> 1
        let expr = Expr::binop(BinOp::Eq, Expr::number(5.0), Expr::number(5.0));
        assert_eq!(expr.eval().unwrap(), 1.0);

        // 5 == 3 -> 0
        let expr = Expr::binop(BinOp::Eq, Expr::number(5.0), Expr::number(3.0));
        assert_eq!(expr.eval().unwrap(), 0.0);

        // 5 != 3 -> 1
        let expr = Expr::binop(BinOp::Ne, Expr::number(5.0), Expr::number(3.0));
        assert_eq!(expr.eval().unwrap(), 1.0);

        // 5 < 10 -> 1
        let expr = Expr::binop(BinOp::Lt, Expr::number(5.0), Expr::number(10.0));
        assert_eq!(expr.eval().unwrap(), 1.0);

        // 5 > 10 -> 0
        let expr = Expr::binop(BinOp::Gt, Expr::number(5.0), Expr::number(10.0));
        assert_eq!(expr.eval().unwrap(), 0.0);
    }

    #[test]
    fn test_logical_operators() {
        // 1 and 1 -> 1
        let expr = Expr::binop(BinOp::And, Expr::number(1.0), Expr::number(1.0));
        assert_eq!(expr.eval().unwrap(), 1.0);

        // 1 and 0 -> 0
        let expr = Expr::binop(BinOp::And, Expr::number(1.0), Expr::number(0.0));
        assert_eq!(expr.eval().unwrap(), 0.0);

        // 0 or 1 -> 1
        let expr = Expr::binop(BinOp::Or, Expr::number(0.0), Expr::number(1.0));
        assert_eq!(expr.eval().unwrap(), 1.0);

        // 0 or 0 -> 0
        let expr = Expr::binop(BinOp::Or, Expr::number(0.0), Expr::number(0.0));
        assert_eq!(expr.eval().unwrap(), 0.0);
    }

    #[test]
    fn test_not_operator() {
        // not 0 -> 1
        let expr = Expr::unary(UnaryOp::Not, Expr::number(0.0));
        assert_eq!(expr.eval().unwrap(), 1.0);

        // not 1 -> 0
        let expr = Expr::unary(UnaryOp::Not, Expr::number(1.0));
        assert_eq!(expr.eval().unwrap(), 0.0);

        // not 42 -> 0 (any non-zero is truthy)
        let expr = Expr::unary(UnaryOp::Not, Expr::number(42.0));
        assert_eq!(expr.eval().unwrap(), 0.0);
    }

    #[test]
    fn test_conditional() {
        // if 1 then 10 else 20 -> 10
        let expr = Expr::conditional(Expr::number(1.0), Expr::number(10.0), Expr::number(20.0));
        assert_eq!(expr.eval().unwrap(), 10.0);

        // if 0 then 10 else 20 -> 20
        let expr = Expr::conditional(Expr::number(0.0), Expr::number(10.0), Expr::number(20.0));
        assert_eq!(expr.eval().unwrap(), 20.0);
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

    #[test]
    fn test_multi_arg_functions() {
        // max(3, 5) -> 5
        let expr = Expr::func_call_multi("max", vec![Expr::number(3.0), Expr::number(5.0)]);
        assert_eq!(expr.eval().unwrap(), 5.0);

        // min(3, 5) -> 3
        let expr = Expr::func_call_multi("min", vec![Expr::number(3.0), Expr::number(5.0)]);
        assert_eq!(expr.eval().unwrap(), 3.0);

        // hypot(3, 4) -> 5
        let expr = Expr::func_call_multi("hypot", vec![Expr::number(3.0), Expr::number(4.0)]);
        assert_eq!(expr.eval().unwrap(), 5.0);

        // clamp(15, 0, 10) -> 10
        let expr = Expr::func_call_multi(
            "clamp",
            vec![Expr::number(15.0), Expr::number(0.0), Expr::number(10.0)],
        );
        assert_eq!(expr.eval().unwrap(), 10.0);
    }
}
