//! # Recursive Descent Parser for Arithmetic Expressions
//!
//! Parses expression strings into an AST for evaluation.
//!
//! ## Grammar
//!
//! ```text
//! program     = (statement | funcdef)*
//! funcdef     = 'def' identifier '(' params? ')' '=' expr
//! params      = identifier (',' identifier)*
//!
//! statement   = assignment | expr
//! assignment  = identifier '=' expr
//!
//! expr        = logical_or
//! logical_or  = logical_and (('or' | '||') logical_and)*
//! logical_and = equality (('and' | '&&') equality)*
//! equality    = comparison (('==' | '!=') comparison)*
//! comparison  = term (('<' | '>' | '<=' | '>=') term)*
//! term        = factor (('+' | '-') factor)*
//! factor      = power (('*' | '/' | '%') power)*
//! power       = unary (('^' | '**') power)?          // right-associative
//! unary       = ('not' | '!' | '-')* call
//! call        = identifier '(' args? ')' | primary
//! args        = expr (',' expr)*
//! primary     = number | identifier | '(' expr ')' | conditional | lambda
//! conditional = 'if' expr 'then' expr 'else' expr
//! lambda      = identifier '=>' expr | '(' params ')' '=>' expr
//! ```
//!
//! ## Operator Precedence (lowest to highest)
//!
//! 1. Logical OR (`or`, `||`)
//! 2. Logical AND (`and`, `&&`)
//! 3. Equality (`==`, `!=`)
//! 4. Comparison (`<`, `>`, `<=`, `>=`)
//! 5. Addition, Subtraction (`+`, `-`)
//! 6. Multiplication, Division, Modulo (`*`, `/`, `%`)
//! 7. Power (`^`, `**`) - right-associative
//! 8. Unary (`-`, `not`, `!`)
//! 9. Function calls, parentheses
//!
//! ## Supported Features
//!
//! - Constants: `pi`, `e`, `tau`, `true`, `false`
//! - Functions: `sin`, `cos`, `tan`, `sqrt`, `abs`, `ln`, `log2`, `log10`, etc.
//! - Variables and assignment
//! - User-defined functions: `def f(x) = x * 2`
//! - Lambda expressions: `x => x * 2` or `(a, b) => a + b`
//! - Conditional expressions: `if x > 0 then x else -x`
//! - Comments: `# comment to end of line`

use anyhow::{Context, Result, bail};

use super::ast::{BinOp, Expr, Program, Statement, UnaryOp, is_keyword};

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
            self.skip_comments();
            self.skip_empty_lines();

            if self.pos >= self.input.len() {
                break;
            }

            let stmt = self.statement()?;
            statements.push(stmt);

            self.skip_whitespace_not_newline();
            self.skip_comments();

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

    /// Skip comments (# to end of line)
    fn skip_comments(&mut self) {
        while self.current_char() == Some('#') {
            // Skip until end of line or end of input
            while let Some(c) = self.current_char() {
                if c == '\n' || c == '\r' {
                    break;
                }
                self.advance();
            }
            self.skip_whitespace_not_newline();
        }
    }

    /// Parse a single statement (function def, assignment, or expression)
    fn statement(&mut self) -> Result<Statement> {
        self.skip_whitespace_not_newline();
        self.skip_comments();

        // Check for function definition: def name(params) = expr
        if self.check_keyword("def") {
            return self.function_def();
        }

        // Look ahead: if we have identifier followed by '=', it's an assignment
        // But need to distinguish from '==' (equality) and '=>' (lambda)
        let start_pos = self.pos;

        if let Some(c) = self.current_char()
            && (c.is_ascii_alphabetic() || c == '_') {
                let name = self.identifier();
                self.skip_whitespace_not_newline();

                match self.current_char() {
                    Some('=') => {
                        self.advance();
                        match self.current_char() {
                            Some('=') => {
                                // It's '==', rewind and parse as expression
                                self.pos = start_pos;
                            }
                            Some('>') => {
                                // It's '=>', rewind and parse as lambda expression
                                self.pos = start_pos;
                            }
                            _ => {
                                // It's assignment: name = expr
                                self.skip_whitespace_not_newline();
                                let value = self.expr()?;
                                return Ok(Statement::Assignment { name, value });
                            }
                        }
                    }
                    _ => {
                        // Not assignment, rewind and parse as expression
                        self.pos = start_pos;
                    }
                }
            }

        // Parse as expression
        let expr = self.expr()?;
        Ok(Statement::Expression(expr))
    }

    /// Parse function definition: def name(params) = expr
    fn function_def(&mut self) -> Result<Statement> {
        // Skip 'def'
        self.advance_n(3);
        self.skip_whitespace_not_newline();

        // Parse function name
        let name = self.identifier();
        if name.is_empty() {
            bail!("Expected function name after 'def'");
        }
        if is_keyword(&name) {
            bail!("Cannot use keyword '{}' as function name", name);
        }

        self.skip_whitespace_not_newline();

        // Parse parameters: (param1, param2, ...)
        if self.current_char() != Some('(') {
            bail!("Expected '(' after function name");
        }
        self.advance();
        self.skip_whitespace_not_newline();

        let mut params = Vec::new();
        if self.current_char() != Some(')') {
            loop {
                self.skip_whitespace_not_newline();
                let param = self.identifier();
                if param.is_empty() {
                    bail!("Expected parameter name");
                }
                if is_keyword(&param) {
                    bail!("Cannot use keyword '{}' as parameter name", param);
                }
                params.push(param);

                self.skip_whitespace_not_newline();
                match self.current_char() {
                    Some(',') => {
                        self.advance();
                    }
                    Some(')') => break,
                    _ => bail!("Expected ',' or ')' in parameter list"),
                }
            }
        }

        if self.current_char() != Some(')') {
            bail!("Expected ')' after parameters");
        }
        self.advance();

        self.skip_whitespace_not_newline();

        // Expect '=' followed by body expression
        if self.current_char() != Some('=') {
            bail!("Expected '=' after function parameters");
        }
        self.advance();

        // Make sure it's not '=>'
        if self.current_char() == Some('>') {
            bail!("Use '=' not '=>' for function definitions. For lambdas, use 'f = x => expr'");
        }

        self.skip_whitespace_not_newline();

        // Parse function body
        let body = self.expr()?;

        Ok(Statement::FuncDef {
            name,
            params,
            body: Box::new(body),
        })
    }

    /// Check if the next characters match a keyword (followed by non-alphanumeric)
    fn check_keyword(&self, keyword: &str) -> bool {
        let remaining = &self.input[self.pos..];
        if remaining.starts_with(keyword) {
            let after = remaining.chars().nth(keyword.len());
            match after {
                None => true,
                Some(c) => !c.is_ascii_alphanumeric() && c != '_',
            }
        } else {
            false
        }
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

    /// Skip empty lines (whitespace-only lines) and comment-only lines
    fn skip_empty_lines(&mut self) {
        loop {
            let start = self.pos;
            self.skip_whitespace_not_newline();
            self.skip_comments();
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

    /// Peek at the next character after current
    fn peek_char(&self) -> Option<char> {
        let mut chars = self.input[self.pos..].chars();
        chars.next();
        chars.next()
    }

    /// Advance position by one character.
    fn advance(&mut self) {
        if let Some(c) = self.current_char() {
            self.pos += c.len_utf8();
        }
    }

    /// Advance position by n characters
    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
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

    /// Parse: expr = logical_or
    fn expr(&mut self) -> Result<Expr> {
        self.logical_or()
    }

    /// Parse: logical_or = logical_and (('or' | '||') logical_and)*
    fn logical_or(&mut self) -> Result<Expr> {
        let mut left = self.logical_and()?;

        loop {
            self.skip_whitespace();
            // Check for 'or' keyword or '||' operator
            let is_or = self.check_keyword("or") || self.matches("||");
            if is_or {
                // Advance past the operator (both are 2 chars)
                self.advance_n(2);
                let right = self.logical_and()?;
                left = Expr::binop(BinOp::Or, left, right);
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse: logical_and = equality (('and' | '&&') equality)*
    fn logical_and(&mut self) -> Result<Expr> {
        let mut left = self.equality()?;

        loop {
            self.skip_whitespace();
            if self.check_keyword("and") {
                self.advance_n(3);
                let right = self.equality()?;
                left = Expr::binop(BinOp::And, left, right);
            } else if self.matches("&&") {
                self.advance_n(2);
                let right = self.equality()?;
                left = Expr::binop(BinOp::And, left, right);
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse: equality = comparison (('==' | '!=') comparison)*
    fn equality(&mut self) -> Result<Expr> {
        let mut left = self.comparison()?;

        loop {
            self.skip_whitespace();
            if self.matches("==") {
                self.advance_n(2);
                let right = self.comparison()?;
                left = Expr::binop(BinOp::Eq, left, right);
            } else if self.matches("!=") {
                self.advance_n(2);
                let right = self.comparison()?;
                left = Expr::binop(BinOp::Ne, left, right);
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse: comparison = term (('<' | '>' | '<=' | '>=') term)*
    fn comparison(&mut self) -> Result<Expr> {
        let mut left = self.term()?;

        loop {
            self.skip_whitespace();
            if self.matches("<=") {
                self.advance_n(2);
                let right = self.term()?;
                left = Expr::binop(BinOp::Le, left, right);
            } else if self.matches(">=") {
                self.advance_n(2);
                let right = self.term()?;
                left = Expr::binop(BinOp::Ge, left, right);
            } else if self.current_char() == Some('<') {
                self.advance();
                let right = self.term()?;
                left = Expr::binop(BinOp::Lt, left, right);
            } else if self.current_char() == Some('>') {
                self.advance();
                let right = self.term()?;
                left = Expr::binop(BinOp::Gt, left, right);
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Check if the input at current position matches a string
    fn matches(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// Parse: term = factor (('+' | '-') factor)*
    fn term(&mut self) -> Result<Expr> {
        let mut left = self.factor()?;

        loop {
            self.skip_whitespace();
            match self.current_char() {
                Some('+') => {
                    self.advance();
                    let right = self.factor()?;
                    left = Expr::binop(BinOp::Add, left, right);
                }
                Some('-') => {
                    self.advance();
                    let right = self.factor()?;
                    left = Expr::binop(BinOp::Sub, left, right);
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse: factor = power (('*' | '/' | '%') power)*
    fn factor(&mut self) -> Result<Expr> {
        let mut left = self.power()?;

        loop {
            self.skip_whitespace();
            match self.current_char() {
                Some('*') if self.peek_char() != Some('*') => {
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
        } else if self.matches("**") {
            self.advance_n(2);
            let exp = self.power()?;
            Ok(Expr::binop(BinOp::Pow, base, exp))
        } else {
            Ok(base)
        }
    }

    /// Parse: unary = ('not' | '!' | '-')* call
    fn unary(&mut self) -> Result<Expr> {
        self.skip_whitespace();

        if self.check_keyword("not") {
            self.advance_n(3);
            let expr = self.unary()?;
            Ok(Expr::unary(UnaryOp::Not, expr))
        } else if self.current_char() == Some('!') && self.peek_char() != Some('=') {
            self.advance();
            let expr = self.unary()?;
            Ok(Expr::unary(UnaryOp::Not, expr))
        } else if self.current_char() == Some('-') {
            self.advance();
            let expr = self.unary()?;
            Ok(Expr::unary(UnaryOp::Neg, expr))
        } else {
            self.call()
        }
    }

    /// Parse: call = identifier '(' args? ')' | constant | variable | primary
    fn call(&mut self) -> Result<Expr> {
        self.skip_whitespace();

        // Check if it starts with a letter (identifier)
        if let Some(c) = self.current_char()
            && (c.is_ascii_alphabetic() || c == '_') {
                // Check for keywords first
                if self.check_keyword("if") {
                    return self.conditional();
                }
                if self.check_keyword("true") {
                    self.advance_n(4);
                    return Ok(Expr::constant("true"));
                }
                if self.check_keyword("false") {
                    self.advance_n(5);
                    return Ok(Expr::constant("false"));
                }

                let name = self.identifier();

                self.skip_whitespace();

                // Check for lambda: identifier => expr
                if self.matches("=>") {
                    self.advance_n(2);
                    self.skip_whitespace();
                    let body = self.expr()?;
                    return Ok(Expr::lambda(vec![name], body));
                }

                if self.current_char() == Some('(') {
                    // Function call
                    self.advance();
                    self.skip_whitespace();

                    let mut args = Vec::new();
                    if self.current_char() != Some(')') {
                        loop {
                            self.skip_whitespace();
                            let arg = self.expr()?;
                            args.push(arg);
                            self.skip_whitespace();
                            match self.current_char() {
                                Some(',') => {
                                    self.advance();
                                }
                                Some(')') => break,
                                _ => bail!("Expected ',' or ')' in function call"),
                            }
                        }
                    }

                    if self.current_char() != Some(')') {
                        bail!(
                            "Expected ')' after function arguments at position {}",
                            self.pos
                        );
                    }
                    self.advance();
                    return Ok(Expr::func_call_multi(name, args));
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

    /// Parse conditional: if expr then expr else expr
    fn conditional(&mut self) -> Result<Expr> {
        // Skip 'if'
        self.advance_n(2);
        self.skip_whitespace();

        let condition = self.expr()?;

        self.skip_whitespace();
        if !self.check_keyword("then") {
            bail!("Expected 'then' in conditional expression");
        }
        self.advance_n(4);
        self.skip_whitespace();

        let then_branch = self.expr()?;

        self.skip_whitespace();
        if !self.check_keyword("else") {
            bail!("Expected 'else' in conditional expression");
        }
        self.advance_n(4);
        self.skip_whitespace();

        let else_branch = self.expr()?;

        Ok(Expr::conditional(condition, then_branch, else_branch))
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

    /// Parse: primary = number | '(' expr ')' | '(' params ')' '=>' expr
    fn primary(&mut self) -> Result<Expr> {
        self.skip_whitespace();

        match self.current_char() {
            Some('(') => {
                self.advance();
                self.skip_whitespace();

                // Could be: (expr) or (params) => expr (lambda)
                // Try to detect lambda by looking for identifier followed by , or )
                let start_pos = self.pos;
                let mut is_lambda = false;
                let mut params = Vec::new();

                // Check if this looks like parameter list
                if let Some(c) = self.current_char()
                    && (c.is_ascii_alphabetic() || c == '_' || c == ')') {
                        // Try parsing as parameters
                        if c != ')' {
                            let first_param = self.identifier();
                            self.skip_whitespace();

                            match self.current_char() {
                                Some(',') | Some(')') => {
                                    // Looks like parameter list
                                    params.push(first_param);
                                    while self.current_char() == Some(',') {
                                        self.advance();
                                        self.skip_whitespace();
                                        let param = self.identifier();
                                        if param.is_empty() {
                                            // Not a valid parameter list
                                            break;
                                        }
                                        params.push(param);
                                        self.skip_whitespace();
                                    }

                                    if self.current_char() == Some(')') {
                                        self.advance();
                                        self.skip_whitespace();
                                        if self.matches("=>") {
                                            is_lambda = true;
                                        }
                                    }
                                }
                                _ => {
                                    // Not a parameter list, rewind
                                    self.pos = start_pos;
                                }
                            }
                        } else {
                            // Empty params: () => expr
                            self.advance(); // skip )
                            self.skip_whitespace();
                            if self.matches("=>") {
                                is_lambda = true;
                            } else {
                                // Rewind - it was () without =>
                                self.pos = start_pos;
                            }
                        }
                    }

                if is_lambda {
                    // Parse lambda body
                    self.advance_n(2); // skip =>
                    self.skip_whitespace();
                    let body = self.expr()?;
                    return Ok(Expr::lambda(params, body));
                }

                // Not a lambda, rewind and parse as parenthesized expression
                self.pos = start_pos;
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

    fn parse_program(input: &str) -> Result<f64> {
        Parser::new(input).parse_program()?.eval()
    }

    // ==================== Basic Numbers ====================

    #[test]
    fn test_basic_numbers() {
        assert_eq!(parse("42").unwrap(), 42.0);
        assert_eq!(parse("3.14").unwrap(), 3.14);
        assert_eq!(parse("0.5").unwrap(), 0.5);
        assert_eq!(parse("0").unwrap(), 0.0);
        assert_eq!(parse("1000000").unwrap(), 1000000.0);
    }

    // ==================== Arithmetic Operations ====================

    #[test]
    fn test_basic_operations() {
        assert_eq!(parse("2 + 2").unwrap(), 4.0);
        assert_eq!(parse("10 - 3").unwrap(), 7.0);
        assert_eq!(parse("4 * 5").unwrap(), 20.0);
        assert_eq!(parse("15 / 3").unwrap(), 5.0);
    }

    #[test]
    fn test_precedence() {
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
        assert_eq!(parse("--5").unwrap(), 5.0);
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

    // ==================== Power Operator ====================

    #[test]
    fn test_power() {
        assert_eq!(parse("2 ^ 3").unwrap(), 8.0);
        assert_eq!(parse("2 ^ 10").unwrap(), 1024.0);
        assert_eq!(parse("4 ^ 0.5").unwrap(), 2.0);
        assert_eq!(parse("27 ^ (1/3)").unwrap(), 3.0);
    }

    #[test]
    fn test_power_double_star() {
        assert_eq!(parse("2 ** 3").unwrap(), 8.0);
        assert_eq!(parse("2 ** 10").unwrap(), 1024.0);
        assert_eq!(parse("2 ** 3 ** 2").unwrap(), 512.0);
        assert_eq!(parse("2 * 3 ** 2").unwrap(), 18.0);
    }

    #[test]
    fn test_power_right_associative() {
        assert_eq!(parse("2 ^ 3 ^ 2").unwrap(), 512.0);
    }

    #[test]
    fn test_power_precedence() {
        assert_eq!(parse("2 * 3 ^ 2").unwrap(), 18.0);
        assert_eq!(parse("2 ^ 3 * 2").unwrap(), 16.0);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(parse("10 % 3").unwrap(), 1.0);
        assert_eq!(parse("15 % 4").unwrap(), 3.0);
        assert_eq!(parse("10 % 5").unwrap(), 0.0);
    }

    // ==================== Constants ====================

    #[test]
    fn test_constants() {
        let pi = parse("pi").unwrap();
        assert!((pi - std::f64::consts::PI).abs() < 1e-10);

        let e = parse("e").unwrap();
        assert!((e - std::f64::consts::E).abs() < 1e-10);

        let tau = parse("tau").unwrap();
        assert!((tau - std::f64::consts::TAU).abs() < 1e-10);

        assert_eq!(parse("true").unwrap(), 1.0);
        assert_eq!(parse("false").unwrap(), 0.0);
    }

    #[test]
    fn test_constants_in_expressions() {
        let result = parse("2 * pi").unwrap();
        assert!((result - std::f64::consts::TAU).abs() < 1e-10);

        let result = parse("e ^ 1").unwrap();
        assert!((result - std::f64::consts::E).abs() < 1e-10);
    }

    // ==================== Comparison Operators ====================

    #[test]
    fn test_comparison_operators() {
        assert_eq!(parse("5 == 5").unwrap(), 1.0);
        assert_eq!(parse("5 == 3").unwrap(), 0.0);
        assert_eq!(parse("5 != 3").unwrap(), 1.0);
        assert_eq!(parse("5 != 5").unwrap(), 0.0);
        assert_eq!(parse("3 < 5").unwrap(), 1.0);
        assert_eq!(parse("5 < 3").unwrap(), 0.0);
        assert_eq!(parse("5 > 3").unwrap(), 1.0);
        assert_eq!(parse("3 > 5").unwrap(), 0.0);
        assert_eq!(parse("3 <= 5").unwrap(), 1.0);
        assert_eq!(parse("5 <= 5").unwrap(), 1.0);
        assert_eq!(parse("6 <= 5").unwrap(), 0.0);
        assert_eq!(parse("5 >= 3").unwrap(), 1.0);
        assert_eq!(parse("5 >= 5").unwrap(), 1.0);
        assert_eq!(parse("4 >= 5").unwrap(), 0.0);
    }

    #[test]
    fn test_chained_comparisons() {
        // (5 > 3) == 1 -> 1 == 1 -> 1
        assert_eq!(parse("(5 > 3) == 1").unwrap(), 1.0);
        // 2 + 3 > 4 -> 5 > 4 -> 1
        assert_eq!(parse("2 + 3 > 4").unwrap(), 1.0);
    }

    // ==================== Logical Operators ====================

    #[test]
    fn test_logical_and() {
        assert_eq!(parse("1 and 1").unwrap(), 1.0);
        assert_eq!(parse("1 and 0").unwrap(), 0.0);
        assert_eq!(parse("0 and 1").unwrap(), 0.0);
        assert_eq!(parse("0 and 0").unwrap(), 0.0);
        assert_eq!(parse("1 && 1").unwrap(), 1.0);
        assert_eq!(parse("1 && 0").unwrap(), 0.0);
    }

    #[test]
    fn test_logical_or() {
        assert_eq!(parse("1 or 1").unwrap(), 1.0);
        assert_eq!(parse("1 or 0").unwrap(), 1.0);
        assert_eq!(parse("0 or 1").unwrap(), 1.0);
        assert_eq!(parse("0 or 0").unwrap(), 0.0);
        assert_eq!(parse("0 || 1").unwrap(), 1.0);
    }

    #[test]
    fn test_logical_not() {
        assert_eq!(parse("not 0").unwrap(), 1.0);
        assert_eq!(parse("not 1").unwrap(), 0.0);
        assert_eq!(parse("not 42").unwrap(), 0.0);
        assert_eq!(parse("!0").unwrap(), 1.0);
        assert_eq!(parse("!1").unwrap(), 0.0);
        assert_eq!(parse("!!1").unwrap(), 1.0);
    }

    #[test]
    fn test_logical_precedence() {
        // or has lower precedence than and
        assert_eq!(parse("1 or 0 and 0").unwrap(), 1.0); // 1 or (0 and 0) = 1 or 0 = 1
        assert_eq!(parse("0 and 0 or 1").unwrap(), 1.0); // (0 and 0) or 1 = 0 or 1 = 1

        // comparison has higher precedence than logical
        assert_eq!(parse("5 > 3 and 2 < 4").unwrap(), 1.0); // (5>3) and (2<4) = 1 and 1 = 1
    }

    // ==================== Conditional Expressions ====================

    #[test]
    fn test_conditional_basic() {
        assert_eq!(parse("if 1 then 10 else 20").unwrap(), 10.0);
        assert_eq!(parse("if 0 then 10 else 20").unwrap(), 20.0);
        assert_eq!(parse("if true then 1 else 0").unwrap(), 1.0);
        assert_eq!(parse("if false then 1 else 0").unwrap(), 0.0);
    }

    #[test]
    fn test_conditional_with_comparison() {
        assert_eq!(parse("if 5 > 3 then 100 else 200").unwrap(), 100.0);
        assert_eq!(parse("if 5 < 3 then 100 else 200").unwrap(), 200.0);
    }

    #[test]
    fn test_conditional_nested() {
        // if 1 then (if 0 then 1 else 2) else 3
        assert_eq!(parse("if 1 then if 0 then 1 else 2 else 3").unwrap(), 2.0);
    }

    #[test]
    fn test_conditional_with_arithmetic() {
        assert_eq!(parse("if 1 then 2 + 3 else 4 * 5").unwrap(), 5.0);
        assert_eq!(parse("if 0 then 2 + 3 else 4 * 5").unwrap(), 20.0);
    }

    // ==================== Single-Argument Functions ====================

    #[test]
    fn test_trig_functions() {
        assert!((parse("sin(0)").unwrap() - 0.0).abs() < 1e-10);
        assert!((parse("cos(0)").unwrap() - 1.0).abs() < 1e-10);
        assert!((parse("tan(0)").unwrap() - 0.0).abs() < 1e-10);
        assert!((parse("sin(pi/2)").unwrap() - 1.0).abs() < 1e-10);
        assert!((parse("cos(pi)").unwrap() - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_trig_functions() {
        assert!((parse("asin(0)").unwrap() - 0.0).abs() < 1e-10);
        assert!((parse("acos(1)").unwrap() - 0.0).abs() < 1e-10);
        assert!((parse("atan(0)").unwrap() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_hyperbolic_functions() {
        assert!((parse("sinh(0)").unwrap() - 0.0).abs() < 1e-10);
        assert!((parse("cosh(0)").unwrap() - 1.0).abs() < 1e-10);
        assert!((parse("tanh(0)").unwrap() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_root_functions() {
        assert_eq!(parse("sqrt(16)").unwrap(), 4.0);
        assert_eq!(parse("sqrt(2)").unwrap(), 2.0_f64.sqrt());
        assert_eq!(parse("cbrt(27)").unwrap(), 3.0);
        assert_eq!(parse("cbrt(-8)").unwrap(), -2.0);
    }

    #[test]
    fn test_rounding_functions() {
        assert_eq!(parse("floor(3.7)").unwrap(), 3.0);
        assert_eq!(parse("floor(-3.7)").unwrap(), -4.0);
        assert_eq!(parse("ceil(3.2)").unwrap(), 4.0);
        assert_eq!(parse("ceil(-3.2)").unwrap(), -3.0);
        assert_eq!(parse("round(3.5)").unwrap(), 4.0);
        assert_eq!(parse("round(3.4)").unwrap(), 3.0);
        assert_eq!(parse("trunc(3.7)").unwrap(), 3.0);
        assert_eq!(parse("trunc(-3.7)").unwrap(), -3.0);
    }

    #[test]
    fn test_log_functions() {
        assert!((parse("ln(e)").unwrap() - 1.0).abs() < 1e-10);
        assert!((parse("log2(8)").unwrap() - 3.0).abs() < 1e-10);
        assert!((parse("log10(100)").unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_exp_function() {
        assert!((parse("exp(1)").unwrap() - std::f64::consts::E).abs() < 1e-10);
        assert!((parse("exp(0)").unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_other_single_arg_functions() {
        assert_eq!(parse("abs(-5)").unwrap(), 5.0);
        assert_eq!(parse("abs(5)").unwrap(), 5.0);
        assert_eq!(parse("sign(-5)").unwrap(), -1.0);
        assert_eq!(parse("sign(5)").unwrap(), 1.0);
        // Note: Rust's signum returns 1.0 for positive zero
        assert_eq!(parse("sign(0)").unwrap(), 1.0);
        assert!((parse("fract(3.75)").unwrap() - 0.75).abs() < 1e-10);
    }

    // ==================== Multi-Argument Functions ====================

    #[test]
    fn test_two_arg_functions() {
        assert_eq!(parse("max(3, 7)").unwrap(), 7.0);
        assert_eq!(parse("max(-5, -2)").unwrap(), -2.0);
        assert_eq!(parse("min(3, 7)").unwrap(), 3.0);
        assert_eq!(parse("min(-5, -2)").unwrap(), -5.0);
        assert_eq!(parse("pow(2, 10)").unwrap(), 1024.0);
        assert_eq!(parse("hypot(3, 4)").unwrap(), 5.0);
        assert!((parse("atan2(1, 1)").unwrap() - std::f64::consts::FRAC_PI_4).abs() < 1e-10);
        assert!((parse("log(8, 2)").unwrap() - 3.0).abs() < 1e-10);
        assert_eq!(parse("mod(10, 3)").unwrap(), 1.0);
    }

    #[test]
    fn test_three_arg_functions() {
        assert_eq!(parse("clamp(15, 0, 10)").unwrap(), 10.0);
        assert_eq!(parse("clamp(-5, 0, 10)").unwrap(), 0.0);
        assert_eq!(parse("clamp(5, 0, 10)").unwrap(), 5.0);
        assert_eq!(parse("lerp(0, 10, 0.5)").unwrap(), 5.0);
        assert_eq!(parse("lerp(0, 100, 0.25)").unwrap(), 25.0);
    }

    #[test]
    fn test_variadic_functions() {
        assert_eq!(parse("sum(1, 2, 3, 4, 5)").unwrap(), 15.0);
        assert_eq!(parse("sum(10)").unwrap(), 10.0);
        assert_eq!(parse("avg(2, 4, 6)").unwrap(), 4.0);
        assert_eq!(parse("avg(10)").unwrap(), 10.0);
    }

    // ==================== Print Function ====================

    #[test]
    fn test_print_function() {
        // print returns its argument
        assert_eq!(parse("print(42)").unwrap(), 42.0);
        assert_eq!(parse("print(3.14)").unwrap(), 3.14);
    }

    // ==================== Functions with Expressions ====================

    #[test]
    fn test_functions_with_expressions() {
        assert_eq!(parse("sqrt(9 + 16)").unwrap(), 5.0);
        assert_eq!(parse("abs(-3 * 4)").unwrap(), 12.0);
        assert_eq!(parse("2 * sqrt(4)").unwrap(), 4.0);
        assert_eq!(parse("max(2 + 3, 4 * 2)").unwrap(), 8.0);
    }

    #[test]
    fn test_nested_functions() {
        assert_eq!(parse("abs(abs(-5))").unwrap(), 5.0);
        assert_eq!(parse("sqrt(sqrt(16))").unwrap(), 2.0);
        assert_eq!(parse("max(min(10, 5), 3)").unwrap(), 5.0);
    }

    #[test]
    fn test_unknown_function() {
        assert!(parse("unknown(5)").is_err());
    }

    #[test]
    fn test_wrong_arg_count() {
        assert!(parse("sin(1, 2)").is_err());
        assert!(parse("max(1)").is_err());
        assert!(parse("clamp(1, 2)").is_err());
    }

    // ==================== Variables ====================

    #[test]
    fn test_variable_reference() {
        let ast = Parser::new("xyz").parse().unwrap();
        assert!(matches!(ast, Expr::Variable { ref name } if name == "xyz"));
        assert!(ast.eval().is_err());
    }

    // ==================== Programs with Variables ====================

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
        let prog = Parser::new("x = print(42); x + 1").parse_program().unwrap();
        assert_eq!(prog.eval().unwrap(), 43.0);
    }

    // ==================== Comments ====================

    #[test]
    fn test_comments() {
        assert_eq!(parse_program("# this is a comment\n42").unwrap(), 42.0);
        assert_eq!(parse_program("5 + 3 # inline comment").unwrap(), 8.0);
        assert_eq!(
            parse_program("x = 5 # set x\ny = 10 # set y\nx + y").unwrap(),
            15.0
        );
    }

    #[test]
    fn test_multiline_with_comments() {
        let prog = r#"
            # Calculate area of circle
            r = 5           # radius
            pi * r ^ 2      # area formula
        "#;
        let result = parse_program(prog).unwrap();
        assert!((result - 78.53981633974483).abs() < 1e-10);
    }

    // ==================== User-Defined Functions ====================

    #[test]
    fn test_function_definition_simple() {
        let prog = parse_program("def double(x) = x * 2\ndouble(5)").unwrap();
        assert_eq!(prog, 10.0);
    }

    #[test]
    fn test_function_definition_two_params() {
        let prog = parse_program("def add(a, b) = a + b\nadd(3, 4)").unwrap();
        assert_eq!(prog, 7.0);
    }

    #[test]
    fn test_function_definition_no_params() {
        let prog = parse_program("def answer() = 42\nanswer()").unwrap();
        assert_eq!(prog, 42.0);
    }

    #[test]
    fn test_function_uses_other_functions() {
        let prog = parse_program("def hyp(a, b) = sqrt(a^2 + b^2)\nhyp(3, 4)").unwrap();
        assert_eq!(prog, 5.0);
    }

    #[test]
    fn test_function_recursion() {
        let prog = parse_program(
            r#"
            def factorial(n) = if n <= 1 then 1 else n * factorial(n - 1)
            factorial(5)
        "#,
        )
        .unwrap();
        assert_eq!(prog, 120.0);
    }

    #[test]
    fn test_function_fibonacci() {
        let prog = parse_program(
            r#"
            def fib(n) = if n <= 1 then n else fib(n - 1) + fib(n - 2)
            fib(10)
        "#,
        )
        .unwrap();
        assert_eq!(prog, 55.0);
    }

    // ==================== Lambda Expressions ====================

    #[test]
    fn test_lambda_single_param() {
        let prog = parse_program("f = x => x * 2\nf(5)").unwrap();
        assert_eq!(prog, 10.0);
    }

    #[test]
    fn test_lambda_two_params() {
        let prog = parse_program("add = (a, b) => a + b\nadd(3, 4)").unwrap();
        assert_eq!(prog, 7.0);
    }

    #[test]
    fn test_lambda_no_params() {
        let prog = parse_program("answer = () => 42\nanswer()").unwrap();
        assert_eq!(prog, 42.0);
    }

    #[test]
    fn test_lambda_with_conditional() {
        let prog = parse_program("absval = x => if x < 0 then -x else x\nabsval(-5)").unwrap();
        assert_eq!(prog, 5.0);
    }

    // ==================== Closures ====================

    #[test]
    fn test_closure_captures_variable() {
        let prog = parse_program(
            r#"
            multiplier = 10
            scale = x => x * multiplier
            scale(5)
        "#,
        )
        .unwrap();
        assert_eq!(prog, 50.0);
    }

    #[test]
    fn test_closure_counter() {
        // Counter that increments captured variable
        // Note: Block syntax {} is not yet supported, so this is a placeholder test
        // Full mutable closures need block support in the parser
        let result = parse_program(
            r#"
            count = 0
            inc = () => count + 1
            inc()
        "#,
        )
        .unwrap();
        assert_eq!(result, 1.0);
    }

    // ==================== Complex Multi-line Programs ====================

    #[test]
    fn test_quadratic_formula() {
        let prog = parse_program(
            r#"
            # Quadratic formula for x^2 - 5x + 6 = 0
            a = 1
            b = -5
            c = 6
            discriminant = b^2 - 4*a*c
            x1 = (-b + sqrt(discriminant)) / (2*a)
            x1
        "#,
        )
        .unwrap();
        assert_eq!(prog, 3.0);
    }

    #[test]
    fn test_pythagorean_theorem() {
        let prog = parse_program(
            r#"
            a = 3
            b = 4
            c = sqrt(a^2 + b^2)
            c
        "#,
        )
        .unwrap();
        assert_eq!(prog, 5.0);
    }

    #[test]
    fn test_compound_interest() {
        let prog = parse_program(
            r#"
            # A = P(1 + r/n)^(nt)
            P = 1000
            r = 0.05
            n = 12
            t = 10
            A = P * (1 + r/n)^(n*t)
            round(A)
        "#,
        )
        .unwrap();
        assert_eq!(prog, 1647.0);
    }

    #[test]
    fn test_temperature_conversion() {
        let prog = parse_program(
            r#"
            celsius = 100
            fahrenheit = celsius * 9/5 + 32
            fahrenheit
        "#,
        )
        .unwrap();
        assert_eq!(prog, 212.0);
    }

    #[test]
    fn test_all_built_in_functions() {
        // Test that all built-in functions are accessible
        let results = vec![
            ("sin(0)", 0.0),
            ("cos(0)", 1.0),
            ("tan(0)", 0.0),
            ("asin(0)", 0.0),
            ("acos(1)", 0.0),
            ("atan(0)", 0.0),
            ("sinh(0)", 0.0),
            ("cosh(0)", 1.0),
            ("tanh(0)", 0.0),
            ("sqrt(4)", 2.0),
            ("cbrt(8)", 2.0),
            ("abs(-5)", 5.0),
            ("floor(3.9)", 3.0),
            ("ceil(3.1)", 4.0),
            ("round(3.5)", 4.0),
            ("trunc(3.9)", 3.0),
            ("exp(0)", 1.0),
            ("sign(-5)", -1.0),
            ("max(3, 7)", 7.0),
            ("min(3, 7)", 3.0),
            ("pow(2, 3)", 8.0),
            ("hypot(3, 4)", 5.0),
            ("mod(10, 3)", 1.0),
            ("clamp(15, 0, 10)", 10.0),
            ("lerp(0, 10, 0.5)", 5.0),
            ("sum(1, 2, 3)", 6.0),
            ("avg(2, 4, 6)", 4.0),
        ];

        for (expr, expected) in results {
            let result = parse(expr).unwrap();
            assert!(
                (result - expected).abs() < 1e-10,
                "Failed for {}: expected {}, got {}",
                expr,
                expected,
                result
            );
        }
    }

    #[test]
    fn test_print_in_program() {
        let prog = parse_program(
            r#"
            x = 5
            print(x)
            y = x * 2
            print(y)
            y + 10
        "#,
        )
        .unwrap();
        assert_eq!(prog, 20.0);
    }

    #[test]
    fn test_complex_expression_with_all_operators() {
        // Test combination of all operator types
        let prog = parse_program(
            r#"
            x = 10
            y = 5
            result = if x > y and not (x == 0) then x + y * 2 else x - y
            result
        "#,
        )
        .unwrap();
        assert_eq!(prog, 20.0); // 10 + 5*2 = 20
    }

    // ==================== AST Structure Tests ====================

    #[test]
    fn test_ast_structure() {
        let ast = Parser::new("2 + 3 * 4").parse().unwrap();
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
}
