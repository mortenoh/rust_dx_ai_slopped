//! Weighted random selection.
//!
//! Provides non-uniform probability distributions for random selection.
//!
//! # Example
//!
//! ```
//! use dx_datagen::selection::{WeightedItem, weighted_pick, WeightedSelector};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! // Using WeightedItem directly
//! let items = vec![
//!     WeightedItem { item: "common", weight: 0.7 },
//!     WeightedItem { item: "rare", weight: 0.2 },
//!     WeightedItem { item: "legendary", weight: 0.1 },
//! ];
//! let picked = weighted_pick(&mut rng, &items);
//!
//! // Using the builder pattern
//! let selector = WeightedSelector::new()
//!     .add("common", 70.0)
//!     .add("rare", 20.0)
//!     .add("legendary", 10.0);
//! let picked = selector.pick(&mut rng);
//! ```

use rand::Rng;

/// An item with an associated weight for weighted selection.
#[derive(Debug, Clone)]
pub struct WeightedItem<T> {
    /// The item value.
    pub item: T,
    /// The relative weight (higher = more likely to be selected).
    pub weight: f64,
}

impl<T> WeightedItem<T> {
    /// Create a new weighted item.
    pub fn new(item: T, weight: f64) -> Self {
        Self { item, weight }
    }
}

/// Pick a random element from weighted items.
///
/// Items with higher weights are more likely to be selected.
/// Weights don't need to sum to 1.0 - they're relative.
///
/// # Panics
///
/// Panics if `items` is empty.
pub fn weighted_pick<'a, R: ?Sized + Rng, T>(rng: &mut R, items: &'a [WeightedItem<T>]) -> &'a T {
    assert!(!items.is_empty(), "Cannot pick from empty slice");

    let total: f64 = items.iter().map(|i| i.weight.max(0.0)).sum();
    if total <= 0.0 {
        // All weights are zero or negative, fall back to uniform selection
        return &items[rng.random_range(0..items.len())].item;
    }

    let mut threshold = rng.random_range(0.0..total);

    for item in items {
        let weight = item.weight.max(0.0);
        threshold -= weight;
        if threshold <= 0.0 {
            return &item.item;
        }
    }

    // Fallback for floating-point edge cases
    &items.last().unwrap().item
}

/// Pick a random element using separate items and weights slices.
///
/// # Panics
///
/// Panics if `items` is empty or if `items` and `weights` have different lengths.
pub fn weighted_pick_from<'a, R: ?Sized + Rng, T>(
    rng: &mut R,
    items: &'a [T],
    weights: &[f64],
) -> &'a T {
    assert!(!items.is_empty(), "Cannot pick from empty slice");
    assert_eq!(
        items.len(),
        weights.len(),
        "Items and weights must have same length"
    );

    let total: f64 = weights.iter().map(|w| w.max(0.0)).sum();
    if total <= 0.0 {
        return &items[rng.random_range(0..items.len())];
    }

    let mut threshold = rng.random_range(0.0..total);

    for (i, &weight) in weights.iter().enumerate() {
        threshold -= weight.max(0.0);
        if threshold <= 0.0 {
            return &items[i];
        }
    }

    items.last().unwrap()
}

/// Builder for weighted selection with a fluent API.
#[derive(Debug, Clone, Default)]
pub struct WeightedSelector<T> {
    items: Vec<WeightedItem<T>>,
}

impl<T> WeightedSelector<T> {
    /// Create a new empty weighted selector.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add an item with a weight.
    pub fn add(mut self, item: T, weight: f64) -> Self {
        self.items.push(WeightedItem::new(item, weight));
        self
    }

    /// Add an item with a weight (mutable version).
    pub fn push(&mut self, item: T, weight: f64) -> &mut Self {
        self.items.push(WeightedItem::new(item, weight));
        self
    }

    /// Check if the selector is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Pick a random item based on weights.
    ///
    /// # Panics
    ///
    /// Panics if the selector is empty.
    pub fn pick<'a, R: ?Sized + Rng>(&'a self, rng: &mut R) -> &'a T {
        weighted_pick(rng, &self.items)
    }

    /// Pick the index of a random item based on weights.
    ///
    /// # Panics
    ///
    /// Panics if the selector is empty.
    pub fn pick_index<R: ?Sized + Rng>(&self, rng: &mut R) -> usize {
        assert!(!self.items.is_empty(), "Cannot pick from empty selector");

        let total: f64 = self.items.iter().map(|i| i.weight.max(0.0)).sum();
        if total <= 0.0 {
            return rng.random_range(0..self.items.len());
        }

        let mut threshold = rng.random_range(0.0..total);

        for (i, item) in self.items.iter().enumerate() {
            threshold -= item.weight.max(0.0);
            if threshold <= 0.0 {
                return i;
            }
        }

        self.items.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_weighted_pick_basic() {
        let mut rng = StdRng::seed_from_u64(42);
        let items = vec![
            WeightedItem::new("a", 1.0),
            WeightedItem::new("b", 1.0),
            WeightedItem::new("c", 1.0),
        ];

        // Just verify it returns something from the list
        for _ in 0..10 {
            let picked = weighted_pick(&mut rng, &items);
            assert!(["a", "b", "c"].contains(picked));
        }
    }

    #[test]
    fn test_weighted_pick_distribution() {
        let mut rng = StdRng::seed_from_u64(42);
        let items = vec![
            WeightedItem::new("high", 90.0),
            WeightedItem::new("low", 10.0),
        ];

        let mut high_count = 0;
        let iterations = 1000;

        for _ in 0..iterations {
            if *weighted_pick(&mut rng, &items) == "high" {
                high_count += 1;
            }
        }

        // High should be picked roughly 90% of the time
        let ratio = high_count as f64 / iterations as f64;
        assert!(ratio > 0.8, "High ratio was {}", ratio);
        assert!(ratio < 0.98, "High ratio was {}", ratio);
    }

    #[test]
    fn test_weighted_pick_from() {
        let mut rng = StdRng::seed_from_u64(42);
        let items = ["a", "b", "c"];
        let weights = [1.0, 2.0, 3.0];

        for _ in 0..10 {
            let picked = weighted_pick_from(&mut rng, &items, &weights);
            assert!(items.contains(picked));
        }
    }

    #[test]
    fn test_weighted_selector() {
        let mut rng = StdRng::seed_from_u64(42);
        let selector = WeightedSelector::new()
            .add("common", 70.0)
            .add("rare", 20.0)
            .add("legendary", 10.0);

        assert_eq!(selector.len(), 3);
        assert!(!selector.is_empty());

        for _ in 0..10 {
            let picked = selector.pick(&mut rng);
            assert!(["common", "rare", "legendary"].contains(picked));
        }
    }

    #[test]
    fn test_weighted_selector_pick_index() {
        let mut rng = StdRng::seed_from_u64(42);
        let selector = WeightedSelector::new()
            .add("a", 1.0)
            .add("b", 1.0)
            .add("c", 1.0);

        for _ in 0..10 {
            let idx = selector.pick_index(&mut rng);
            assert!(idx < 3);
        }
    }

    #[test]
    fn test_zero_weights_fallback() {
        let mut rng = StdRng::seed_from_u64(42);
        let items = vec![WeightedItem::new("a", 0.0), WeightedItem::new("b", 0.0)];

        // Should still work with uniform selection
        for _ in 0..10 {
            let picked = weighted_pick(&mut rng, &items);
            assert!(["a", "b"].contains(picked));
        }
    }

    #[test]
    fn test_single_item() {
        let mut rng = StdRng::seed_from_u64(42);
        let items = vec![WeightedItem::new("only", 1.0)];

        for _ in 0..10 {
            assert_eq!(*weighted_pick(&mut rng, &items), "only");
        }
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let items = vec![WeightedItem::new("a", 1.0), WeightedItem::new("b", 1.0)];

        let picked = weighted_pick(&mut *rng, &items);
        assert!(["a", "b"].contains(picked));
    }
}
