//! Expression DSL for data generation.
//!
//! Provides a Datafaker-compatible expression language for generating complex data.
//!
//! # Expression Syntax
//!
//! Expressions are enclosed in `#{...}`:
//!
//! ```text
//! #{Provider.method}              - Call a provider method
//! #{Provider.method arg1, arg2}   - With arguments
//! #{function 'arg'}               - Call a built-in function
//! Hello, #{Name.firstName}!       - Mix with literal text
//! ```
//!
//! # Providers
//!
//! Providers are data generators organized by category:
//!
//! - `Name` - firstName, lastName, fullName, prefix, suffix
//! - `Address` - street, city, state, zipCode, country
//! - `Phone` - phoneNumber, cellPhone
//! - `Internet` - email, url, domain, ipV4, ipV6, uuid
//! - `Commerce` - product, price, department
//! - `Company` - name, buzzword, industry
//! - `Number` - between, digit, decimal
//! - `Date` - past, future, birthday
//! - `Lorem` - word, sentence, paragraph
//! - And many more...
//!
//! # Built-in Functions
//!
//! - `regexify 'pattern'` - Generate string from regex pattern
//! - `templatify 'template', 'char', 'class'` - Replace characters
//! - `exemplify 'pattern'` - Replace ? with letters, # with digits
//! - `letterify 'pattern'` - Replace ? with letters
//! - `numerify 'pattern'` - Replace # with digits
//! - `options.option 'A', 'B', 'C'` - Random choice
//! - `options.weighted 'A', 80, 'B', 20` - Weighted choice
//!
//! # Examples
//!
//! ```rust
//! use rand::SeedableRng;
//! use rand_chacha::ChaCha8Rng;
//! use dx_datagen::expression::evaluate;
//!
//! let mut rng = ChaCha8Rng::seed_from_u64(42);
//!
//! // Provider call
//! let name = evaluate(&mut rng, "#{Name.firstName}").unwrap();
//!
//! // Mixed with literals
//! let greeting = evaluate(&mut rng, "Hello, #{Name.firstName}!").unwrap();
//!
//! // Regex generation
//! let code = evaluate(&mut rng, "#{regexify '[A-Z]{3}-[0-9]{4}'}").unwrap();
//!
//! // Random choice
//! let color = evaluate(&mut rng, "#{options.option 'red', 'green', 'blue'}").unwrap();
//!
//! // Numeric ranges
//! let age = evaluate(&mut rng, "#{Number.between 18, 65}").unwrap();
//! ```

pub mod ast;
pub mod evaluator;
pub mod functions;
pub mod lexer;
pub mod parser;
pub mod providers;

// Re-export main types and functions
pub use ast::{Argument, Expression, FunctionCall, Literal, ProviderCall, Template, TemplatePart};
pub use evaluator::{evaluate, EvalError, Evaluator};
pub use functions::{call_function, FunctionError};
pub use lexer::{Lexer, LexerError, Token};
pub use parser::{ParseError, Parser};
pub use providers::{call_provider, ProviderError};

/// Parse an expression template string into an AST.
pub fn parse(input: &str) -> Result<Template, ParseError> {
    Parser::parse(input)
}

/// Generate data from an expression template with the given RNG.
///
/// This is the main entry point for using the expression DSL.
pub fn generate<R: rand::Rng + ?Sized>(rng: &mut R, template: &str) -> Result<String, EvalError> {
    evaluate(rng, template)
}

/// Generate multiple values from an expression template.
pub fn generate_batch<R: rand::Rng + ?Sized>(
    rng: &mut R,
    template: &str,
    count: usize,
) -> Result<Vec<String>, EvalError> {
    let parsed = parse(template).map_err(|e| EvalError::new(&e.message))?;
    let mut evaluator = Evaluator::new(rng);
    let mut results = Vec::with_capacity(count);

    for _ in 0..count {
        results.push(evaluator.evaluate(&parsed)?);
    }

    Ok(results)
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
    fn test_generate_name() {
        let mut rng = test_rng();
        let result = generate(&mut rng, "#{Name.firstName}").unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_generate_mixed() {
        let mut rng = test_rng();
        let result = generate(&mut rng, "Hello, #{Name.firstName} #{Name.lastName}!").unwrap();
        assert!(result.starts_with("Hello, "));
        assert!(result.ends_with("!"));
        assert!(result.len() > 10); // Should have names in the middle
    }

    #[test]
    fn test_generate_regexify() {
        let mut rng = test_rng();
        let result = generate(&mut rng, "#{regexify '[A-Z]{3}-[0-9]{4}'}").unwrap();
        assert_eq!(result.len(), 8);
        assert!(result.chars().nth(3) == Some('-'));
    }

    #[test]
    fn test_generate_options() {
        let mut rng = test_rng();
        let result = generate(&mut rng, "#{options.option 'A', 'B', 'C'}").unwrap();
        assert!(result == "A" || result == "B" || result == "C");
    }

    #[test]
    fn test_generate_batch() {
        let mut rng = test_rng();
        let results = generate_batch(&mut rng, "#{Name.firstName}", 10).unwrap();
        assert_eq!(results.len(), 10);
        for name in &results {
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn test_generate_number_between() {
        let mut rng = test_rng();
        let result = generate(&mut rng, "#{Number.between 1, 100}").unwrap();
        let num: i64 = result.parse().unwrap();
        assert!((1..=100).contains(&num));
    }

    #[test]
    fn test_generate_email() {
        let mut rng = test_rng();
        let result = generate(&mut rng, "#{Internet.email}").unwrap();
        assert!(result.contains('@'));
        assert!(result.contains('.'));
    }

    #[test]
    fn test_generate_address() {
        let mut rng = test_rng();
        let result = generate(
            &mut rng,
            "#{Address.streetAddress}, #{Address.city}, #{Address.country}",
        )
        .unwrap();
        assert!(result.contains(','));
    }

    #[test]
    fn test_generate_complex_template() {
        let mut rng = test_rng();
        let result = generate(
            &mut rng,
            "Order #{regexify '[A-Z]{3}-[0-9]{4}'}: #{Commerce.product} - $#{Number.between 10, 100}",
        )
        .unwrap();
        assert!(result.starts_with("Order "));
        assert!(result.contains('$'));
    }

    #[test]
    fn test_plain_text() {
        let mut rng = test_rng();
        let result = generate(&mut rng, "No expressions here!").unwrap();
        assert_eq!(result, "No expressions here!");
    }

    #[test]
    fn test_multiple_providers() {
        let mut rng = test_rng();
        // Test various providers work
        assert!(!generate(&mut rng, "#{Lorem.word}").unwrap().is_empty());
        assert!(!generate(&mut rng, "#{Color.name}").unwrap().is_empty());
        assert!(!generate(&mut rng, "#{Vehicle.make}").unwrap().is_empty());
        assert!(!generate(&mut rng, "#{Food.dish}").unwrap().is_empty());
        assert!(!generate(&mut rng, "#{Weather.condition}")
            .unwrap()
            .is_empty());
    }
}
