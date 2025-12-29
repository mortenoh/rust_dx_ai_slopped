//! Video game-related data generators.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Game title patterns.
static TITLE_PATTERNS: &[&str] = &[
    "The {adj} {noun}",
    "{noun} of {noun}",
    "{adj} {noun}",
    "{noun} Quest",
    "{noun} Wars",
    "{noun} Legends",
    "Super {noun}",
    "{noun} Adventures",
    "The {noun} Chronicles",
    "{noun} Simulator",
    "{noun} Tactics",
    "Call of {noun}",
    "{noun} Rising",
    "{adj} {noun} Online",
    "{noun} Heroes",
];

static TITLE_ADJECTIVES: &[&str] = &[
    "Dark",
    "Epic",
    "Ultimate",
    "Mega",
    "Super",
    "Hyper",
    "Cyber",
    "Neon",
    "Crystal",
    "Shadow",
    "Eternal",
    "Infinite",
    "Legendary",
    "Mystic",
    "Royal",
    "Ancient",
    "Future",
    "Final",
    "Total",
    "Grand",
];

static TITLE_NOUNS: &[&str] = &[
    "Warrior", "Knight", "Dragon", "Realm", "Kingdom", "Empire", "World", "Galaxy", "Force",
    "Power", "Fury", "Storm", "Battle", "Legend", "Hero", "Quest", "Saga", "Fantasy", "Dungeon",
    "Castle",
];

/// Gaming platforms.
static PLATFORMS: &[&str] = &[
    "PC",
    "PlayStation 5",
    "PlayStation 4",
    "Xbox Series X",
    "Xbox One",
    "Nintendo Switch",
    "Steam Deck",
    "iOS",
    "Android",
    "macOS",
    "Linux",
    "PlayStation VR2",
    "Meta Quest",
    "Nintendo 3DS",
    "PlayStation Vita",
];

/// Game genres.
static GENRES: &[&str] = &[
    "Action",
    "Adventure",
    "Role-Playing",
    "Simulation",
    "Strategy",
    "Sports",
    "Racing",
    "Fighting",
    "Puzzle",
    "Platformer",
    "Shooter",
    "Horror",
    "Survival",
    "Sandbox",
    "Open World",
    "MMORPG",
    "MOBA",
    "Battle Royale",
    "Roguelike",
    "Metroidvania",
    "Visual Novel",
    "Rhythm",
    "Card Game",
    "Tower Defense",
    "City Builder",
    "Farming Simulator",
    "Dating Simulator",
    "Educational",
    "Party Game",
    "Trivia",
];

/// Game development studios.
static STUDIOS: &[&str] = &[
    "Quantum Games",
    "Phoenix Interactive",
    "Nova Studios",
    "Stellar Entertainment",
    "Pixel Works",
    "Digital Dreams",
    "Infinite Games",
    "Crystal Dynamics",
    "Thunder Software",
    "Eclipse Studios",
    "Nebula Games",
    "Apex Interactive",
    "Frontier Games",
    "Summit Studios",
    "Horizon Entertainment",
    "Catalyst Games",
    "Vertex Interactive",
    "Prism Studios",
    "Ember Games",
    "Zenith Entertainment",
];

/// Generate a random video game title.
pub fn game_title<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, TITLE_PATTERNS);
    pattern
        .replace("{adj}", pick(rng, TITLE_ADJECTIVES))
        .replace("{noun}", pick(rng, TITLE_NOUNS))
}

/// Generate a random gaming platform.
pub fn game_platform<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, PLATFORMS)
}

/// Generate a random game genre.
pub fn game_genre<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, GENRES)
}

/// Generate a random game studio name.
pub fn game_studio<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, STUDIOS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_game_title() {
        let mut rng = StdRng::seed_from_u64(42);
        let title = game_title(&mut rng);
        assert!(!title.is_empty());
        assert!(!title.contains('{'));
    }

    #[test]
    fn test_game_platform() {
        let mut rng = StdRng::seed_from_u64(42);
        let platform = game_platform(&mut rng);
        assert!(PLATFORMS.contains(&platform));
    }

    #[test]
    fn test_game_genre() {
        let mut rng = StdRng::seed_from_u64(42);
        let genre = game_genre(&mut rng);
        assert!(GENRES.contains(&genre));
    }

    #[test]
    fn test_game_studio() {
        let mut rng = StdRng::seed_from_u64(42);
        let studio = game_studio(&mut rng);
        assert!(STUDIOS.contains(&studio));
    }
}
