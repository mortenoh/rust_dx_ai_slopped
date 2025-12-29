//! Built-in functions for the expression DSL.
//!
//! Provides functions like regexify, templatify, options.option, etc.

mod number;
mod options;
mod string;

use rand::Rng;

use crate::expression::ast::Argument;

/// Function error type.
#[derive(Debug, Clone)]
pub struct FunctionError {
    pub message: String,
}

impl FunctionError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn unknown_function(name: &str) -> Self {
        Self::new(&format!("Unknown function: {}", name))
    }

    pub fn wrong_arg_count(name: &str, expected: usize, got: usize) -> Self {
        Self::new(&format!(
            "Function '{}' expects {} argument(s), got {}",
            name, expected, got
        ))
    }

    pub fn wrong_arg_type(name: &str, index: usize, expected: &str) -> Self {
        Self::new(&format!(
            "Function '{}' argument {} must be {}",
            name, index, expected
        ))
    }
}

impl std::fmt::Display for FunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function error: {}", self.message)
    }
}

impl std::error::Error for FunctionError {}

/// Call a built-in function by name.
pub fn call_function<R: Rng + ?Sized>(
    rng: &mut R,
    name: &str,
    args: &[Argument],
) -> Result<String, FunctionError> {
    match name {
        // String functions
        "regexify" => string::regexify(rng, args),
        "templatify" => string::templatify(rng, args),
        "exemplify" => string::exemplify(rng, args),
        "bothify" => string::bothify(rng, args),
        "letterify" => string::letterify(rng, args),
        "numerify" => string::numerify(rng, args),
        "uppercase" => string::uppercase(args),
        "lowercase" => string::lowercase(args),
        "capitalize" => string::capitalize(args),

        // Number functions
        "number" | "Number.number" => number::number(rng, args),
        "Number.between" | "numberBetween" => number::number_between(rng, args),
        "Number.decimal" | "decimal" => number::decimal(rng, args),
        "Number.positive" | "positive" => number::positive(rng, args),
        "Number.negative" | "negative" => number::negative(rng, args),

        // Options functions
        "options.option" | "option" => options::option(rng, args),
        "options.weighted" | "weighted" => options::weighted(rng, args),

        _ => Err(FunctionError::unknown_function(name)),
    }
}

/// Get the list of available functions.
pub fn available_functions() -> Vec<&'static str> {
    vec![
        "regexify",
        "templatify",
        "exemplify",
        "bothify",
        "letterify",
        "numerify",
        "uppercase",
        "lowercase",
        "capitalize",
        "number",
        "Number.between",
        "Number.decimal",
        "Number.positive",
        "Number.negative",
        "options.option",
        "options.weighted",
    ]
}
