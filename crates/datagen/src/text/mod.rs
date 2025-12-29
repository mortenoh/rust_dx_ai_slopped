//! Text generation utilities.
//!
//! This module provides pattern-based text generation, word lists, and lorem ipsum.

pub mod lorem;
pub mod patterns;
pub mod words;

pub use lorem::{paragraph, paragraphs, sentence, sentences};
pub use patterns::from_pattern;
pub use words::{adjective, noun, verb, word};
