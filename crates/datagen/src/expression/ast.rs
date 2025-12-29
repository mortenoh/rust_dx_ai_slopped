//! Abstract Syntax Tree types for the expression DSL.
//!
//! Supports Datafaker-style expressions like:
//! - `#{Name.firstName}` - Provider call
//! - `#{regexify '[A-Z]{3}-[0-9]{4}'}` - Regex generation
//! - `#{options.option 'A','B','C'}` - Random choice
//! - `#{Number.numberBetween '1','100'}` - Parameterized call
//! - `#{templatify '###-###','#','0-9'}` - Character replacement

use std::fmt;

/// A complete expression template that may contain literal text and expressions.
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    pub parts: Vec<TemplatePart>,
}

impl Template {
    pub fn new(parts: Vec<TemplatePart>) -> Self {
        Self { parts }
    }

    pub fn literal(text: &str) -> Self {
        Self {
            parts: vec![TemplatePart::Literal(text.to_string())],
        }
    }
}

/// A part of a template - either literal text or an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum TemplatePart {
    /// Literal text that is copied as-is.
    Literal(String),
    /// An expression to be evaluated: `#{...}`.
    Expression(Expression),
}

/// An expression that can be evaluated to produce a value.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A provider method call: `Name.firstName`, `Address.city`.
    ProviderCall(ProviderCall),
    /// A function call: `regexify '[A-Z]{3}'`, `options.option 'A','B'`.
    FunctionCall(FunctionCall),
    /// A literal value (string, number, boolean).
    Literal(Literal),
    /// A conditional expression: `if condition then_value else_value`.
    Conditional(Box<Conditional>),
}

/// A call to a data provider: `Provider.method` or `Provider.method 'arg1','arg2'`.
#[derive(Debug, Clone, PartialEq)]
pub struct ProviderCall {
    /// The provider name (e.g., "Name", "Address", "Commerce").
    pub provider: String,
    /// The method name (e.g., "firstName", "city", "productName").
    pub method: String,
    /// Optional arguments to the method.
    pub args: Vec<Argument>,
}

impl ProviderCall {
    pub fn new(provider: &str, method: &str) -> Self {
        Self {
            provider: provider.to_string(),
            method: method.to_string(),
            args: Vec::new(),
        }
    }

    pub fn with_args(mut self, args: Vec<Argument>) -> Self {
        self.args = args;
        self
    }
}

/// A function call: `functionName 'arg1','arg2'`.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    /// The function name (e.g., "regexify", "templatify", "options.option").
    pub name: String,
    /// Arguments to the function.
    pub args: Vec<Argument>,
}

impl FunctionCall {
    pub fn new(name: &str, args: Vec<Argument>) -> Self {
        Self {
            name: name.to_string(),
            args,
        }
    }
}

/// An argument to a function or provider method.
#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    /// A string literal: `'value'` or `"value"`.
    String(String),
    /// A numeric literal: `42`, `3.14`.
    Number(f64),
    /// A boolean literal: `true`, `false`.
    Boolean(bool),
    /// A nested expression.
    Expression(Box<Expression>),
}

impl Argument {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Argument::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Argument::Number(n) => Some(*n as i64),
            Argument::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Argument::Number(n) => Some(*n),
            Argument::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        match self {
            Argument::Number(n) if *n >= 0.0 => Some(*n as usize),
            Argument::String(s) => s.parse().ok(),
            _ => None,
        }
    }
}

/// A literal value.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Null => write!(f, "null"),
        }
    }
}

/// A conditional expression: `if condition then else`.
#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    /// The condition to evaluate.
    pub condition: Expression,
    /// Value if condition is true.
    pub then_branch: Expression,
    /// Value if condition is false.
    pub else_branch: Expression,
}

impl Conditional {
    pub fn new(condition: Expression, then_branch: Expression, else_branch: Expression) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_call() {
        let call = ProviderCall::new("Name", "firstName");
        assert_eq!(call.provider, "Name");
        assert_eq!(call.method, "firstName");
        assert!(call.args.is_empty());
    }

    #[test]
    fn test_provider_call_with_args() {
        let call = ProviderCall::new("Name", "firstName")
            .with_args(vec![Argument::String("en".to_string())]);
        assert_eq!(call.args.len(), 1);
    }

    #[test]
    fn test_function_call() {
        let call = FunctionCall::new("regexify", vec![Argument::String("[A-Z]{3}".to_string())]);
        assert_eq!(call.name, "regexify");
        assert_eq!(call.args.len(), 1);
    }

    #[test]
    fn test_argument_conversions() {
        let num = Argument::Number(42.0);
        assert_eq!(num.as_i64(), Some(42));
        assert_eq!(num.as_f64(), Some(42.0));
        assert_eq!(num.as_usize(), Some(42));

        let str_num = Argument::String("100".to_string());
        assert_eq!(str_num.as_i64(), Some(100));
        assert_eq!(str_num.as_string(), Some("100"));
    }

    #[test]
    fn test_template() {
        let template = Template::new(vec![
            TemplatePart::Literal("Hello, ".to_string()),
            TemplatePart::Expression(Expression::ProviderCall(ProviderCall::new(
                "Name",
                "firstName",
            ))),
            TemplatePart::Literal("!".to_string()),
        ]);
        assert_eq!(template.parts.len(), 3);
    }
}
