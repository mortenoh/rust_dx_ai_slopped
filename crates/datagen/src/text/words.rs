//! Word lists and random word generation.
//!
//! Provides common word lists (adjectives, nouns, verbs) for generating
//! random text, usernames, identifiers, etc.

use rand::Rng;

/// Common English adjectives.
pub const ADJECTIVES: &[&str] = &[
    "quick",
    "lazy",
    "happy",
    "sad",
    "bright",
    "dark",
    "warm",
    "cold",
    "soft",
    "hard",
    "big",
    "small",
    "tall",
    "short",
    "fast",
    "slow",
    "loud",
    "quiet",
    "smooth",
    "rough",
    "clean",
    "dirty",
    "fresh",
    "stale",
    "sweet",
    "sour",
    "hot",
    "cool",
    "dry",
    "wet",
    "young",
    "old",
    "new",
    "ancient",
    "modern",
    "classic",
    "simple",
    "complex",
    "easy",
    "difficult",
    "brave",
    "calm",
    "clever",
    "eager",
    "gentle",
    "kind",
    "proud",
    "silly",
    "wise",
    "witty",
    "angry",
    "busy",
    "careful",
    "crazy",
    "curious",
    "famous",
    "friendly",
    "funny",
    "helpful",
    "honest",
    "hungry",
    "jolly",
    "lively",
    "lucky",
    "mighty",
    "nervous",
    "peaceful",
    "polite",
    "powerful",
    "rich",
    "shy",
    "sleepy",
    "strange",
    "strong",
    "tired",
    "ugly",
    "unusual",
    "useful",
    "weak",
    "wonderful",
];

/// Common English nouns.
pub const NOUNS: &[&str] = &[
    "apple",
    "ball",
    "cat",
    "dog",
    "eagle",
    "fish",
    "goat",
    "horse",
    "island",
    "jungle",
    "king",
    "lion",
    "moon",
    "night",
    "ocean",
    "planet",
    "queen",
    "river",
    "star",
    "tree",
    "umbrella",
    "valley",
    "water",
    "xylophone",
    "yacht",
    "zebra",
    "bird",
    "cloud",
    "door",
    "earth",
    "fire",
    "garden",
    "house",
    "ice",
    "jewel",
    "knife",
    "lamp",
    "mountain",
    "nest",
    "owl",
    "paper",
    "quilt",
    "rock",
    "sun",
    "tower",
    "universe",
    "village",
    "wind",
    "yard",
    "zone",
    "book",
    "chair",
    "desk",
    "floor",
    "glass",
    "hat",
    "ink",
    "jacket",
    "key",
    "leaf",
    "mirror",
    "nail",
    "orange",
    "pen",
    "ring",
    "shoe",
    "table",
    "vase",
    "wall",
    "window",
    "bridge",
    "castle",
    "diamond",
    "engine",
    "forest",
    "gold",
    "hammer",
    "iron",
    "jade",
    "ladder",
];

/// Common English verbs.
pub const VERBS: &[&str] = &[
    "run",
    "jump",
    "walk",
    "talk",
    "eat",
    "sleep",
    "read",
    "write",
    "think",
    "see",
    "hear",
    "feel",
    "touch",
    "smell",
    "taste",
    "know",
    "want",
    "need",
    "like",
    "love",
    "hate",
    "fear",
    "hope",
    "wish",
    "dream",
    "believe",
    "understand",
    "remember",
    "forget",
    "learn",
    "teach",
    "show",
    "tell",
    "ask",
    "answer",
    "help",
    "give",
    "take",
    "bring",
    "carry",
    "hold",
    "drop",
    "throw",
    "catch",
    "push",
    "pull",
    "open",
    "close",
    "start",
    "stop",
    "begin",
    "end",
    "continue",
    "wait",
    "stay",
    "leave",
    "go",
    "come",
    "return",
    "arrive",
    "build",
    "break",
    "fix",
    "create",
    "destroy",
    "find",
    "lose",
    "save",
    "spend",
    "buy",
    "sell",
    "pay",
    "work",
    "play",
    "rest",
    "move",
    "change",
    "grow",
    "shrink",
    "improve",
];

/// Common English adverbs.
pub const ADVERBS: &[&str] = &[
    "quickly",
    "slowly",
    "carefully",
    "carelessly",
    "happily",
    "sadly",
    "loudly",
    "quietly",
    "easily",
    "hardly",
    "nearly",
    "almost",
    "always",
    "never",
    "often",
    "rarely",
    "usually",
    "sometimes",
    "everywhere",
    "nowhere",
    "somewhere",
    "anywhere",
    "here",
    "there",
    "now",
    "then",
    "today",
    "yesterday",
    "tomorrow",
    "soon",
    "later",
    "early",
    "already",
    "still",
    "yet",
    "very",
    "really",
    "quite",
    "rather",
    "too",
    "enough",
    "just",
    "only",
    "even",
    "also",
];

/// Pick a random word from any category.
pub fn word<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let category = rng.random_range(0..3);
    match category {
        0 => adjective(rng),
        1 => noun(rng),
        _ => verb(rng),
    }
}

/// Pick a random adjective.
pub fn adjective<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    ADJECTIVES[rng.random_range(0..ADJECTIVES.len())]
}

/// Pick a random noun.
pub fn noun<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    NOUNS[rng.random_range(0..NOUNS.len())]
}

/// Pick a random verb.
pub fn verb<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    VERBS[rng.random_range(0..VERBS.len())]
}

/// Pick a random adverb.
pub fn adverb<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    ADVERBS[rng.random_range(0..ADVERBS.len())]
}

/// Generate a random phrase in "adjective noun" format.
pub fn adjective_noun<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} {}", adjective(rng), noun(rng))
}

/// Generate a random phrase in "adjective adjective noun" format.
pub fn double_adjective_noun<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} {} {}", adjective(rng), adjective(rng), noun(rng))
}

/// Generate a random identifier-style string (adjective-noun format).
pub fn slug<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{}-{}", adjective(rng), noun(rng))
}

/// Generate a random identifier with a number suffix.
pub fn slug_with_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{}-{}-{}",
        adjective(rng),
        noun(rng),
        rng.random_range(1..1000)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_adjective() {
        let mut rng = StdRng::seed_from_u64(42);
        let adj = adjective(&mut rng);
        assert!(ADJECTIVES.contains(&adj));
    }

    #[test]
    fn test_noun() {
        let mut rng = StdRng::seed_from_u64(42);
        let n = noun(&mut rng);
        assert!(NOUNS.contains(&n));
    }

    #[test]
    fn test_verb() {
        let mut rng = StdRng::seed_from_u64(42);
        let v = verb(&mut rng);
        assert!(VERBS.contains(&v));
    }

    #[test]
    fn test_adverb() {
        let mut rng = StdRng::seed_from_u64(42);
        let adv = adverb(&mut rng);
        assert!(ADVERBS.contains(&adv));
    }

    #[test]
    fn test_word() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..20 {
            let w = word(&mut rng);
            assert!(
                ADJECTIVES.contains(&w) || NOUNS.contains(&w) || VERBS.contains(&w),
                "Word '{}' not found in any list",
                w
            );
        }
    }

    #[test]
    fn test_adjective_noun() {
        let mut rng = StdRng::seed_from_u64(42);
        let phrase = adjective_noun(&mut rng);
        let parts: Vec<&str> = phrase.split(' ').collect();
        assert_eq!(parts.len(), 2);
        assert!(ADJECTIVES.contains(&parts[0]));
        assert!(NOUNS.contains(&parts[1]));
    }

    #[test]
    fn test_slug() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = slug(&mut rng);
        assert!(s.contains('-'));
        let parts: Vec<&str> = s.split('-').collect();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_slug_with_number() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = slug_with_number(&mut rng);
        let parts: Vec<&str> = s.split('-').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts[2].parse::<i32>().is_ok());
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let w = word(&mut *rng);
        assert!(!w.is_empty());
    }
}
