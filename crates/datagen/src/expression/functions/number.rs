//! Number generation functions for the expression DSL.

use rand::Rng;

use super::FunctionError;
use crate::expression::ast::Argument;

/// Generate a random number with optional digit count.
///
/// Syntax: `number` or `number digits`
///
/// Example: `number 5` -> "12345"
pub fn number<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    let digits = if args.is_empty() {
        1
    } else {
        args[0].as_usize().unwrap_or(1)
    };

    if digits == 0 {
        return Ok("0".to_string());
    }

    // Generate a number with the specified number of digits
    let min = if digits == 1 {
        0
    } else {
        10_i64.pow(digits as u32 - 1)
    };
    let max = 10_i64.pow(digits as u32) - 1;

    let num = rng.random_range(min..=max);
    Ok(num.to_string())
}

/// Generate a random number between min and max (inclusive).
///
/// Syntax: `Number.between min, max`
///
/// Example: `Number.between 1, 100` -> "42"
pub fn number_between<R: Rng + ?Sized>(
    rng: &mut R,
    args: &[Argument],
) -> Result<String, FunctionError> {
    if args.len() < 2 {
        return Err(FunctionError::wrong_arg_count(
            "Number.between",
            2,
            args.len(),
        ));
    }

    let min = args[0]
        .as_i64()
        .ok_or_else(|| FunctionError::wrong_arg_type("Number.between", 0, "a number"))?;

    let max = args[1]
        .as_i64()
        .ok_or_else(|| FunctionError::wrong_arg_type("Number.between", 1, "a number"))?;

    if min > max {
        return Err(FunctionError::new(
            "Number.between: min must be less than or equal to max",
        ));
    }

    let num = rng.random_range(min..=max);
    Ok(num.to_string())
}

/// Generate a random decimal number.
///
/// Syntax: `decimal` or `decimal min, max` or `decimal min, max, decimals`
///
/// Example: `decimal 0, 100, 2` -> "42.75"
pub fn decimal<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    let (min, max, decimals) = match args.len() {
        0 => (0.0, 1.0, 2),
        2 => {
            let min = args[0]
                .as_f64()
                .ok_or_else(|| FunctionError::wrong_arg_type("decimal", 0, "a number"))?;
            let max = args[1]
                .as_f64()
                .ok_or_else(|| FunctionError::wrong_arg_type("decimal", 1, "a number"))?;
            (min, max, 2)
        }
        n if n >= 3 => {
            let min = args[0]
                .as_f64()
                .ok_or_else(|| FunctionError::wrong_arg_type("decimal", 0, "a number"))?;
            let max = args[1]
                .as_f64()
                .ok_or_else(|| FunctionError::wrong_arg_type("decimal", 1, "a number"))?;
            let decimals = args[2]
                .as_usize()
                .ok_or_else(|| FunctionError::wrong_arg_type("decimal", 2, "a number"))?;
            (min, max, decimals)
        }
        _ => {
            return Err(FunctionError::new(
                "decimal: expected 0, 2, or 3+ arguments",
            ))
        }
    };

    if min > max {
        return Err(FunctionError::new(
            "decimal: min must be less than or equal to max",
        ));
    }

    let num: f64 = rng.random_range(min..max);
    Ok(format!("{:.prec$}", num, prec = decimals))
}

/// Generate a random positive number.
///
/// Syntax: `positive` or `positive max`
///
/// Example: `positive 1000` -> "742"
pub fn positive<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    let max = if args.is_empty() {
        i64::MAX
    } else {
        args[0]
            .as_i64()
            .ok_or_else(|| FunctionError::wrong_arg_type("positive", 0, "a number"))?
    };

    if max < 1 {
        return Err(FunctionError::new("positive: max must be at least 1"));
    }

    let num = rng.random_range(1..=max);
    Ok(num.to_string())
}

/// Generate a random negative number.
///
/// Syntax: `negative` or `negative min`
///
/// Example: `negative -1000` -> "-742"
pub fn negative<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    let min = if args.is_empty() {
        i64::MIN + 1 // Avoid overflow
    } else {
        args[0]
            .as_i64()
            .ok_or_else(|| FunctionError::wrong_arg_type("negative", 0, "a number"))?
    };

    if min >= 0 {
        return Err(FunctionError::new("negative: min must be negative"));
    }

    let num = rng.random_range(min..0);
    Ok(num.to_string())
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
    fn test_number() {
        let mut rng = test_rng();
        let result = number(&mut rng, &[Argument::Number(3.0)]).unwrap();
        assert_eq!(result.len(), 3);
        assert!(result.parse::<i64>().is_ok());
    }

    #[test]
    fn test_number_between() {
        let mut rng = test_rng();
        let result =
            number_between(&mut rng, &[Argument::Number(1.0), Argument::Number(100.0)]).unwrap();
        let num: i64 = result.parse().unwrap();
        assert!((1..=100).contains(&num));
    }

    #[test]
    fn test_decimal() {
        let mut rng = test_rng();
        let result = decimal(
            &mut rng,
            &[
                Argument::Number(0.0),
                Argument::Number(100.0),
                Argument::Number(2.0),
            ],
        )
        .unwrap();
        let num: f64 = result.parse().unwrap();
        assert!((0.0..100.0).contains(&num));
    }

    #[test]
    fn test_positive() {
        let mut rng = test_rng();
        let result = positive(&mut rng, &[Argument::Number(1000.0)]).unwrap();
        let num: i64 = result.parse().unwrap();
        assert!(num > 0);
        assert!(num <= 1000);
    }

    #[test]
    fn test_negative() {
        let mut rng = test_rng();
        let result = negative(&mut rng, &[Argument::Number(-1000.0)]).unwrap();
        let num: i64 = result.parse().unwrap();
        assert!(num < 0);
        assert!(num >= -1000);
    }
}
