//! Batch generation helpers.
//!
//! Provides utilities for generating multiple values at once.
//!
//! # Example
//!
//! ```
//! use dx_datagen::selection::{generate_batch, generate_batch_unique};
//! use rand::{Rng, SeedableRng};
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! // Generate 10 random integers
//! let numbers: Vec<i32> = generate_batch(&mut rng, 10, |r| r.random_range(1..=100));
//! assert_eq!(numbers.len(), 10);
//!
//! // Generate 5 unique integers
//! let unique: Vec<i32> = generate_batch_unique(&mut rng, 5, |r| r.random_range(1..=1000)).unwrap();
//! assert_eq!(unique.len(), 5);
//! ```

use super::unique::{UniqueError, UniqueTracker};
use rand::Rng;
use std::hash::Hash;

/// Generate a batch of values using the provided generator function.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `count` - Number of values to generate
/// * `generator` - Function that generates a single value
///
/// # Example
///
/// ```
/// use dx_datagen::selection::generate_batch;
/// use rand::{Rng, SeedableRng};
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let names: Vec<String> = generate_batch(&mut rng, 5, |r| {
///     format!("user_{}", r.random_range(1000..9999))
/// });
/// assert_eq!(names.len(), 5);
/// ```
pub fn generate_batch<R, F, T>(rng: &mut R, count: usize, generator: F) -> Vec<T>
where
    R: ?Sized + Rng,
    F: Fn(&mut R) -> T,
{
    (0..count).map(|_| generator(rng)).collect()
}

/// Generate a batch of unique values using the provided generator function.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `count` - Number of unique values to generate
/// * `generator` - Function that generates a single value
///
/// # Returns
///
/// `Ok(Vec<T>)` if successful, `Err(UniqueError)` if unable to generate
/// enough unique values after max retries.
///
/// # Example
///
/// ```
/// use dx_datagen::selection::generate_batch_unique;
/// use rand::{Rng, SeedableRng};
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let ids: Vec<i32> = generate_batch_unique(&mut rng, 5, |r| r.random_range(1..=100)).unwrap();
/// assert_eq!(ids.len(), 5);
/// // All values are unique
/// let unique: std::collections::HashSet<_> = ids.iter().collect();
/// assert_eq!(unique.len(), 5);
/// ```
pub fn generate_batch_unique<R, F, T>(
    rng: &mut R,
    count: usize,
    generator: F,
) -> Result<Vec<T>, UniqueError>
where
    R: ?Sized + Rng,
    F: Fn(&mut R) -> T,
    T: Hash + Eq + Clone,
{
    generate_batch_unique_with_retries(rng, count, generator, 1000)
}

/// Generate a batch of unique values with custom max retries.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `count` - Number of unique values to generate
/// * `generator` - Function that generates a single value
/// * `max_retries` - Maximum retries per value before giving up
pub fn generate_batch_unique_with_retries<R, F, T>(
    rng: &mut R,
    count: usize,
    generator: F,
    max_retries: usize,
) -> Result<Vec<T>, UniqueError>
where
    R: ?Sized + Rng,
    F: Fn(&mut R) -> T,
    T: Hash + Eq + Clone,
{
    let mut tracker: UniqueTracker<T> = UniqueTracker::with_max_retries(max_retries);
    let mut results = Vec::with_capacity(count);

    for _ in 0..count {
        results.push(tracker.generate(rng, &generator)?);
    }

    Ok(results)
}

/// Generate values until a predicate is satisfied.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `generator` - Function that generates a single value
/// * `predicate` - Function that returns true when generation should stop
/// * `max_iterations` - Maximum number of values to generate
///
/// # Example
///
/// ```
/// use dx_datagen::selection::generate_until;
/// use rand::{Rng, SeedableRng};
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// // Generate numbers until we get one > 95
/// let numbers: Vec<i32> = generate_until(
///     &mut rng,
///     |r| r.random_range(1..=100),
///     |n| *n > 95,
///     1000
/// );
/// assert!(!numbers.is_empty());
/// assert!(*numbers.last().unwrap() > 95);
/// ```
pub fn generate_until<R, F, T, P>(
    rng: &mut R,
    generator: F,
    predicate: P,
    max_iterations: usize,
) -> Vec<T>
where
    R: ?Sized + Rng,
    F: Fn(&mut R) -> T,
    P: Fn(&T) -> bool,
{
    let mut results = Vec::new();

    for _ in 0..max_iterations {
        let value = generator(rng);
        let done = predicate(&value);
        results.push(value);
        if done {
            break;
        }
    }

    results
}

/// Generate a batch and apply a transformation to each value.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `count` - Number of values to generate
/// * `generator` - Function that generates a single value
/// * `transform` - Function to transform each generated value
///
/// # Example
///
/// ```
/// use dx_datagen::selection::generate_batch_map;
/// use rand::{Rng, SeedableRng};
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let usernames: Vec<String> = generate_batch_map(
///     &mut rng,
///     5,
///     |r| r.random_range(1000..9999),
///     |id| format!("user_{}", id)
/// );
/// assert_eq!(usernames.len(), 5);
/// assert!(usernames[0].starts_with("user_"));
/// ```
pub fn generate_batch_map<R, F, T, M, U>(
    rng: &mut R,
    count: usize,
    generator: F,
    transform: M,
) -> Vec<U>
where
    R: ?Sized + Rng,
    F: Fn(&mut R) -> T,
    M: Fn(T) -> U,
{
    (0..count).map(|_| transform(generator(rng))).collect()
}

/// Generate a batch where some values may be None (nullable data).
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `count` - Number of values to generate
/// * `generator` - Function that generates a single value
/// * `null_probability` - Probability (0.0-1.0) that a value will be None
///
/// # Example
///
/// ```
/// use dx_datagen::selection::generate_batch_nullable;
/// use rand::{Rng, SeedableRng};
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let values: Vec<Option<i32>> = generate_batch_nullable(
///     &mut rng,
///     100,
///     |r| r.random_range(1..=100),
///     0.2  // 20% null
/// );
/// // Roughly 20% should be None
/// let null_count = values.iter().filter(|v| v.is_none()).count();
/// assert!(null_count > 0);
/// ```
pub fn generate_batch_nullable<R, F, T>(
    rng: &mut R,
    count: usize,
    generator: F,
    null_probability: f64,
) -> Vec<Option<T>>
where
    R: ?Sized + Rng,
    F: Fn(&mut R) -> T,
{
    let prob = null_probability.clamp(0.0, 1.0);
    (0..count)
        .map(|_| {
            if rng.random::<f64>() < prob {
                None
            } else {
                Some(generator(rng))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use std::collections::HashSet;

    #[test]
    fn test_generate_batch() {
        let mut rng = StdRng::seed_from_u64(42);
        let values: Vec<i32> = generate_batch(&mut rng, 10, |r| r.random_range(1..=100));

        assert_eq!(values.len(), 10);
        for v in &values {
            assert!(*v >= 1 && *v <= 100);
        }
    }

    #[test]
    fn test_generate_batch_empty() {
        let mut rng = StdRng::seed_from_u64(42);
        let values: Vec<i32> = generate_batch(&mut rng, 0, |r| r.random_range(1..=100));
        assert!(values.is_empty());
    }

    #[test]
    fn test_generate_batch_unique() {
        let mut rng = StdRng::seed_from_u64(42);
        let values: Vec<i32> =
            generate_batch_unique(&mut rng, 10, |r| r.random_range(1..=1000)).unwrap();

        assert_eq!(values.len(), 10);
        let unique: HashSet<_> = values.iter().collect();
        assert_eq!(unique.len(), 10);
    }

    #[test]
    fn test_generate_batch_unique_failure() {
        let mut rng = StdRng::seed_from_u64(42);
        // Try to generate 10 unique values from a range of 3
        let result: Result<Vec<i32>, _> =
            generate_batch_unique_with_retries(&mut rng, 10, |r| r.random_range(1..=3), 100);

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_until() {
        let mut rng = StdRng::seed_from_u64(42);
        let values: Vec<i32> =
            generate_until(&mut rng, |r| r.random_range(1..=100), |n| *n > 90, 1000);

        assert!(!values.is_empty());
        assert!(*values.last().unwrap() > 90);
    }

    #[test]
    fn test_generate_until_max() {
        let mut rng = StdRng::seed_from_u64(42);
        // Predicate never satisfied
        let values: Vec<i32> =
            generate_until(&mut rng, |r| r.random_range(1..=10), |n| *n > 100, 5);

        assert_eq!(values.len(), 5); // Hit max iterations
    }

    #[test]
    fn test_generate_batch_map() {
        let mut rng = StdRng::seed_from_u64(42);
        let values: Vec<String> = generate_batch_map(
            &mut rng,
            5,
            |r| r.random_range(1..=100),
            |n| format!("item_{}", n),
        );

        assert_eq!(values.len(), 5);
        for v in &values {
            assert!(v.starts_with("item_"));
        }
    }

    #[test]
    fn test_generate_batch_nullable() {
        let mut rng = StdRng::seed_from_u64(42);
        let values: Vec<Option<i32>> =
            generate_batch_nullable(&mut rng, 1000, |r| r.random_range(1..=100), 0.2);

        assert_eq!(values.len(), 1000);
        let null_count = values.iter().filter(|v| v.is_none()).count();
        // Should be roughly 20% (allow 10-30%)
        assert!(null_count > 100, "null_count was {}", null_count);
        assert!(null_count < 300, "null_count was {}", null_count);
    }

    #[test]
    fn test_generate_batch_nullable_zero() {
        let mut rng = StdRng::seed_from_u64(42);
        let values: Vec<Option<i32>> =
            generate_batch_nullable(&mut rng, 100, |r| r.random_range(1..=100), 0.0);

        assert!(values.iter().all(|v| v.is_some()));
    }

    #[test]
    fn test_generate_batch_nullable_all() {
        let mut rng = StdRng::seed_from_u64(42);
        let values: Vec<Option<i32>> =
            generate_batch_nullable(&mut rng, 100, |r| r.random_range(1..=100), 1.0);

        assert!(values.iter().all(|v| v.is_none()));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let values: Vec<i32> = generate_batch(&mut *rng, 5, |r| r.random_range(1..=100));
        assert_eq!(values.len(), 5);
    }
}
