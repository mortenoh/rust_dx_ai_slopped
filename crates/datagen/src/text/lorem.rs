//! Lorem ipsum text generation.
//!
//! Generates placeholder text using the classic "Lorem ipsum" format.
//!
//! # Example
//!
//! ```
//! use dx_datagen::text::lorem::{words, sentence, paragraph, paragraphs};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let w = words(&mut rng, 5);
//! let s = sentence(&mut rng);
//! let p = paragraph(&mut rng);
//! ```

use rand::Rng;

/// Classic lorem ipsum words for generating placeholder text.
const LOREM_WORDS: &[&str] = &[
    "lorem",
    "ipsum",
    "dolor",
    "sit",
    "amet",
    "consectetur",
    "adipiscing",
    "elit",
    "sed",
    "do",
    "eiusmod",
    "tempor",
    "incididunt",
    "ut",
    "labore",
    "et",
    "dolore",
    "magna",
    "aliqua",
    "enim",
    "ad",
    "minim",
    "veniam",
    "quis",
    "nostrud",
    "exercitation",
    "ullamco",
    "laboris",
    "nisi",
    "aliquip",
    "ex",
    "ea",
    "commodo",
    "consequat",
    "duis",
    "aute",
    "irure",
    "in",
    "reprehenderit",
    "voluptate",
    "velit",
    "esse",
    "cillum",
    "fugiat",
    "nulla",
    "pariatur",
    "excepteur",
    "sint",
    "occaecat",
    "cupidatat",
    "non",
    "proident",
    "sunt",
    "culpa",
    "qui",
    "officia",
    "deserunt",
    "mollit",
    "anim",
    "id",
    "est",
    "laborum",
    "ac",
    "ante",
    "at",
    "auctor",
    "augue",
    "bibendum",
    "blandit",
    "condimentum",
    "congue",
    "cras",
    "cursus",
    "diam",
    "dictum",
    "dignissim",
    "donec",
    "dui",
    "efficitur",
    "egestas",
    "elementum",
    "eleifend",
    "eros",
    "euismod",
    "facilisis",
    "fames",
    "faucibus",
    "felis",
    "fermentum",
    "feugiat",
    "finibus",
    "gravida",
    "habitant",
    "hendrerit",
    "himenaeos",
    "iaculis",
    "imperdiet",
    "integer",
    "interdum",
    "justo",
    "lacinia",
    "lacus",
    "laoreet",
    "lectus",
    "leo",
    "libero",
    "ligula",
    "litora",
    "lobortis",
    "luctus",
    "maecenas",
    "malesuada",
    "massa",
    "mattis",
    "mauris",
    "maximus",
    "metus",
    "mi",
    "morbi",
    "nam",
    "nec",
    "neque",
    "nibh",
    "nullam",
    "nunc",
    "odio",
    "orci",
    "ornare",
    "pellentesque",
    "pharetra",
    "phasellus",
    "placerat",
    "porta",
    "porttitor",
    "posuere",
    "praesent",
    "pretium",
    "primis",
    "proin",
    "pulvinar",
    "purus",
    "quam",
    "quisque",
    "rhoncus",
    "risus",
    "rutrum",
    "sagittis",
    "sapien",
    "scelerisque",
    "semper",
    "senectus",
    "sociosqu",
    "sodales",
    "sollicitudin",
    "suscipit",
    "suspendisse",
    "tellus",
    "tincidunt",
    "torquent",
    "tortor",
    "tristique",
    "turpis",
    "ultrices",
    "ultricies",
    "urna",
    "varius",
    "vehicula",
    "vel",
    "vestibulum",
    "vitae",
    "vivamus",
    "viverra",
    "volutpat",
    "vulputate",
];

/// Generate a specified number of lorem ipsum words.
pub fn words<R: ?Sized + Rng>(rng: &mut R, count: usize) -> String {
    (0..count)
        .map(|_| LOREM_WORDS[rng.random_range(0..LOREM_WORDS.len())])
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a single lorem ipsum sentence.
pub fn sentence<R: ?Sized + Rng>(rng: &mut R) -> String {
    let word_count = rng.random_range(6..15);
    let text = words(rng, word_count);
    capitalize_first(&text) + "."
}

/// Generate multiple sentences.
pub fn sentences<R: ?Sized + Rng>(rng: &mut R, count: usize) -> String {
    (0..count)
        .map(|_| sentence(rng))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a paragraph of lorem ipsum text.
pub fn paragraph<R: ?Sized + Rng>(rng: &mut R) -> String {
    let sentence_count = rng.random_range(3..8);
    sentences(rng, sentence_count)
}

/// Generate multiple paragraphs of lorem ipsum text.
pub fn paragraphs<R: ?Sized + Rng>(rng: &mut R, count: usize) -> String {
    (0..count)
        .map(|_| paragraph(rng))
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Generate a title (capitalized words).
pub fn title<R: ?Sized + Rng>(rng: &mut R, word_count: usize) -> String {
    let text = words(rng, word_count);
    text.split_whitespace()
        .map(capitalize_first)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a short blurb (1-2 sentences).
pub fn blurb<R: ?Sized + Rng>(rng: &mut R) -> String {
    let count = rng.random_range(1..3);
    sentences(rng, count)
}

/// Generate lorem ipsum text starting with "Lorem ipsum dolor sit amet".
pub fn lorem_ipsum<R: ?Sized + Rng>(rng: &mut R, word_count: usize) -> String {
    const OPENING: &[&str] = &["Lorem", "ipsum", "dolor", "sit", "amet"];

    if word_count <= 5 {
        OPENING[..word_count].join(" ")
    } else {
        let remaining = word_count.saturating_sub(5);
        let extra = words(rng, remaining);
        format!("{}, {}", OPENING.join(" "), extra)
    }
}

/// Capitalize the first letter of a string.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_words() {
        let mut rng = StdRng::seed_from_u64(42);
        let w = words(&mut rng, 5);
        let word_count = w.split_whitespace().count();
        assert_eq!(word_count, 5);
    }

    #[test]
    fn test_sentence() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = sentence(&mut rng);
        assert!(s.ends_with('.'));
        // First letter should be capitalized
        assert!(s.chars().next().unwrap().is_uppercase());
    }

    #[test]
    fn test_sentences() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = sentences(&mut rng, 3);
        let period_count = s.matches('.').count();
        assert_eq!(period_count, 3);
    }

    #[test]
    fn test_paragraph() {
        let mut rng = StdRng::seed_from_u64(42);
        let p = paragraph(&mut rng);
        // Should have multiple sentences (3-7)
        let period_count = p.matches('.').count();
        assert!(period_count >= 3 && period_count <= 7);
    }

    #[test]
    fn test_paragraphs() {
        let mut rng = StdRng::seed_from_u64(42);
        let p = paragraphs(&mut rng, 3);
        // Should have 2 paragraph separators
        let separator_count = p.matches("\n\n").count();
        assert_eq!(separator_count, 2);
    }

    #[test]
    fn test_title() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = title(&mut rng, 4);
        // All words should be capitalized
        for word in t.split_whitespace() {
            assert!(word.chars().next().unwrap().is_uppercase());
        }
    }

    #[test]
    fn test_lorem_ipsum() {
        let mut rng = StdRng::seed_from_u64(42);
        let l = lorem_ipsum(&mut rng, 10);
        assert!(l.starts_with("Lorem ipsum dolor sit amet"));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(words(&mut rng1, 5), words(&mut rng2, 5));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let w = words(&mut *rng, 5);
        assert!(!w.is_empty());
    }
}
