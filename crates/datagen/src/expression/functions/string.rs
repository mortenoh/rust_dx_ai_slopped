//! String manipulation functions for the expression DSL.
//!
//! Includes regexify, templatify, letterify, numerify, etc.

use rand::Rng;

use super::FunctionError;
use crate::expression::ast::Argument;

/// Generate a string matching a regex pattern.
///
/// Syntax: `regexify 'pattern'`
///
/// Supports:
/// - Character classes: `[A-Z]`, `[0-9]`, `[a-zA-Z]`
/// - Quantifiers: `{n}`, `{n,m}`
/// - Literal characters
///
/// Example: `regexify '[A-Z]{3}-[0-9]{4}'` -> "ABC-1234"
pub fn regexify<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    if args.is_empty() {
        return Err(FunctionError::wrong_arg_count("regexify", 1, 0));
    }

    let pattern = args[0]
        .as_string()
        .ok_or_else(|| FunctionError::wrong_arg_type("regexify", 0, "a string"))?;

    generate_from_regex(rng, pattern)
}

/// Generate a string by replacing template characters.
///
/// Syntax: `templatify 'template', 'char', 'replacement_class'`
///
/// Example: `templatify '###-###-####', '#', '0-9'` -> "123-456-7890"
pub fn templatify<R: Rng + ?Sized>(
    rng: &mut R,
    args: &[Argument],
) -> Result<String, FunctionError> {
    if args.len() < 3 {
        return Err(FunctionError::wrong_arg_count("templatify", 3, args.len()));
    }

    let template = args[0]
        .as_string()
        .ok_or_else(|| FunctionError::wrong_arg_type("templatify", 0, "a string"))?;

    let placeholder = args[1]
        .as_string()
        .ok_or_else(|| FunctionError::wrong_arg_type("templatify", 1, "a string"))?;

    let replacement = args[2]
        .as_string()
        .ok_or_else(|| FunctionError::wrong_arg_type("templatify", 2, "a string"))?;

    let placeholder_char = placeholder
        .chars()
        .next()
        .ok_or_else(|| FunctionError::new("templatify: placeholder must be a single character"))?;

    let chars: Vec<char> = parse_char_class(replacement);
    if chars.is_empty() {
        return Err(FunctionError::new("templatify: replacement class is empty"));
    }

    let result: String = template
        .chars()
        .map(|c| {
            if c == placeholder_char {
                chars[rng.random_range(0..chars.len())]
            } else {
                c
            }
        })
        .collect();

    Ok(result)
}

/// Generate a string by replacing '?' with random letters and '#' with random digits.
///
/// Syntax: `exemplify 'pattern'`
///
/// Example: `exemplify '??-###'` -> "AB-123"
pub fn exemplify<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    if args.is_empty() {
        return Err(FunctionError::wrong_arg_count("exemplify", 1, 0));
    }

    let pattern = args[0]
        .as_string()
        .ok_or_else(|| FunctionError::wrong_arg_type("exemplify", 0, "a string"))?;

    let result: String = pattern
        .chars()
        .map(|c| match c {
            '?' => random_letter(rng),
            '#' => random_digit(rng),
            _ => c,
        })
        .collect();

    Ok(result)
}

/// Generate a string by replacing '?' with random letters and '#' with random digits.
/// Alias for exemplify.
///
/// Syntax: `bothify 'pattern'`
pub fn bothify<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    exemplify(rng, args)
}

/// Generate a string by replacing '?' with random letters.
///
/// Syntax: `letterify 'pattern'`
///
/// Example: `letterify '???-???'` -> "ABC-XYZ"
pub fn letterify<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    if args.is_empty() {
        return Err(FunctionError::wrong_arg_count("letterify", 1, 0));
    }

    let pattern = args[0]
        .as_string()
        .ok_or_else(|| FunctionError::wrong_arg_type("letterify", 0, "a string"))?;

    let result: String = pattern
        .chars()
        .map(|c| if c == '?' { random_letter(rng) } else { c })
        .collect();

    Ok(result)
}

/// Generate a string by replacing '#' with random digits.
///
/// Syntax: `numerify 'pattern'`
///
/// Example: `numerify '###-####'` -> "123-4567"
pub fn numerify<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    if args.is_empty() {
        return Err(FunctionError::wrong_arg_count("numerify", 1, 0));
    }

    let pattern = args[0]
        .as_string()
        .ok_or_else(|| FunctionError::wrong_arg_type("numerify", 0, "a string"))?;

    let result: String = pattern
        .chars()
        .map(|c| if c == '#' { random_digit(rng) } else { c })
        .collect();

    Ok(result)
}

/// Convert string to uppercase.
///
/// Syntax: `uppercase 'text'`
pub fn uppercase(args: &[Argument]) -> Result<String, FunctionError> {
    if args.is_empty() {
        return Err(FunctionError::wrong_arg_count("uppercase", 1, 0));
    }

    let text = args[0]
        .as_string()
        .ok_or_else(|| FunctionError::wrong_arg_type("uppercase", 0, "a string"))?;

    Ok(text.to_uppercase())
}

/// Convert string to lowercase.
///
/// Syntax: `lowercase 'text'`
pub fn lowercase(args: &[Argument]) -> Result<String, FunctionError> {
    if args.is_empty() {
        return Err(FunctionError::wrong_arg_count("lowercase", 1, 0));
    }

    let text = args[0]
        .as_string()
        .ok_or_else(|| FunctionError::wrong_arg_type("lowercase", 0, "a string"))?;

    Ok(text.to_lowercase())
}

/// Capitalize first letter of string.
///
/// Syntax: `capitalize 'text'`
pub fn capitalize(args: &[Argument]) -> Result<String, FunctionError> {
    if args.is_empty() {
        return Err(FunctionError::wrong_arg_count("capitalize", 1, 0));
    }

    let text = args[0]
        .as_string()
        .ok_or_else(|| FunctionError::wrong_arg_type("capitalize", 0, "a string"))?;

    let mut chars = text.chars();
    match chars.next() {
        None => Ok(String::new()),
        Some(first) => Ok(first.to_uppercase().chain(chars).collect()),
    }
}

// Helper functions

fn random_letter<R: Rng + ?Sized>(rng: &mut R) -> char {
    (b'A' + rng.random_range(0..26u8)) as char
}

fn random_digit<R: Rng + ?Sized>(rng: &mut R) -> char {
    (b'0' + rng.random_range(0..10u8)) as char
}

fn parse_char_class(class: &str) -> Vec<char> {
    let mut chars = Vec::new();

    // Handle range notation like "0-9", "A-Z", "a-z"
    if class.len() == 3 && class.chars().nth(1) == Some('-') {
        let start = class.chars().next().unwrap();
        let end = class.chars().nth(2).unwrap();
        for c in start..=end {
            chars.push(c);
        }
    } else {
        // Treat as literal characters
        chars.extend(class.chars());
    }

    chars
}

/// Generate a string from a simplified regex pattern.
fn generate_from_regex<R: Rng + ?Sized>(
    rng: &mut R,
    pattern: &str,
) -> Result<String, FunctionError> {
    let mut result = String::new();
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '[' => {
                // Parse character class
                let mut class_chars = Vec::new();
                let mut prev_char: Option<char> = None;

                while let Some(&next) = chars.peek() {
                    if next == ']' {
                        chars.next();
                        break;
                    }
                    chars.next();

                    if next == '-' {
                        if let Some(start) = prev_char {
                            if let Some(&end) = chars.peek() {
                                if end != ']' {
                                    chars.next();
                                    // Remove the last char (it was the start of range)
                                    class_chars.pop();
                                    for ch in start..=end {
                                        class_chars.push(ch);
                                    }
                                    prev_char = Some(end);
                                    continue;
                                }
                            }
                        }
                        class_chars.push('-');
                        prev_char = Some('-');
                    } else {
                        class_chars.push(next);
                        prev_char = Some(next);
                    }
                }

                // Check for quantifier
                let count = parse_quantifier(&mut chars);
                for _ in 0..count {
                    if !class_chars.is_empty() {
                        let idx = rng.random_range(0..class_chars.len());
                        result.push(class_chars[idx]);
                    }
                }
            }
            '\\' => {
                // Escape sequence
                if let Some(escaped) = chars.next() {
                    match escaped {
                        'd' => {
                            let count = parse_quantifier(&mut chars);
                            for _ in 0..count {
                                result.push(random_digit(rng));
                            }
                        }
                        'w' => {
                            let count = parse_quantifier(&mut chars);
                            let word_chars: Vec<char> =
                                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"
                                    .chars()
                                    .collect();
                            for _ in 0..count {
                                let idx = rng.random_range(0..word_chars.len());
                                result.push(word_chars[idx]);
                            }
                        }
                        _ => result.push(escaped),
                    }
                }
            }
            '{' | '+' | '*' | '?' => {
                // Skip standalone quantifiers (shouldn't happen in well-formed patterns)
            }
            _ => {
                // Literal character - check for quantifier
                let count = parse_quantifier(&mut chars);
                if count > 1 {
                    for _ in 0..count {
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
            }
        }
    }

    Ok(result)
}

fn parse_quantifier(chars: &mut std::iter::Peekable<std::str::Chars>) -> usize {
    match chars.peek() {
        Some(&'{') => {
            chars.next(); // consume '{'
            let mut num_str = String::new();
            let mut max_str = String::new();
            let mut in_max = false;

            while let Some(&c) = chars.peek() {
                if c == '}' {
                    chars.next();
                    break;
                } else if c == ',' {
                    chars.next();
                    in_max = true;
                } else if c.is_ascii_digit() {
                    chars.next();
                    if in_max {
                        max_str.push(c);
                    } else {
                        num_str.push(c);
                    }
                } else {
                    break;
                }
            }

            let min: usize = num_str.parse().unwrap_or(1);
            if in_max && !max_str.is_empty() {
                let max: usize = max_str.parse().unwrap_or(min);
                rand::rng().random_range(min..=max)
            } else {
                min
            }
        }
        Some(&'+') => {
            chars.next();
            rand::rng().random_range(1..=5) // Default 1-5 for +
        }
        Some(&'*') => {
            chars.next();
            rand::rng().random_range(0..=5) // Default 0-5 for *
        }
        Some(&'?') => {
            chars.next();
            rand::rng().random_range(0..=1)
        }
        _ => 1,
    }
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
    fn test_regexify_simple() {
        let mut rng = test_rng();
        let result = regexify(&mut rng, &[Argument::String("[A-Z]{3}".to_string())]).unwrap();
        assert_eq!(result.len(), 3);
        assert!(result.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_regexify_mixed() {
        let mut rng = test_rng();
        let result = regexify(
            &mut rng,
            &[Argument::String("[A-Z]{3}-[0-9]{4}".to_string())],
        )
        .unwrap();
        assert_eq!(result.len(), 8);
        assert!(result.chars().nth(3) == Some('-'));
    }

    #[test]
    fn test_templatify() {
        let mut rng = test_rng();
        let result = templatify(
            &mut rng,
            &[
                Argument::String("###-###".to_string()),
                Argument::String("#".to_string()),
                Argument::String("0-9".to_string()),
            ],
        )
        .unwrap();
        assert_eq!(result.len(), 7);
        assert!(result.chars().nth(3) == Some('-'));
    }

    #[test]
    fn test_exemplify() {
        let mut rng = test_rng();
        let result = exemplify(&mut rng, &[Argument::String("??-###".to_string())]).unwrap();
        assert_eq!(result.len(), 6);
        assert!(result.chars().take(2).all(|c| c.is_ascii_uppercase()));
        assert!(result.chars().nth(2) == Some('-'));
        assert!(result.chars().skip(3).all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_letterify() {
        let mut rng = test_rng();
        let result = letterify(&mut rng, &[Argument::String("???".to_string())]).unwrap();
        assert_eq!(result.len(), 3);
        assert!(result.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_numerify() {
        let mut rng = test_rng();
        let result = numerify(&mut rng, &[Argument::String("###".to_string())]).unwrap();
        assert_eq!(result.len(), 3);
        assert!(result.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_uppercase() {
        let result = uppercase(&[Argument::String("hello".to_string())]).unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_lowercase() {
        let result = lowercase(&[Argument::String("HELLO".to_string())]).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_capitalize() {
        let result = capitalize(&[Argument::String("hello world".to_string())]).unwrap();
        assert_eq!(result, "Hello world");
    }
}
