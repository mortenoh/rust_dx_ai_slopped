//! Unique value generation.
//!
//! Ensures generated values are unique within a session.
//!
//! # Example
//!
//! ```
//! use dx_datagen::selection::UniqueGenerator;
//! use rand::{Rng, SeedableRng};
//! use rand::rngs::StdRng;
//!
//! let rng = StdRng::seed_from_u64(42);
//! let mut unique = UniqueGenerator::new(rng);
//!
//! // Generate unique integers
//! let a = unique.generate(|r| r.random_range(1..=10)).unwrap();
//! let b = unique.generate(|r| r.random_range(1..=10)).unwrap();
//! assert_ne!(a, b);
//!
//! // Clear to reset tracking
//! unique.clear();
//! ```

use rand::Rng;
use std::collections::HashSet;
use std::hash::Hash;

/// Error returned when unable to generate a unique value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniqueError {
    /// Number of attempts made before giving up.
    pub attempts: usize,
    /// Maximum retries allowed.
    pub max_retries: usize,
}

impl std::fmt::Display for UniqueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to generate unique value after {} attempts (max: {})",
            self.attempts, self.max_retries
        )
    }
}

impl std::error::Error for UniqueError {}

/// A generator that tracks previously generated values to ensure uniqueness.
///
/// Wraps an RNG and maintains a set of seen values for each type.
#[derive(Debug)]
pub struct UniqueGenerator<R: Rng> {
    rng: R,
    max_retries: usize,
}

impl<R: Rng> UniqueGenerator<R> {
    /// Create a new unique generator with the given RNG.
    ///
    /// Default max retries is 1000.
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            max_retries: 1000,
        }
    }

    /// Create a new unique generator with custom max retries.
    pub fn with_max_retries(rng: R, max_retries: usize) -> Self {
        Self { rng, max_retries }
    }

    /// Get a mutable reference to the underlying RNG.
    pub fn rng_mut(&mut self) -> &mut R {
        &mut self.rng
    }

    /// Get a reference to the underlying RNG.
    pub fn rng(&self) -> &R {
        &self.rng
    }

    /// Set the maximum number of retries.
    pub fn set_max_retries(&mut self, max_retries: usize) {
        self.max_retries = max_retries;
    }

    /// Generate a unique value using the provided generator function.
    ///
    /// Uses a fresh HashSet for tracking, so uniqueness is only guaranteed
    /// within this single call. For persistent uniqueness tracking across
    /// multiple calls, use `UniqueTracker`.
    pub fn generate<F, T>(&mut self, generator: F) -> Result<T, UniqueError>
    where
        F: Fn(&mut R) -> T,
        T: Hash + Eq + Clone,
    {
        let mut seen = HashSet::new();
        for attempts in 1..=self.max_retries {
            let value = generator(&mut self.rng);
            if seen.insert(value.clone()) {
                return Ok(value);
            }
            if attempts >= self.max_retries {
                return Err(UniqueError {
                    attempts,
                    max_retries: self.max_retries,
                });
            }
        }
        Err(UniqueError {
            attempts: self.max_retries,
            max_retries: self.max_retries,
        })
    }

    /// Clear is a no-op for UniqueGenerator since it doesn't persist state.
    /// Use UniqueTracker for persistent uniqueness tracking.
    pub fn clear(&mut self) {
        // No-op - this generator doesn't maintain persistent state
    }
}

/// Tracks unique values across multiple generation calls.
///
/// Unlike `UniqueGenerator`, this maintains a persistent set of seen values
/// that can be used across multiple generate calls.
#[derive(Debug)]
pub struct UniqueTracker<T> {
    seen: HashSet<T>,
    max_retries: usize,
}

impl<T: Hash + Eq> Default for UniqueTracker<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Hash + Eq> UniqueTracker<T> {
    /// Create a new unique tracker.
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
            max_retries: 1000,
        }
    }

    /// Create a new unique tracker with custom max retries.
    pub fn with_max_retries(max_retries: usize) -> Self {
        Self {
            seen: HashSet::new(),
            max_retries,
        }
    }

    /// Generate a unique value using the provided generator function.
    pub fn generate<R, F>(&mut self, rng: &mut R, generator: F) -> Result<T, UniqueError>
    where
        R: ?Sized + Rng,
        F: Fn(&mut R) -> T,
        T: Clone,
    {
        for attempts in 1..=self.max_retries {
            let value = generator(rng);
            if self.seen.insert(value.clone()) {
                return Ok(value);
            }
            if attempts >= self.max_retries {
                return Err(UniqueError {
                    attempts,
                    max_retries: self.max_retries,
                });
            }
        }
        Err(UniqueError {
            attempts: self.max_retries,
            max_retries: self.max_retries,
        })
    }

    /// Check if a value has been seen.
    pub fn contains(&self, value: &T) -> bool {
        self.seen.contains(value)
    }

    /// Mark a value as seen.
    pub fn insert(&mut self, value: T) -> bool {
        self.seen.insert(value)
    }

    /// Get the number of unique values seen.
    pub fn len(&self) -> usize {
        self.seen.len()
    }

    /// Check if no values have been seen.
    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }

    /// Clear all seen values.
    pub fn clear(&mut self) {
        self.seen.clear();
    }

    /// Set the maximum number of retries.
    pub fn set_max_retries(&mut self, max_retries: usize) {
        self.max_retries = max_retries;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_unique_generator_basic() {
        let rng = StdRng::seed_from_u64(42);
        let mut unique = UniqueGenerator::new(rng);

        // Generate should succeed on first call
        let result = unique.generate(|r| r.random_range(1..=1000));
        assert!(result.is_ok());
    }

    #[test]
    fn test_unique_tracker_basic() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut tracker: UniqueTracker<i32> = UniqueTracker::new();

        // Generate 10 unique values from a small range
        let mut values = Vec::new();
        for _ in 0..10 {
            let value = tracker
                .generate(&mut rng, |r| r.random_range(1..=100))
                .unwrap();
            values.push(value);
        }

        // All values should be unique
        let unique_count = values.iter().collect::<HashSet<_>>().len();
        assert_eq!(unique_count, 10);
    }

    #[test]
    fn test_unique_tracker_exhaustion() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut tracker: UniqueTracker<i32> = UniqueTracker::with_max_retries(100);

        // Generate all possible values (1-3)
        for _ in 0..3 {
            let _ = tracker.generate(&mut rng, |r| r.random_range(1..=3));
        }

        // Fourth should fail
        let result = tracker.generate(&mut rng, |r| r.random_range(1..=3));
        assert!(result.is_err());
    }

    #[test]
    fn test_unique_tracker_clear() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut tracker: UniqueTracker<i32> = UniqueTracker::with_max_retries(100);

        // Generate all values
        for _ in 0..3 {
            let _ = tracker.generate(&mut rng, |r| r.random_range(1..=3));
        }

        // Clear and try again - should succeed
        tracker.clear();
        let result = tracker.generate(&mut rng, |r| r.random_range(1..=3));
        assert!(result.is_ok());
    }

    #[test]
    fn test_unique_tracker_contains() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut tracker: UniqueTracker<i32> = UniqueTracker::new();

        tracker.insert(42);
        assert!(tracker.contains(&42));
        assert!(!tracker.contains(&99));

        let result = tracker.generate(&mut rng, |_| 42);
        // Should fail because 42 is already seen
        assert!(result.is_err());
    }

    #[test]
    fn test_unique_tracker_len() {
        let mut tracker: UniqueTracker<i32> = UniqueTracker::new();

        assert!(tracker.is_empty());
        assert_eq!(tracker.len(), 0);

        tracker.insert(1);
        tracker.insert(2);
        tracker.insert(3);

        assert!(!tracker.is_empty());
        assert_eq!(tracker.len(), 3);

        tracker.clear();
        assert!(tracker.is_empty());
    }

    #[test]
    fn test_unique_error_display() {
        let err = UniqueError {
            attempts: 100,
            max_retries: 100,
        };
        let msg = err.to_string();
        assert!(msg.contains("100"));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let mut tracker: UniqueTracker<i32> = UniqueTracker::new();

        let result = tracker.generate(&mut *rng, |r| r.random_range(1..=100));
        assert!(result.is_ok());
    }
}
