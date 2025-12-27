//! # Recursive Descent Parser for Arithmetic Expressions
//!
//! Parses expression strings into an AST for evaluation.
//!
//! ## Grammar
//!
//! ```text
//! expr    = term (('+' | '-') term)*
//! term    = power (('*' | '/' | '%') power)*
//! power   = unary ('^' power)?          // right-associative
//! unary   = '-'? call
//! call    = identifier '(' expr ')' | primary
//! primary = number | identifier | '(' expr ')'
//! ```
//!
//! ## Operator Precedence (lowest to highest)
//!
//! 1. Addition, Subtraction (`+`, `-`)
//! 2. Multiplication, Division, Modulo (`*`, `/`, `%`)
//! 3. Power (`^`) - right-associative
//! 4. Unary minus (`-`)
//! 5. Function calls, parentheses
//!
//! ## Supported Features
//!
//! - Constants: `pi`, `e`, `tau`
//! - Functions: `sin`, `cos`, `tan`, `sqrt`, `abs`, `ln`, `log2`, `log10`, etc.

use anyhow::{Context, Result, bail};

use super::ast::{BinOp, Expr, Program, Statement, UnaryOp};

/// Expression parser using recursive descent.
///
/// Parses an input string into an AST.
pub(crate) struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    /// Create a new parser for the given input string.
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    /// Parse the entire expression and return the AST.
    pub fn parse(&mut self) -> Result<Expr> {
        self.skip_all_whitespace();
        let result = self.expr()?;
        self.skip_all_whitespace();
        if self.pos < self.input.len() {
            bail!(
                "Unexpected character at position {}: '{}'",
                self.pos,
                self.current_char().unwrap_or('?')
            );
        }
        Ok(result)
    }

    /// Parse a multi-line program (statements separated by newlines or semicolons)
    pub fn parse_program(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        loop {
            self.skip_whitespace_not_newline();
            self.skip_empty_lines();

            if self.pos >= self.input.len() {
                break;
            }

            let stmt = self.statement()?;
            statements.push(stmt);

            self.skip_whitespace_not_newline();

            // Expect newline, semicolon, or EOF
            match self.current_char() {
                None => break,
                Some('\n') | Some('\r') => {
                    self.skip_newlines();
                }
                Some(';') => {
                    self.advance();
                }
                Some(c) => {
                    bail!(
                        "Expected newline or semicolon, got '{}' at position {}",
                        c,
                        self.pos
                    );
                }
            }
        }

        Ok(Program { statements })
    }

    /// Parse a single statement (assignment or expression)
    fn statement(&mut self) -> Result<Statement> {
        self.skip_whitespace_not_newline();

        // Look ahead: if we have identifier followed by '=', it's an assignment
        let start_pos = self.pos;

        if let Some(c) = self.current_char()
            && c.is_ascii_alphabetic()
        {
            let name = self.identifier();
            self.skip_whitespace_not_newline();

            if self.current_char() == Some('=') {
                // Check it's not '==' (comparison - not supported yet, but guard against it)
                let _eq_pos = self.pos;
                self.advance();
                if self.current_char() == Some('=') {
                    // It's '==', rewind and parse as expression
                    self.pos = start_pos;
                } else {
                    // It's assignment
                    let value = self.expr()?;
                    return Ok(Statement::Assignment { name, value });
                }
            } else {
                // Not assignment, rewind and parse as expression
                self.pos = start_pos;
            }
        }

        // Parse as expression
        let expr = self.expr()?;
        Ok(Statement::Expression(expr))
    }

    /// Skip whitespace except newlines
    fn skip_whitespace_not_newline(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() && c != '\n' && c != '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip newline characters
    fn skip_newlines(&mut self) {
        while let Some(c) = self.current_char() {
            if c == '\n' || c == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip empty lines (whitespace-only lines)
    fn skip_empty_lines(&mut self) {
        loop {
            let start = self.pos;
            self.skip_whitespace_not_newline();
            match self.current_char() {
                Some('\n') | Some('\r') => {
                    self.skip_newlines();
                }
                _ => {
                    self.pos = start;
                    break;
                }
            }
        }
    }

    /// Get current character without advancing.
    fn current_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// Advance position by one character.
    fn advance(&mut self) {
        if let Some(c) = self.current_char() {
            self.pos += c.len_utf8();
        }
    }

    /// Skip whitespace characters (not including newlines for statement separation).
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() && c != '\n' && c != '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip all whitespace including newlines
    fn skip_all_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Parse: expr = term (('+' | '-') term)*
    fn expr(&mut self) -> Result<Expr> {
        let mut left = self.term()?;

        loop {
            self.skip_whitespace();
            match self.current_char() {
                Some('+') => {
                    self.advance();
                    let right = self.term()?;
                    left = Expr::binop(BinOp::Add, left, right);
                }
                Some('-') => {
                    self.advance();
                    let right = self.term()?;
                    left = Expr::binop(BinOp::Sub, left, right);
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse: term = power (('*' | '/' | '%') power)*
    fn term(&mut self) -> Result<Expr> {
        let mut left = self.power()?;

        loop {
            self.skip_whitespace();
            match self.current_char() {
                Some('*') => {
                    self.advance();
                    let right = self.power()?;
                    left = Expr::binop(BinOp::Mul, left, right);
                }
                Some('/') => {
                    self.advance();
                    let right = self.power()?;
                    left = Expr::binop(BinOp::Div, left, right);
                }
                Some('%') => {
                    self.advance();
                    let right = self.power()?;
                    left = Expr::binop(BinOp::Mod, left, right);
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse: power = unary (('^' | '**') power)?  (right-associative)
    fn power(&mut self) -> Result<Expr> {
        let base = self.unary()?;

        self.skip_whitespace();
        if self.current_char() == Some('^') {
            self.advance();
            let exp = self.power()?; // Right-associative: recurse into power
            Ok(Expr::binop(BinOp::Pow, base, exp))
        } else if self.current_char() == Some('*') {
            // Check for **
            let start = self.pos;
            self.advance();
            if self.current_char() == Some('*') {
                self.advance();
                let exp = self.power()?;
                Ok(Expr::binop(BinOp::Pow, base, exp))
            } else {
                // Just a single *, rewind and let term() handle it
                self.pos = start;
                Ok(base)
            }
        } else {
            Ok(base)
        }
    }

    /// Parse: unary = '-'? call
    fn unary(&mut self) -> Result<Expr> {
        self.skip_whitespace();
        if self.current_char() == Some('-') {
            self.advance();
            let expr = self.unary()?; // Allow chained unary: --5
            Ok(Expr::unary(UnaryOp::Neg, expr))
        } else {
            self.call()
        }
    }

    /// Parse: call = identifier '(' expr ')' | constant | variable | primary
    fn call(&mut self) -> Result<Expr> {
        self.skip_whitespace();

        // Check if it starts with a letter (identifier)
        if let Some(c) = self.current_char()
            && c.is_ascii_alphabetic()
        {
            let name = self.identifier();

            self.skip_whitespace();
            if self.current_char() == Some('(') {
                // Function call
                self.advance();
                let arg = self.expr()?;
                self.skip_whitespace();
                if self.current_char() != Some(')') {
                    bail!(
                        "Expected ')' after function argument at position {}",
                        self.pos
                    );
                }
                self.advance();
                return Ok(Expr::func_call(name, arg));
            } else if matches!(name.as_str(), "pi" | "e" | "tau") {
                // Built-in constant
                return Ok(Expr::constant(name));
            } else {
                // Variable reference
                return Ok(Expr::variable(name));
            }
        }

        self.primary()
    }

    /// Parse an identifier (letters and digits, starting with letter)
    fn identifier(&mut self) -> String {
        let start = self.pos;
        while let Some(c) = self.current_char() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    /// Parse: primary = number | '(' expr ')'
    fn primary(&mut self) -> Result<Expr> {
        self.skip_whitespace();

        match self.current_char() {
            Some('(') => {
                self.advance();
                let result = self.expr()?;
                self.skip_whitespace();
                if self.current_char() != Some(')') {
                    bail!("Expected ')' at position {}", self.pos);
                }
                self.advance();
                Ok(result)
            }
            Some(c) if c.is_ascii_digit() || c == '.' => self.number(),
            Some(c) => bail!("Unexpected character '{}' at position {}", c, self.pos),
            None => bail!("Unexpected end of expression"),
        }
    }

    /// Parse a number (integer or float).
    fn number(&mut self) -> Result<Expr> {
        let start = self.pos;

        // Consume digits before decimal point
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point
        if self.current_char() == Some('.') {
            self.advance();
            // Consume digits after decimal point
            while let Some(c) = self.current_char() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let num_str = &self.input[start..self.pos];
        let value = num_str
            .parse::<f64>()
            .with_context(|| format!("Invalid number: '{}'", num_str))?;

        Ok(Expr::number(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> Result<f64> {
        Parser::new(input).parse()?.eval()
    }

    #[test]
    fn test_basic_numbers() {
        assert_eq!(parse("42").unwrap(), 42.0);
        assert_eq!(parse("3.14").unwrap(), 3.14);
        assert_eq!(parse("0.5").unwrap(), 0.5);
    }

    #[test]
    fn test_basic_operations() {
        assert_eq!(parse("2 + 2").unwrap(), 4.0);
        assert_eq!(parse("10 - 3").unwrap(), 7.0);
        assert_eq!(parse("4 * 5").unwrap(), 20.0);
        assert_eq!(parse("15 / 3").unwrap(), 5.0);
    }

    #[test]
    fn test_precedence() {
        // * binds tighter than +
        assert_eq!(parse("2 + 3 * 4").unwrap(), 14.0);
        assert_eq!(parse("10 - 2 * 3").unwrap(), 4.0);
        assert_eq!(parse("100 / 10 + 5").unwrap(), 15.0);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(parse("(2 + 3) * 4").unwrap(), 20.0);
        assert_eq!(parse("(10 - 2) * 3").unwrap(), 24.0);
        assert_eq!(parse("((2 + 3))").unwrap(), 5.0);
        assert_eq!(parse("(1 + (2 * 3))").unwrap(), 7.0);
        assert_eq!(parse("((1 + 2) * (3 + 4))").unwrap(), 21.0);
    }

    #[test]
    fn test_unary_minus() {
        assert_eq!(parse("-5").unwrap(), -5.0);
        assert_eq!(parse("10 + -3").unwrap(), 7.0);
        assert_eq!(parse("-2 * -3").unwrap(), 6.0);
        assert_eq!(parse("(-5)").unwrap(), -5.0);
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(parse("  2 + 2  ").unwrap(), 4.0);
        assert_eq!(parse("1+2").unwrap(), 3.0);
        assert_eq!(parse("1  +  2").unwrap(), 3.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(parse("1 / 0").is_err());
    }

    #[test]
    fn test_invalid_expressions() {
        assert!(Parser::new("").parse().is_err());
        assert!(Parser::new("1 +").parse().is_err());
        assert!(Parser::new("(1 + 2").parse().is_err());
        assert!(Parser::new("1 2").parse().is_err());
    }

    #[test]
    fn test_ast_structure() {
        // Verify the AST structure for "2 + 3 * 4"
        let ast = Parser::new("2 + 3 * 4").parse().unwrap();

        // Should be: Add(2, Mul(3, 4))
        match ast {
            Expr::BinOp {
                op: BinOp::Add,
                left,
                right,
            } => {
                assert!(matches!(*left, Expr::Number { value } if value == 2.0));
                assert!(matches!(*right, Expr::BinOp { op: BinOp::Mul, .. }));
            }
            _ => panic!("Expected BinOp::Add at root"),
        }
    }

    #[test]
    fn test_ast_json_serialization() {
        let ast = Parser::new("2 + 3").parse().unwrap();
        let json = serde_json::to_string(&ast).unwrap();
        assert!(json.contains("\"type\":\"binop\""));
        assert!(json.contains("\"op\":\"add\""));
    }

    #[test]
    fn test_power() {
        assert_eq!(parse("2 ^ 3").unwrap(), 8.0);
        assert_eq!(parse("2 ^ 10").unwrap(), 1024.0);
        assert_eq!(parse("4 ^ 0.5").unwrap(), 2.0); // sqrt
        assert_eq!(parse("27 ^ (1/3)").unwrap(), 3.0); // cube root
    }

    #[test]
    fn test_power_double_star() {
        // ** is an alternative syntax for power (like Python)
        assert_eq!(parse("2 ** 3").unwrap(), 8.0);
        assert_eq!(parse("2 ** 10").unwrap(), 1024.0);
        assert_eq!(parse("2 ** 3 ** 2").unwrap(), 512.0); // right-associative
        assert_eq!(parse("2 * 3 ** 2").unwrap(), 18.0); // precedence
    }

    #[test]
    fn test_power_right_associative() {
        // 2^3^2 should be 2^(3^2) = 2^9 = 512, not (2^3)^2 = 8^2 = 64
        assert_eq!(parse("2 ^ 3 ^ 2").unwrap(), 512.0);
    }

    #[test]
    fn test_power_precedence() {
        // Power binds tighter than multiplication
        assert_eq!(parse("2 * 3 ^ 2").unwrap(), 18.0); // 2 * 9
        assert_eq!(parse("2 ^ 3 * 2").unwrap(), 16.0); // 8 * 2
    }

    #[test]
    fn test_modulo() {
        assert_eq!(parse("10 % 3").unwrap(), 1.0);
        assert_eq!(parse("15 % 4").unwrap(), 3.0);
        assert_eq!(parse("10 % 5").unwrap(), 0.0);
    }

    #[test]
    fn test_constants() {
        let pi = parse("pi").unwrap();
        assert!((pi - std::f64::consts::PI).abs() < 1e-10);

        let e = parse("e").unwrap();
        assert!((e - std::f64::consts::E).abs() < 1e-10);

        let tau = parse("tau").unwrap();
        assert!((tau - std::f64::consts::TAU).abs() < 1e-10);
    }

    #[test]
    fn test_constants_in_expressions() {
        let result = parse("2 * pi").unwrap();
        assert!((result - std::f64::consts::TAU).abs() < 1e-10);

        let result = parse("e ^ 1").unwrap();
        assert!((result - std::f64::consts::E).abs() < 1e-10);
    }

    #[test]
    fn test_functions() {
        assert_eq!(parse("abs(-5)").unwrap(), 5.0);
        assert_eq!(parse("sqrt(16)").unwrap(), 4.0);
        assert_eq!(parse("sqrt(2)").unwrap(), 2.0_f64.sqrt());

        // Trig functions
        assert!((parse("sin(0)").unwrap() - 0.0).abs() < 1e-10);
        assert!((parse("cos(0)").unwrap() - 1.0).abs() < 1e-10);

        // Rounding
        assert_eq!(parse("floor(3.7)").unwrap(), 3.0);
        assert_eq!(parse("ceil(3.2)").unwrap(), 4.0);
        assert_eq!(parse("round(3.5)").unwrap(), 4.0);
    }

    #[test]
    fn test_functions_with_expressions() {
        assert_eq!(parse("sqrt(9 + 16)").unwrap(), 5.0);
        assert_eq!(parse("abs(-3 * 4)").unwrap(), 12.0);
        assert_eq!(parse("2 * sqrt(4)").unwrap(), 4.0);
    }

    #[test]
    fn test_nested_functions() {
        assert_eq!(parse("abs(abs(-5))").unwrap(), 5.0);
        assert_eq!(parse("sqrt(sqrt(16))").unwrap(), 2.0);
    }

    #[test]
    fn test_logarithms() {
        assert!((parse("ln(e)").unwrap() - 1.0).abs() < 1e-10);
        assert!((parse("log2(8)").unwrap() - 3.0).abs() < 1e-10);
        assert!((parse("log10(100)").unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_exp() {
        assert!((parse("exp(1)").unwrap() - std::f64::consts::E).abs() < 1e-10);
        assert!((parse("exp(0)").unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_unknown_function() {
        assert!(parse("unknown(5)").is_err());
    }

    #[test]
    fn test_variable_reference() {
        // Variables should parse but fail at eval time if undefined
        let ast = Parser::new("xyz").parse().unwrap();
        assert!(matches!(ast, Expr::Variable { ref name } if name == "xyz"));
        // Undefined variable should fail at eval
        assert!(ast.eval().is_err());
    }

    #[test]
    fn test_program_single_statement() {
        let prog = Parser::new("42").parse_program().unwrap();
        assert_eq!(prog.statements.len(), 1);
        assert_eq!(prog.eval().unwrap(), 42.0);
    }

    #[test]
    fn test_program_multiple_statements_semicolon() {
        let prog = Parser::new("x = 5; y = 10; x + y").parse_program().unwrap();
        assert_eq!(prog.statements.len(), 3);
        assert_eq!(prog.eval().unwrap(), 15.0);
    }

    #[test]
    fn test_program_multiple_statements_newline() {
        let prog = Parser::new("x = 5\ny = 10\nx + y").parse_program().unwrap();
        assert_eq!(prog.statements.len(), 3);
        assert_eq!(prog.eval().unwrap(), 15.0);
    }

    #[test]
    fn test_program_variable_assignment() {
        let prog = Parser::new("x = 2 + 3").parse_program().unwrap();
        assert_eq!(prog.eval().unwrap(), 5.0);
    }

    #[test]
    fn test_program_variable_reference() {
        let prog = Parser::new("x = 5; x * 2").parse_program().unwrap();
        assert_eq!(prog.eval().unwrap(), 10.0);
    }

    #[test]
    fn test_program_variable_shadowing() {
        let prog = Parser::new("x = 5; x = 10; x").parse_program().unwrap();
        assert_eq!(prog.eval().unwrap(), 10.0);
    }

    #[test]
    fn test_program_with_functions_and_variables() {
        let prog = Parser::new("r = 5; pi * r ^ 2").parse_program().unwrap();
        let result = prog.eval().unwrap();
        assert!((result - 78.53981633974483).abs() < 1e-10);
    }

    #[test]
    fn test_program_multiline_with_indentation() {
        let prog = Parser::new(
            r#"
            x = 5
            y = x + 3
            y * 2
        "#,
        )
        .parse_program()
        .unwrap();
        assert_eq!(prog.eval().unwrap(), 16.0);
    }

    #[test]
    fn test_program_cannot_assign_to_constant() {
        let prog = Parser::new("pi = 5").parse_program().unwrap();
        assert!(prog.eval().is_err());
    }

    #[test]
    fn test_program_cannot_assign_to_function() {
        let prog = Parser::new("sin = 5").parse_program().unwrap();
        assert!(prog.eval().is_err());
    }

    #[test]
    fn test_program_undefined_variable() {
        let prog = Parser::new("x + 5").parse_program().unwrap();
        assert!(prog.eval().is_err());
    }

    #[test]
    fn test_program_print_function() {
        // print should return its argument
        let prog = Parser::new("x = print(42); x + 1").parse_program().unwrap();
        assert_eq!(prog.eval().unwrap(), 43.0);
    }
}
