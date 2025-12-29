//! Music-related data generators.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Artist name patterns.
static ARTIST_PATTERNS: &[&str] = &[
    "{first} {last}",
    "The {adj} {noun}s",
    "{noun} {noun}",
    "DJ {name}",
    "{adj} {noun}",
    "The {noun}s",
    "{name} and the {noun}s",
    "{adj} {adj}",
];

static ARTIST_FIRSTS: &[&str] = &[
    "John", "Sarah", "Michael", "Emma", "David", "Lisa", "James", "Anna", "Robert", "Maria",
    "Chris", "Jessica", "Alex", "Taylor", "Jordan", "Casey", "Morgan", "Riley", "Quinn", "Avery",
];

static ARTIST_LASTS: &[&str] = &[
    "Smith", "Jones", "Williams", "Brown", "Taylor", "Davis", "Wilson", "Moore", "Jackson",
    "Martin", "Lee", "Harris", "Clark", "Lewis", "Walker", "Hall", "Young", "King", "Wright",
    "Lopez",
];

static BAND_ADJECTIVES: &[&str] = &[
    "Electric", "Cosmic", "Velvet", "Silver", "Golden", "Neon", "Crystal", "Midnight", "Atomic",
    "Sonic", "Digital", "Mystic", "Wild", "Dark", "Bright",
];

static BAND_NOUNS: &[&str] = &[
    "Wolf",
    "Tiger",
    "Eagle",
    "Phoenix",
    "Dragon",
    "Thunder",
    "Lightning",
    "Storm",
    "Sun",
    "Moon",
    "Star",
    "Sky",
    "Wave",
    "Fire",
    "Ice",
    "Dream",
    "Ghost",
    "Spirit",
    "Soul",
    "Heart",
];

/// Album name patterns.
static ALBUM_PATTERNS: &[&str] = &[
    "The {adj} {noun}",
    "{noun} of {noun}",
    "{adj} {noun}",
    "Songs from the {noun}",
    "{noun} Dreams",
    "Welcome to {noun}",
    "Return to {noun}",
    "Beyond the {noun}",
    "Inside the {noun}",
    "{noun} Rising",
];

/// Song name patterns.
static SONG_PATTERNS: &[&str] = &[
    "{adj} {noun}",
    "The {noun}",
    "{noun} Tonight",
    "Dancing in the {noun}",
    "{noun} Heart",
    "Never {verb}",
    "Always {verb}",
    "{verb} Me",
    "Can't Stop {verb}",
    "{noun} Love",
    "Summer {noun}",
    "Midnight {noun}",
    "{adj} Eyes",
    "My {noun}",
    "Your {noun}",
];

static SONG_VERBS: &[&str] = &[
    "Forget", "Remember", "Believe", "Dream", "Love", "Leave", "Stay", "Run", "Fall", "Rise",
    "Shine", "Glow", "Fly", "Dance", "Sing",
];

/// Music genres.
static GENRES: &[&str] = &[
    "Rock",
    "Pop",
    "Hip Hop",
    "R&B",
    "Country",
    "Jazz",
    "Classical",
    "Electronic",
    "Dance",
    "Reggae",
    "Blues",
    "Folk",
    "Soul",
    "Funk",
    "Metal",
    "Punk",
    "Alternative",
    "Indie",
    "Latin",
    "World",
    "Gospel",
    "Ambient",
    "House",
    "Techno",
    "Trap",
    "Dubstep",
    "Drum and Bass",
    "Synthwave",
    "Lo-Fi",
    "K-Pop",
];

/// Musical instruments.
static INSTRUMENTS: &[&str] = &[
    "Guitar",
    "Piano",
    "Drums",
    "Bass",
    "Violin",
    "Cello",
    "Flute",
    "Saxophone",
    "Trumpet",
    "Trombone",
    "Clarinet",
    "Oboe",
    "Harp",
    "Accordion",
    "Harmonica",
    "Banjo",
    "Mandolin",
    "Ukulele",
    "Synthesizer",
    "Keyboard",
    "Organ",
    "Xylophone",
    "Vibraphone",
    "Timpani",
    "Tambourine",
    "Bongos",
    "Congas",
    "Djembe",
    "Maracas",
    "Triangle",
];

/// Generate a random music artist/band name.
pub fn music_artist<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, ARTIST_PATTERNS);
    pattern
        .replace("{first}", pick(rng, ARTIST_FIRSTS))
        .replace("{last}", pick(rng, ARTIST_LASTS))
        .replace("{name}", pick(rng, ARTIST_FIRSTS))
        .replace("{adj}", pick(rng, BAND_ADJECTIVES))
        .replace("{noun}", pick(rng, BAND_NOUNS))
}

/// Generate a random album name.
pub fn music_album<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, ALBUM_PATTERNS);
    pattern
        .replace("{adj}", pick(rng, BAND_ADJECTIVES))
        .replace("{noun}", pick(rng, BAND_NOUNS))
}

/// Generate a random song name.
pub fn music_song<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, SONG_PATTERNS);
    pattern
        .replace("{adj}", pick(rng, BAND_ADJECTIVES))
        .replace("{noun}", pick(rng, BAND_NOUNS))
        .replace("{verb}", pick(rng, SONG_VERBS))
}

/// Generate a random music genre.
pub fn music_genre<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, GENRES)
}

/// Generate a random musical instrument.
pub fn music_instrument<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, INSTRUMENTS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_music_artist() {
        let mut rng = StdRng::seed_from_u64(42);
        let artist = music_artist(&mut rng);
        assert!(!artist.is_empty());
        assert!(!artist.contains('{'));
    }

    #[test]
    fn test_music_album() {
        let mut rng = StdRng::seed_from_u64(42);
        let album = music_album(&mut rng);
        assert!(!album.is_empty());
        assert!(!album.contains('{'));
    }

    #[test]
    fn test_music_song() {
        let mut rng = StdRng::seed_from_u64(42);
        let song = music_song(&mut rng);
        assert!(!song.is_empty());
        assert!(!song.contains('{'));
    }

    #[test]
    fn test_music_genre() {
        let mut rng = StdRng::seed_from_u64(42);
        let genre = music_genre(&mut rng);
        assert!(GENRES.contains(&genre));
    }

    #[test]
    fn test_music_instrument() {
        let mut rng = StdRng::seed_from_u64(42);
        let instrument = music_instrument(&mut rng);
        assert!(INSTRUMENTS.contains(&instrument));
    }
}
