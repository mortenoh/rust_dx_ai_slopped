//! Advanced selection utilities.
//!
//! This module provides:
//! - **weighted**: Weighted random selection for non-uniform distributions
//! - **unique**: Unique value generation with collision tracking
//! - **batch**: Batch generation helpers for multiple values

pub mod batch;
pub mod unique;
pub mod weighted;

pub use batch::{
    generate_batch, generate_batch_map, generate_batch_nullable, generate_batch_unique,
    generate_batch_unique_with_retries, generate_until,
};
pub use unique::{UniqueError, UniqueGenerator, UniqueTracker};
pub use weighted::{weighted_pick, weighted_pick_from, WeightedItem, WeightedSelector};
