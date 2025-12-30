//! Option selection functions for the expression DSL.

use rand::Rng;

use super::FunctionError;
use crate::expression::ast::Argument;

/// Select one option randomly from the given choices.
///
/// Syntax: `options.option 'A', 'B', 'C'`
///
/// Example: `options.option 'red', 'green', 'blue'` -> "green"
pub fn option<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    if args.is_empty() {
        return Err(FunctionError::new(
            "options.option: at least one option is required",
        ));
    }

    let idx = rng.random_range(0..args.len());

    match &args[idx] {
        Argument::String(s) => Ok(s.clone()),
        Argument::Number(n) => Ok(n.to_string()),
        Argument::Boolean(b) => Ok(b.to_string()),
        Argument::Expression(_) => Err(FunctionError::new(
            "options.option: nested expressions should be resolved before calling",
        )),
    }
}

/// Select one option with weighted probability.
///
/// Syntax: `options.weighted 'A', 70, 'B', 20, 'C', 10`
///
/// Values can be percentages (must sum to 100) or arbitrary weights.
///
/// Example: `options.weighted 'common', 80, 'rare', 15, 'legendary', 5` -> "common"
pub fn weighted<R: Rng + ?Sized>(rng: &mut R, args: &[Argument]) -> Result<String, FunctionError> {
    if args.len() < 2 {
        return Err(FunctionError::new(
            "options.weighted: at least one value-weight pair is required",
        ));
    }

    if !args.len().is_multiple_of(2) {
        return Err(FunctionError::new(
            "options.weighted: arguments must be value-weight pairs",
        ));
    }

    // Collect value-weight pairs
    let mut options: Vec<(&Argument, f64)> = Vec::new();
    let mut total_weight = 0.0;

    for i in (0..args.len()).step_by(2) {
        let value = &args[i];
        let weight = args[i + 1]
            .as_f64()
            .ok_or_else(|| FunctionError::wrong_arg_type("options.weighted", i + 1, "a number"))?;

        if weight < 0.0 {
            return Err(FunctionError::new(
                "options.weighted: weights must be non-negative",
            ));
        }

        total_weight += weight;
        options.push((value, weight));
    }

    if total_weight <= 0.0 {
        return Err(FunctionError::new(
            "options.weighted: total weight must be positive",
        ));
    }

    // Select a random value based on weights
    let mut random_value = rng.random_range(0.0..total_weight);

    for (value, weight) in &options {
        if random_value < *weight {
            return match value {
                Argument::String(s) => Ok(s.clone()),
                Argument::Number(n) => Ok(n.to_string()),
                Argument::Boolean(b) => Ok(b.to_string()),
                Argument::Expression(_) => Err(FunctionError::new(
                    "options.weighted: nested expressions should be resolved before calling",
                )),
            };
        }
        random_value -= *weight;
    }

    // Fallback to last option (shouldn't happen with correct math)
    match &options.last().unwrap().0 {
        Argument::String(s) => Ok(s.clone()),
        Argument::Number(n) => Ok(n.to_string()),
        Argument::Boolean(b) => Ok(b.to_string()),
        Argument::Expression(_) => Err(FunctionError::new(
            "options.weighted: nested expressions should be resolved before calling",
        )),
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
    fn test_option() {
        let mut rng = test_rng();
        let args = vec![
            Argument::String("A".to_string()),
            Argument::String("B".to_string()),
            Argument::String("C".to_string()),
        ];
        let result = option(&mut rng, &args).unwrap();
        assert!(result == "A" || result == "B" || result == "C");
    }

    #[test]
    fn test_option_single() {
        let mut rng = test_rng();
        let args = vec![Argument::String("only".to_string())];
        let result = option(&mut rng, &args).unwrap();
        assert_eq!(result, "only");
    }

    #[test]
    fn test_option_numbers() {
        let mut rng = test_rng();
        let args = vec![
            Argument::Number(1.0),
            Argument::Number(2.0),
            Argument::Number(3.0),
        ];
        let result = option(&mut rng, &args).unwrap();
        assert!(result == "1" || result == "2" || result == "3");
    }

    #[test]
    fn test_weighted() {
        let mut rng = test_rng();
        let args = vec![
            Argument::String("common".to_string()),
            Argument::Number(80.0),
            Argument::String("rare".to_string()),
            Argument::Number(15.0),
            Argument::String("legendary".to_string()),
            Argument::Number(5.0),
        ];
        let result = weighted(&mut rng, &args).unwrap();
        assert!(result == "common" || result == "rare" || result == "legendary");
    }

    #[test]
    fn test_weighted_deterministic() {
        // Run many times and verify distribution roughly matches weights
        let mut common_count = 0;
        let mut rare_count = 0;
        let mut legendary_count = 0;

        for seed in 0..1000 {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let args = vec![
                Argument::String("common".to_string()),
                Argument::Number(80.0),
                Argument::String("rare".to_string()),
                Argument::Number(15.0),
                Argument::String("legendary".to_string()),
                Argument::Number(5.0),
            ];
            let result = weighted(&mut rng, &args).unwrap();
            match result.as_str() {
                "common" => common_count += 1,
                "rare" => rare_count += 1,
                "legendary" => legendary_count += 1,
                _ => panic!("Unexpected result"),
            }
        }

        // Check that distribution is roughly correct (within 10% margin)
        assert!(common_count > 700 && common_count < 900);
        assert!(rare_count > 100 && rare_count < 250);
        assert!(legendary_count > 20 && legendary_count < 100);
    }

    #[test]
    fn test_weighted_error_no_args() {
        let mut rng = test_rng();
        let result = weighted(&mut rng, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_weighted_error_odd_args() {
        let mut rng = test_rng();
        let args = vec![
            Argument::String("A".to_string()),
            Argument::Number(50.0),
            Argument::String("B".to_string()),
        ];
        let result = weighted(&mut rng, &args);
        assert!(result.is_err());
    }
}
