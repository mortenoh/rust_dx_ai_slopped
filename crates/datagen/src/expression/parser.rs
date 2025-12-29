//! Parser for the expression DSL.
//!
//! Parses tokens into an AST.

use super::ast::{
    Argument, Expression, FunctionCall, Literal, ProviderCall, Template, TemplatePart,
};
use super::lexer::{Lexer, LexerError, Token};

/// Parser error type.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: Option<usize>,
}

impl ParseError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            position: None,
        }
    }

    pub fn at_position(message: &str, position: usize) -> Self {
        Self {
            message: message.to_string(),
            position: Some(position),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.position {
            Some(pos) => write!(f, "Parse error at position {}: {}", pos, self.message),
            None => write!(f, "Parse error: {}", self.message),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<LexerError> for ParseError {
    fn from(err: LexerError) -> Self {
        ParseError::at_position(&err.message, err.position)
    }
}

/// Parser for expression templates.
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Parse an expression template string.
    pub fn parse(input: &str) -> Result<Template, ParseError> {
        let tokens = Lexer::new(input).tokenize()?;
        let mut parser = Parser {
            tokens,
            position: 0,
        };
        parser.parse_template()
    }

    /// Parse the template.
    fn parse_template(&mut self) -> Result<Template, ParseError> {
        let mut parts = Vec::new();

        while !self.is_at_end() {
            match self.peek() {
                Token::Literal(text) => {
                    parts.push(TemplatePart::Literal(text.clone()));
                    self.advance();
                }
                Token::ExprStart => {
                    self.advance(); // consume `#{`
                    let expr = self.parse_expression()?;
                    parts.push(TemplatePart::Expression(expr));
                    self.expect(Token::ExprEnd)?;
                }
                Token::Eof => break,
                other => {
                    return Err(ParseError::new(&format!("Unexpected token: {:?}", other)));
                }
            }
        }

        Ok(Template::new(parts))
    }

    /// Parse an expression inside `#{...}`.
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        // Check for function call or provider call
        // Format: `Ident` or `Ident.Ident` optionally followed by args

        let first_ident = self.expect_ident()?;

        if self.check(&Token::Dot) {
            self.advance(); // consume `.`
            let second_ident = self.expect_ident()?;

            // Check for more dots (e.g., `options.option`)
            if self.check(&Token::Dot) {
                self.advance();
                let third_ident = self.expect_ident()?;
                // This is a namespaced function: `namespace.function`
                let name = format!("{}.{}.{}", first_ident, second_ident, third_ident);
                let args = self.parse_arguments()?;
                return Ok(Expression::FunctionCall(FunctionCall::new(&name, args)));
            }

            // Could be `Provider.method` or `namespace.function`
            // Heuristic: if first char is uppercase, it's a provider
            if first_ident
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
            {
                let args = self.parse_arguments()?;
                Ok(Expression::ProviderCall(
                    ProviderCall::new(&first_ident, &second_ident).with_args(args),
                ))
            } else {
                // namespaced function like `options.option`
                let name = format!("{}.{}", first_ident, second_ident);
                let args = self.parse_arguments()?;
                Ok(Expression::FunctionCall(FunctionCall::new(&name, args)))
            }
        } else {
            // Just a function name or a simple identifier
            let args = self.parse_arguments()?;

            if args.is_empty() && !self.check(&Token::ExprEnd) {
                // Could be a simple literal or identifier
                Ok(Expression::Literal(Literal::String(first_ident)))
            } else {
                Ok(Expression::FunctionCall(FunctionCall::new(
                    &first_ident,
                    args,
                )))
            }
        }
    }

    /// Parse function/method arguments.
    fn parse_arguments(&mut self) -> Result<Vec<Argument>, ParseError> {
        let mut args = Vec::new();

        // Arguments are space or comma separated values until `}`
        while !self.check(&Token::ExprEnd) && !self.is_at_end() {
            // Skip optional comma
            if self.check(&Token::Comma) {
                self.advance();
                continue;
            }

            let arg = self.parse_argument()?;
            args.push(arg);
        }

        Ok(args)
    }

    /// Parse a single argument.
    fn parse_argument(&mut self) -> Result<Argument, ParseError> {
        match self.peek() {
            Token::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Argument::String(value))
            }
            Token::Number(n) => {
                let value = *n;
                self.advance();
                Ok(Argument::Number(value))
            }
            Token::True => {
                self.advance();
                Ok(Argument::Boolean(true))
            }
            Token::False => {
                self.advance();
                Ok(Argument::Boolean(false))
            }
            Token::Ident(s) => {
                // Could be a nested provider/function call or just a string
                let value = s.clone();
                self.advance();
                Ok(Argument::String(value))
            }
            Token::ExprStart => {
                // Nested expression
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::ExprEnd)?;
                Ok(Argument::Expression(Box::new(expr)))
            }
            other => Err(ParseError::new(&format!(
                "Expected argument, got {:?}",
                other
            ))),
        }
    }

    /// Expect and consume an identifier token.
    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.peek() {
            Token::Ident(s) => {
                let value = s.clone();
                self.advance();
                Ok(value)
            }
            other => Err(ParseError::new(&format!(
                "Expected identifier, got {:?}",
                other
            ))),
        }
    }

    /// Expect and consume a specific token.
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.check(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(&format!(
                "Expected {:?}, got {:?}",
                expected,
                self.peek()
            )))
        }
    }

    /// Check if the current token matches.
    fn check(&self, token: &Token) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
    }

    /// Peek at the current token.
    fn peek(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    /// Advance to the next token.
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
        }
    }

    /// Check if we're at the end.
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_provider_call() {
        let template = Parser::parse("#{Name.firstName}").unwrap();
        assert_eq!(template.parts.len(), 1);

        match &template.parts[0] {
            TemplatePart::Expression(Expression::ProviderCall(call)) => {
                assert_eq!(call.provider, "Name");
                assert_eq!(call.method, "firstName");
                assert!(call.args.is_empty());
            }
            _ => panic!("Expected provider call"),
        }
    }

    #[test]
    fn test_parse_function_call() {
        let template = Parser::parse("#{regexify '[A-Z]{3}'}").unwrap();
        assert_eq!(template.parts.len(), 1);

        match &template.parts[0] {
            TemplatePart::Expression(Expression::FunctionCall(call)) => {
                assert_eq!(call.name, "regexify");
                assert_eq!(call.args.len(), 1);
                assert_eq!(call.args[0].as_string(), Some("[A-Z]{3}"));
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_parse_namespaced_function() {
        let template = Parser::parse("#{options.option 'A','B','C'}").unwrap();
        assert_eq!(template.parts.len(), 1);

        match &template.parts[0] {
            TemplatePart::Expression(Expression::FunctionCall(call)) => {
                assert_eq!(call.name, "options.option");
                assert_eq!(call.args.len(), 3);
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_parse_literal_and_expression() {
        let template = Parser::parse("Hello, #{Name.firstName}!").unwrap();
        assert_eq!(template.parts.len(), 3);

        match &template.parts[0] {
            TemplatePart::Literal(text) => assert_eq!(text, "Hello, "),
            _ => panic!("Expected literal"),
        }

        match &template.parts[1] {
            TemplatePart::Expression(Expression::ProviderCall(call)) => {
                assert_eq!(call.provider, "Name");
                assert_eq!(call.method, "firstName");
            }
            _ => panic!("Expected provider call"),
        }

        match &template.parts[2] {
            TemplatePart::Literal(text) => assert_eq!(text, "!"),
            _ => panic!("Expected literal"),
        }
    }

    #[test]
    fn test_parse_multiple_expressions() {
        let template = Parser::parse("#{Name.firstName} #{Name.lastName}").unwrap();
        assert_eq!(template.parts.len(), 3);
    }

    #[test]
    fn test_parse_provider_with_args() {
        let template = Parser::parse("#{Number.between 1, 100}").unwrap();
        assert_eq!(template.parts.len(), 1);

        match &template.parts[0] {
            TemplatePart::Expression(Expression::ProviderCall(call)) => {
                assert_eq!(call.provider, "Number");
                assert_eq!(call.method, "between");
                assert_eq!(call.args.len(), 2);
                assert_eq!(call.args[0].as_i64(), Some(1));
                assert_eq!(call.args[1].as_i64(), Some(100));
            }
            _ => panic!("Expected provider call"),
        }
    }

    #[test]
    fn test_parse_plain_text() {
        let template = Parser::parse("Just plain text").unwrap();
        assert_eq!(template.parts.len(), 1);

        match &template.parts[0] {
            TemplatePart::Literal(text) => assert_eq!(text, "Just plain text"),
            _ => panic!("Expected literal"),
        }
    }

    #[test]
    fn test_parse_templatify() {
        let template = Parser::parse("#{templatify '###-###-####', '#', '0-9'}").unwrap();
        assert_eq!(template.parts.len(), 1);

        match &template.parts[0] {
            TemplatePart::Expression(Expression::FunctionCall(call)) => {
                assert_eq!(call.name, "templatify");
                assert_eq!(call.args.len(), 3);
                assert_eq!(call.args[0].as_string(), Some("###-###-####"));
                assert_eq!(call.args[1].as_string(), Some("#"));
                assert_eq!(call.args[2].as_string(), Some("0-9"));
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_parse_boolean_args() {
        let template = Parser::parse("#{test true, false}").unwrap();
        match &template.parts[0] {
            TemplatePart::Expression(Expression::FunctionCall(call)) => {
                assert_eq!(call.args.len(), 2);
                match &call.args[0] {
                    Argument::Boolean(b) => assert!(*b),
                    _ => panic!("Expected boolean"),
                }
                match &call.args[1] {
                    Argument::Boolean(b) => assert!(!*b),
                    _ => panic!("Expected boolean"),
                }
            }
            _ => panic!("Expected function call"),
        }
    }
}
