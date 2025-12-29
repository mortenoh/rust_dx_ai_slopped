//! Text generation utilities.
//!
//! This module provides pattern-based text generation, word lists, lorem ipsum,
//! and template interpolation.

pub mod lorem;
pub mod patterns;
pub mod template;
pub mod words;

pub use lorem::{paragraph, paragraphs, sentence, sentences};
pub use patterns::from_pattern;
pub use template::{render, render_default, ProviderRegistry, Template};
pub use words::{adjective, noun, verb, word};
