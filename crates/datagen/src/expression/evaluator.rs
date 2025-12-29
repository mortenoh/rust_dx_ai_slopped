//! Evaluator for the expression DSL.
//!
//! Evaluates parsed templates to produce generated strings.

use rand::Rng;

use super::ast::{
    Argument, Expression, FunctionCall, Literal, ProviderCall, Template, TemplatePart,
};
use super::functions::{call_function, FunctionError};
use super::providers::{call_provider, ProviderError};

/// Evaluation error type.
#[derive(Debug, Clone)]
pub struct EvalError {
    pub message: String,
}

impl EvalError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Evaluation error: {}", self.message)
    }
}

impl std::error::Error for EvalError {}

impl From<FunctionError> for EvalError {
    fn from(err: FunctionError) -> Self {
        EvalError::new(&err.message)
    }
}

impl From<ProviderError> for EvalError {
    fn from(err: ProviderError) -> Self {
        EvalError::new(&err.message)
    }
}

/// Evaluator for expression templates.
pub struct Evaluator<'a, R: ?Sized> {
    rng: &'a mut R,
}

impl<'a, R: Rng + ?Sized> Evaluator<'a, R> {
    /// Create a new evaluator with the given RNG.
    pub fn new(rng: &'a mut R) -> Self {
        Self { rng }
    }

    /// Evaluate a template and return the generated string.
    pub fn evaluate(&mut self, template: &Template) -> Result<String, EvalError> {
        let mut result = String::new();

        for part in &template.parts {
            match part {
                TemplatePart::Literal(text) => {
                    result.push_str(text);
                }
                TemplatePart::Expression(expr) => {
                    let value = self.evaluate_expression(expr)?;
                    result.push_str(&value);
                }
            }
        }

        Ok(result)
    }

    /// Evaluate a single expression.
    fn evaluate_expression(&mut self, expr: &Expression) -> Result<String, EvalError> {
        match expr {
            Expression::ProviderCall(call) => self.evaluate_provider_call(call),
            Expression::FunctionCall(call) => self.evaluate_function_call(call),
            Expression::Literal(lit) => Ok(self.evaluate_literal(lit)),
            Expression::Conditional(cond) => {
                // Evaluate condition - for now, treat non-empty strings and non-zero numbers as true
                let condition_value = self.evaluate_expression(&cond.condition)?;
                let is_true = !condition_value.is_empty()
                    && condition_value != "0"
                    && condition_value.to_lowercase() != "false";

                if is_true {
                    self.evaluate_expression(&cond.then_branch)
                } else {
                    self.evaluate_expression(&cond.else_branch)
                }
            }
        }
    }

    /// Evaluate a provider method call.
    fn evaluate_provider_call(&mut self, call: &ProviderCall) -> Result<String, EvalError> {
        // Resolve any expression arguments first
        let resolved_args = self.resolve_arguments(&call.args)?;
        call_provider(self.rng, &call.provider, &call.method, &resolved_args).map_err(Into::into)
    }

    /// Evaluate a function call.
    fn evaluate_function_call(&mut self, call: &FunctionCall) -> Result<String, EvalError> {
        // Resolve any expression arguments first
        let resolved_args = self.resolve_arguments(&call.args)?;
        call_function(self.rng, &call.name, &resolved_args).map_err(Into::into)
    }

    /// Evaluate a literal value.
    fn evaluate_literal(&self, lit: &Literal) -> String {
        lit.to_string()
    }

    /// Resolve arguments, evaluating any nested expressions.
    fn resolve_arguments(&mut self, args: &[Argument]) -> Result<Vec<Argument>, EvalError> {
        let mut resolved = Vec::with_capacity(args.len());

        for arg in args {
            match arg {
                Argument::Expression(expr) => {
                    // Evaluate the nested expression and convert to a string argument
                    let value = self.evaluate_expression(expr)?;
                    resolved.push(Argument::String(value));
                }
                other => {
                    resolved.push(other.clone());
                }
            }
        }

        Ok(resolved)
    }
}

/// Convenience function to evaluate a template string.
pub fn evaluate<R: Rng + ?Sized>(rng: &mut R, input: &str) -> Result<String, EvalError> {
    use super::parser::{ParseError, Parser};

    let template = Parser::parse(input).map_err(|e: ParseError| EvalError::new(&e.message))?;
    let mut evaluator = Evaluator::new(rng);
    evaluator.evaluate(&template)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn test_rng() -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(42)
    }

    #[test]
    fn test_evaluate_literal() {
        let mut rng = test_rng();
        let result = evaluate(&mut rng, "Hello, World!").unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_evaluate_provider_call() {
        let mut rng = test_rng();
        let result = evaluate(&mut rng, "#{Name.firstName}").unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_evaluate_mixed() {
        let mut rng = test_rng();
        let result = evaluate(&mut rng, "Hello, #{Name.firstName}!").unwrap();
        assert!(result.starts_with("Hello, "));
        assert!(result.ends_with("!"));
    }

    #[test]
    fn test_evaluate_number_between() {
        let mut rng = test_rng();
        let result = evaluate(&mut rng, "#{Number.between 1, 100}").unwrap();
        let num: i64 = result.parse().unwrap();
        assert!((1..=100).contains(&num));
    }

    #[test]
    fn test_evaluate_options() {
        let mut rng = test_rng();
        let result = evaluate(&mut rng, "#{options.option 'A', 'B', 'C'}").unwrap();
        assert!(result == "A" || result == "B" || result == "C");
    }
}
