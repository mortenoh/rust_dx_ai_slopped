//! TV show-related data generators.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// TV show name patterns.
static SHOW_PATTERNS: &[&str] = &[
    "The {adj} {noun}",
    "{noun} Squad",
    "{noun} & {noun}",
    "The {noun}s",
    "{adj} {noun}s",
    "{noun} Files",
    "Project {noun}",
    "{noun} City",
    "The {noun} Show",
    "{adj} Nights",
    "{noun} Island",
    "House of {noun}",
    "The {noun} Legacy",
    "Breaking {noun}",
    "{noun} Street",
];

static SHOW_ADJECTIVES: &[&str] = &[
    "Good", "Bad", "Big", "Little", "New", "Old", "Modern", "Ancient", "Wild", "Crazy", "Happy",
    "Sad", "Dark", "Bright", "Secret", "Hidden", "Lost", "Found", "True", "Real",
];

static SHOW_NOUNS: &[&str] = &[
    "Family", "Love", "Life", "Dream", "World", "Place", "Time", "Story", "Mind", "Heart", "House",
    "Home", "Office", "Club", "Team", "Force", "Agency", "Unit", "Division", "Zone",
];

/// TV networks.
static NETWORKS: &[&str] = &[
    "NBC",
    "ABC",
    "CBS",
    "Fox",
    "The CW",
    "HBO",
    "Showtime",
    "AMC",
    "FX",
    "TNT",
    "USA Network",
    "TBS",
    "Comedy Central",
    "MTV",
    "Syfy",
    "Lifetime",
    "A&E",
    "History Channel",
    "Discovery",
    "National Geographic",
    "Netflix",
    "Amazon Prime",
    "Hulu",
    "Disney+",
    "Apple TV+",
    "Peacock",
    "Paramount+",
    "Max",
];

/// TV channels (for cable/broadcast).
static CHANNELS: &[&str] = &[
    "Channel 4",
    "Channel 5",
    "Channel 7",
    "Channel 9",
    "Channel 11",
    "Channel 13",
    "BBC One",
    "BBC Two",
    "ITV",
    "Channel 4 UK",
    "Sky One",
    "E4",
    "Dave",
    "Comedy Central",
    "Nickelodeon",
    "Cartoon Network",
    "Disney Channel",
    "ESPN",
    "CNN",
    "Fox News",
];

/// TV show genres.
static GENRES: &[&str] = &[
    "Drama",
    "Comedy",
    "Sitcom",
    "Crime",
    "Mystery",
    "Thriller",
    "Horror",
    "Science Fiction",
    "Fantasy",
    "Action",
    "Adventure",
    "Romance",
    "Documentary",
    "Reality",
    "Game Show",
    "Talk Show",
    "News",
    "Sports",
    "Animated",
    "Medical",
    "Legal",
    "Police Procedural",
    "Soap Opera",
    "Miniseries",
    "Anthology",
];

/// Generate a random TV show name.
pub fn tv_show<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, SHOW_PATTERNS);
    pattern
        .replace("{adj}", pick(rng, SHOW_ADJECTIVES))
        .replace("{noun}", pick(rng, SHOW_NOUNS))
}

/// Generate a random TV network name.
pub fn tv_network<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, NETWORKS)
}

/// Generate a random TV channel.
pub fn tv_channel<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, CHANNELS)
}

/// Generate a random TV genre.
pub fn tv_genre<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, GENRES)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_tv_show() {
        let mut rng = StdRng::seed_from_u64(42);
        let show = tv_show(&mut rng);
        assert!(!show.is_empty());
        assert!(!show.contains('{'));
    }

    #[test]
    fn test_tv_network() {
        let mut rng = StdRng::seed_from_u64(42);
        let network = tv_network(&mut rng);
        assert!(NETWORKS.contains(&network));
    }

    #[test]
    fn test_tv_channel() {
        let mut rng = StdRng::seed_from_u64(42);
        let channel = tv_channel(&mut rng);
        assert!(CHANNELS.contains(&channel));
    }

    #[test]
    fn test_tv_genre() {
        let mut rng = StdRng::seed_from_u64(42);
        let genre = tv_genre(&mut rng);
        assert!(GENRES.contains(&genre));
    }
}
