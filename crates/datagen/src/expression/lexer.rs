//! Lexer/tokenizer for the expression DSL.
//!
//! Tokenizes expressions like `#{Name.firstName}` and `#{regexify '[A-Z]{3}'}`.

use std::iter::Peekable;
use std::str::Chars;

/// A token in the expression language.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Start of expression: `#{`
    ExprStart,
    /// End of expression: `}`
    ExprEnd,
    /// A dot separator: `.`
    Dot,
    /// A comma separator: `,`
    Comma,
    /// An identifier: `Name`, `firstName`, `regexify`
    Ident(String),
    /// A string literal: `'value'` or `"value"`
    String(String),
    /// A number literal: `42`, `3.14`, `-5`
    Number(f64),
    /// Boolean true
    True,
    /// Boolean false
    False,
    /// Literal text outside expressions
    Literal(String),
    /// End of input
    Eof,
}

/// Lexer error type.
#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    pub message: String,
    pub position: usize,
}

impl LexerError {
    pub fn new(message: &str, position: usize) -> Self {
        Self {
            message: message.to_string(),
            position,
        }
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lexer error at position {}: {}",
            self.position, self.message
        )
    }
}

impl std::error::Error for LexerError {}

/// Lexer for the expression DSL.
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    position: usize,
    in_expression: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            position: 0,
            in_expression: false,
        }
    }

    /// Tokenize the entire input.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token == Token::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }

    /// Get the next token.
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        if self.in_expression {
            self.next_expression_token()
        } else {
            self.next_literal_or_expr_start()
        }
    }

    /// Read literal text or expression start.
    fn next_literal_or_expr_start(&mut self) -> Result<Token, LexerError> {
        let mut literal = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch == '#' {
                // Check for expression start `#{`
                let saved_pos = self.position;
                self.advance();
                if self.chars.peek() == Some(&'{') {
                    self.advance();
                    self.in_expression = true;

                    // Return any accumulated literal first
                    if !literal.is_empty() {
                        // We need to "unread" the `#{` - this is tricky
                        // Instead, we'll handle this differently
                        // For now, return the literal and the caller will get ExprStart next time
                        // Actually, let's return the literal now and reset
                        self.position = saved_pos;
                        self.chars = self.input[saved_pos..].chars().peekable();
                        self.in_expression = false;
                        return Ok(Token::Literal(literal));
                    }
                    return Ok(Token::ExprStart);
                } else {
                    // Just a `#`, add to literal
                    literal.push('#');
                }
            } else {
                literal.push(ch);
                self.advance();
            }
        }

        if literal.is_empty() {
            Ok(Token::Eof)
        } else {
            Ok(Token::Literal(literal))
        }
    }

    /// Read a token inside an expression.
    fn next_expression_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        match self.chars.peek() {
            None => Ok(Token::Eof),
            Some(&'}') => {
                self.advance();
                self.in_expression = false;
                Ok(Token::ExprEnd)
            }
            Some(&'.') => {
                self.advance();
                Ok(Token::Dot)
            }
            Some(&',') => {
                self.advance();
                Ok(Token::Comma)
            }
            Some(&'\'') | Some(&'"') => self.read_string(),
            Some(&ch) if ch.is_ascii_digit() || ch == '-' => self.read_number(),
            Some(&ch) if is_ident_start(ch) => self.read_identifier(),
            Some(&ch) => Err(LexerError::new(
                &format!("Unexpected character: '{}'", ch),
                self.position,
            )),
        }
    }

    /// Read a string literal.
    fn read_string(&mut self) -> Result<Token, LexerError> {
        let quote = self.chars.peek().copied().unwrap();
        self.advance(); // consume opening quote

        let mut value = String::new();
        let start_pos = self.position;

        loop {
            match self.chars.peek() {
                None => {
                    return Err(LexerError::new("Unterminated string", start_pos));
                }
                Some(&ch) if ch == quote => {
                    self.advance(); // consume closing quote
                    break;
                }
                Some(&'\\') => {
                    self.advance();
                    match self.chars.peek() {
                        Some(&'n') => {
                            value.push('\n');
                            self.advance();
                        }
                        Some(&'t') => {
                            value.push('\t');
                            self.advance();
                        }
                        Some(&'r') => {
                            value.push('\r');
                            self.advance();
                        }
                        Some(&'\\') => {
                            value.push('\\');
                            self.advance();
                        }
                        Some(&ch) if ch == quote => {
                            value.push(ch);
                            self.advance();
                        }
                        Some(&ch) => {
                            value.push('\\');
                            value.push(ch);
                            self.advance();
                        }
                        None => {
                            return Err(LexerError::new(
                                "Unterminated escape sequence",
                                self.position,
                            ));
                        }
                    }
                }
                Some(&ch) => {
                    value.push(ch);
                    self.advance();
                }
            }
        }

        Ok(Token::String(value))
    }

    /// Read a number literal.
    fn read_number(&mut self) -> Result<Token, LexerError> {
        let mut value = String::new();
        let start_pos = self.position;

        // Handle negative sign
        if self.chars.peek() == Some(&'-') {
            value.push('-');
            self.advance();
        }

        // Read integer part
        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Read decimal part
        if self.chars.peek() == Some(&'.') {
            value.push('.');
            self.advance();

            while let Some(&ch) = self.chars.peek() {
                if ch.is_ascii_digit() {
                    value.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        value
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| LexerError::new(&format!("Invalid number: {}", value), start_pos))
    }

    /// Read an identifier.
    fn read_identifier(&mut self) -> Result<Token, LexerError> {
        let mut value = String::new();

        while let Some(&ch) = self.chars.peek() {
            if is_ident_char(ch) {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for keywords
        match value.as_str() {
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            _ => Ok(Token::Ident(value)),
        }
    }

    /// Skip whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Advance to the next character.
    fn advance(&mut self) {
        if let Some(ch) = self.chars.next() {
            self.position += ch.len_utf8();
        }
    }
}

/// Check if a character can start an identifier.
fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

/// Check if a character can be part of an identifier.
fn is_ident_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<Token> {
        Lexer::new(input).tokenize().unwrap()
    }

    #[test]
    fn test_simple_expression() {
        let tokens = tokenize("#{Name.firstName}");
        assert_eq!(
            tokens,
            vec![
                Token::ExprStart,
                Token::Ident("Name".to_string()),
                Token::Dot,
                Token::Ident("firstName".to_string()),
                Token::ExprEnd,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_literal_and_expression() {
        let tokens = tokenize("Hello, #{Name.firstName}!");
        assert_eq!(
            tokens,
            vec![
                Token::Literal("Hello, ".to_string()),
                Token::ExprStart,
                Token::Ident("Name".to_string()),
                Token::Dot,
                Token::Ident("firstName".to_string()),
                Token::ExprEnd,
                Token::Literal("!".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_function_with_string_arg() {
        let tokens = tokenize("#{regexify '[A-Z]{3}'}");
        assert_eq!(
            tokens,
            vec![
                Token::ExprStart,
                Token::Ident("regexify".to_string()),
                Token::String("[A-Z]{3}".to_string()),
                Token::ExprEnd,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_function_with_multiple_args() {
        let tokens = tokenize("#{options.option 'A','B','C'}");
        assert_eq!(
            tokens,
            vec![
                Token::ExprStart,
                Token::Ident("options".to_string()),
                Token::Dot,
                Token::Ident("option".to_string()),
                Token::String("A".to_string()),
                Token::Comma,
                Token::String("B".to_string()),
                Token::Comma,
                Token::String("C".to_string()),
                Token::ExprEnd,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_number_args() {
        let tokens = tokenize("#{Number.between 1, 100}");
        assert_eq!(
            tokens,
            vec![
                Token::ExprStart,
                Token::Ident("Number".to_string()),
                Token::Dot,
                Token::Ident("between".to_string()),
                Token::Number(1.0),
                Token::Comma,
                Token::Number(100.0),
                Token::ExprEnd,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_boolean() {
        let tokens = tokenize("#{test true, false}");
        assert_eq!(
            tokens,
            vec![
                Token::ExprStart,
                Token::Ident("test".to_string()),
                Token::True,
                Token::Comma,
                Token::False,
                Token::ExprEnd,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_escaped_string() {
        let tokens = tokenize("#{test 'hello\\'world'}");
        assert_eq!(
            tokens,
            vec![
                Token::ExprStart,
                Token::Ident("test".to_string()),
                Token::String("hello'world".to_string()),
                Token::ExprEnd,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_multiple_expressions() {
        let tokens = tokenize("#{A} and #{B}");
        assert_eq!(
            tokens,
            vec![
                Token::ExprStart,
                Token::Ident("A".to_string()),
                Token::ExprEnd,
                Token::Literal(" and ".to_string()),
                Token::ExprStart,
                Token::Ident("B".to_string()),
                Token::ExprEnd,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_plain_literal() {
        let tokens = tokenize("Just plain text");
        assert_eq!(
            tokens,
            vec![Token::Literal("Just plain text".to_string()), Token::Eof]
        );
    }

    #[test]
    fn test_negative_number() {
        let tokens = tokenize("#{test -5}");
        assert_eq!(
            tokens,
            vec![
                Token::ExprStart,
                Token::Ident("test".to_string()),
                Token::Number(-5.0),
                Token::ExprEnd,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_float_number() {
        let tokens = tokenize("#{test 3.14}");
        assert_eq!(
            tokens,
            vec![
                Token::ExprStart,
                Token::Ident("test".to_string()),
                Token::Number(3.14),
                Token::ExprEnd,
                Token::Eof,
            ]
        );
    }
}
