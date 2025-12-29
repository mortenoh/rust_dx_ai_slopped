//! Advanced selection utilities.
//!
//! This module provides weighted random selection for non-uniform distributions.

pub mod weighted;

pub use weighted::{weighted_pick, weighted_pick_from, WeightedItem, WeightedSelector};
