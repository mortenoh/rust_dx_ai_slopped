//! Book-related data generators.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Book title patterns for generating realistic book names.
static TITLE_PATTERNS: &[&str] = &[
    "The {adj} {noun}",
    "A {noun} of {noun}",
    "The {noun}'s {noun}",
    "{adj} {noun}",
    "The Last {noun}",
    "Beyond the {noun}",
    "Secrets of the {noun}",
    "The {noun} Chronicles",
    "Return of the {noun}",
    "The {adj} {noun} of {place}",
    "When {noun} Falls",
    "The {noun} Conspiracy",
    "Midnight {noun}",
    "The {noun} Code",
    "Shadow of the {noun}",
];

static TITLE_ADJECTIVES: &[&str] = &[
    "Lost",
    "Hidden",
    "Dark",
    "Silent",
    "Forgotten",
    "Ancient",
    "Eternal",
    "Golden",
    "Silver",
    "Broken",
    "Crimson",
    "Shattered",
    "Burning",
    "Frozen",
    "Sacred",
    "Cursed",
    "Twisted",
    "Hollow",
    "Fallen",
    "Rising",
    "Wicked",
    "Noble",
    "Final",
    "First",
    "Secret",
];

static TITLE_NOUNS: &[&str] = &[
    "Kingdom", "Shadow", "Crown", "Throne", "Storm", "Dream", "Night", "Dawn", "Flame", "Heart",
    "Soul", "Moon", "Sun", "Star", "Wind", "Stone", "Blood", "Fire", "Ice", "Dragon", "Sword",
    "Tower", "Garden", "Ocean", "Mountain", "River", "Forest", "Castle", "City", "World",
];

static TITLE_PLACES: &[&str] = &[
    "Avalon",
    "Elysium",
    "Atlantis",
    "Olympus",
    "Arcadia",
    "Camelot",
    "Babylon",
    "Eden",
    "Valhalla",
    "Shangri-La",
];

/// Author first names.
static AUTHOR_FIRST_NAMES: &[&str] = &[
    "James",
    "Sarah",
    "Michael",
    "Emily",
    "David",
    "Jennifer",
    "Robert",
    "Elizabeth",
    "William",
    "Margaret",
    "John",
    "Catherine",
    "Thomas",
    "Victoria",
    "Charles",
    "Alexandra",
    "Richard",
    "Charlotte",
    "Daniel",
    "Rebecca",
    "Christopher",
    "Olivia",
    "Matthew",
    "Sophia",
    "Andrew",
    "Isabella",
    "Stephen",
    "Grace",
    "Jonathan",
    "Hannah",
];

/// Author last names.
static AUTHOR_LAST_NAMES: &[&str] = &[
    "Anderson",
    "Mitchell",
    "Harper",
    "Collins",
    "Stewart",
    "Morgan",
    "Brooks",
    "Sullivan",
    "Reynolds",
    "Foster",
    "Graham",
    "Bennett",
    "Crawford",
    "Spencer",
    "Harrison",
    "Wallace",
    "Mason",
    "Palmer",
    "Douglas",
    "Fletcher",
    "Thornton",
    "Ashworth",
    "Blackwood",
    "Sinclair",
    "Whitmore",
    "Kingsley",
    "Hartwell",
    "Carmichael",
    "Montgomery",
    "Pemberton",
];

/// Book genres.
static GENRES: &[&str] = &[
    "Fiction",
    "Non-Fiction",
    "Mystery",
    "Thriller",
    "Romance",
    "Science Fiction",
    "Fantasy",
    "Horror",
    "Historical Fiction",
    "Literary Fiction",
    "Young Adult",
    "Children's",
    "Biography",
    "Autobiography",
    "Memoir",
    "Self-Help",
    "Business",
    "True Crime",
    "Poetry",
    "Drama",
    "Adventure",
    "Dystopian",
    "Contemporary",
    "Paranormal",
    "Urban Fantasy",
    "Epic Fantasy",
    "Space Opera",
    "Cozy Mystery",
    "Psychological Thriller",
    "Romantic Comedy",
];

/// Book publishers.
static PUBLISHERS: &[&str] = &[
    "Penguin Random House",
    "HarperCollins",
    "Simon & Schuster",
    "Macmillan",
    "Hachette",
    "Scholastic",
    "Wiley",
    "McGraw-Hill",
    "Pearson",
    "Oxford University Press",
    "Cambridge University Press",
    "Bloomsbury",
    "Tor Books",
    "Del Rey",
    "Bantam Books",
    "Ace Books",
    "Berkley",
    "Avon",
    "Crown Publishing",
    "Knopf",
    "Vintage Books",
    "Anchor Books",
    "Riverhead Books",
    "Putnam",
    "Dutton",
];

/// Book series name patterns.
static SERIES_PATTERNS: &[&str] = &[
    "The {noun} Saga",
    "The {noun} Chronicles",
    "The {noun} Trilogy",
    "The {adj} {noun} Series",
    "{noun} Wars",
    "The {noun} Legacy",
    "Tales of {noun}",
    "The {noun} Cycle",
    "The {noun} Quartet",
    "Children of the {noun}",
];

/// Generate a random book title.
///
/// # Example
/// ```
/// use dx_datagen::entertainment::books::book_title;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let title = book_title(&mut rng);
/// assert!(!title.is_empty());
/// ```
pub fn book_title<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, TITLE_PATTERNS);
    pattern
        .replace("{adj}", pick(rng, TITLE_ADJECTIVES))
        .replace("{noun}", pick(rng, TITLE_NOUNS))
        .replace("{place}", pick(rng, TITLE_PLACES))
}

/// Generate a random author name.
///
/// # Example
/// ```
/// use dx_datagen::entertainment::books::book_author;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let author = book_author(&mut rng);
/// assert!(author.contains(' '));
/// ```
pub fn book_author<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{} {}",
        pick(rng, AUTHOR_FIRST_NAMES),
        pick(rng, AUTHOR_LAST_NAMES)
    )
}

/// Generate a random book genre.
///
/// # Example
/// ```
/// use dx_datagen::entertainment::books::book_genre;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let genre = book_genre(&mut rng);
/// assert!(!genre.is_empty());
/// ```
pub fn book_genre<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, GENRES)
}

/// Generate a random publisher name.
///
/// # Example
/// ```
/// use dx_datagen::entertainment::books::book_publisher;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let publisher = book_publisher(&mut rng);
/// assert!(!publisher.is_empty());
/// ```
pub fn book_publisher<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, PUBLISHERS)
}

/// Generate a random book series name.
///
/// # Example
/// ```
/// use dx_datagen::entertainment::books::book_series;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let series = book_series(&mut rng);
/// assert!(!series.is_empty());
/// ```
pub fn book_series<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, SERIES_PATTERNS);
    pattern
        .replace("{adj}", pick(rng, TITLE_ADJECTIVES))
        .replace("{noun}", pick(rng, TITLE_NOUNS))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_book_title() {
        let mut rng = StdRng::seed_from_u64(42);
        let title = book_title(&mut rng);
        assert!(!title.is_empty());
        // Should not contain template placeholders
        assert!(!title.contains('{'));
    }

    #[test]
    fn test_book_author() {
        let mut rng = StdRng::seed_from_u64(42);
        let author = book_author(&mut rng);
        assert!(author.contains(' '));
    }

    #[test]
    fn test_book_genre() {
        let mut rng = StdRng::seed_from_u64(42);
        let genre = book_genre(&mut rng);
        assert!(GENRES.contains(&genre));
    }

    #[test]
    fn test_book_publisher() {
        let mut rng = StdRng::seed_from_u64(42);
        let publisher = book_publisher(&mut rng);
        assert!(PUBLISHERS.contains(&publisher));
    }

    #[test]
    fn test_book_series() {
        let mut rng = StdRng::seed_from_u64(42);
        let series = book_series(&mut rng);
        assert!(!series.is_empty());
        assert!(!series.contains('{'));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);
        assert_eq!(book_title(&mut rng1), book_title(&mut rng2));
    }
}
