//! Movie-related data generators.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Movie title patterns.
static TITLE_PATTERNS: &[&str] = &[
    "The {adj} {noun}",
    "{noun} of {noun}",
    "The Last {noun}",
    "{adj} {noun}",
    "Beyond {noun}",
    "The {noun} Files",
    "{noun} Rising",
    "Operation {noun}",
    "The {noun} Effect",
    "Project {noun}",
    "{noun} Protocol",
    "The {adj} Mile",
    "{noun} Junction",
    "The {noun} Incident",
    "Code {noun}",
];

static TITLE_ADJECTIVES: &[&str] = &[
    "Dark", "Final", "Last", "First", "Deadly", "Silent", "Hidden", "Lost", "Broken", "Crimson",
    "Black", "White", "Red", "Blue", "Golden", "Silver", "Iron", "Steel", "Glass", "Velvet",
];

static TITLE_NOUNS: &[&str] = &[
    "Night",
    "Day",
    "Dawn",
    "Dusk",
    "Storm",
    "Thunder",
    "Lightning",
    "Rain",
    "Shadow",
    "Light",
    "Fire",
    "Ice",
    "Wolf",
    "Eagle",
    "Phoenix",
    "Dragon",
    "Knight",
    "King",
    "Queen",
    "Warrior",
    "Hunter",
    "Assassin",
    "Agent",
    "Soldier",
    "Hero",
];

/// Director first names.
static DIRECTOR_FIRST_NAMES: &[&str] = &[
    "Christopher",
    "Steven",
    "Martin",
    "James",
    "David",
    "Denis",
    "Ridley",
    "Peter",
    "Quentin",
    "Wes",
    "Guillermo",
    "Alfonso",
    "Alejandro",
    "Michael",
    "Robert",
    "Francis",
    "Stanley",
    "Clint",
    "Oliver",
    "Spike",
];

/// Director last names.
static DIRECTOR_LAST_NAMES: &[&str] = &[
    "Nolan",
    "Spielberg",
    "Scorsese",
    "Cameron",
    "Fincher",
    "Villeneuve",
    "Scott",
    "Jackson",
    "Tarantino",
    "Anderson",
    "del Toro",
    "Cuaron",
    "Inarritu",
    "Bay",
    "Zemeckis",
    "Coppola",
    "Kubrick",
    "Eastwood",
    "Stone",
    "Lee",
];

/// Actor first names.
static ACTOR_FIRST_NAMES: &[&str] = &[
    "Tom",
    "Leonardo",
    "Brad",
    "Robert",
    "Denzel",
    "Morgan",
    "Samuel",
    "Matt",
    "Christian",
    "Ryan",
    "Jake",
    "Joaquin",
    "Cate",
    "Meryl",
    "Nicole",
    "Scarlett",
    "Emma",
    "Natalie",
    "Jennifer",
    "Sandra",
    "Viola",
    "Margot",
    "Charlize",
    "Anne",
    "Amy",
];

/// Actor last names.
static ACTOR_LAST_NAMES: &[&str] = &[
    "Hanks",
    "DiCaprio",
    "Pitt",
    "Downey Jr.",
    "Washington",
    "Freeman",
    "Jackson",
    "Damon",
    "Bale",
    "Gosling",
    "Gyllenhaal",
    "Phoenix",
    "Blanchett",
    "Streep",
    "Kidman",
    "Johansson",
    "Stone",
    "Portman",
    "Lawrence",
    "Bullock",
    "Davis",
    "Robbie",
    "Theron",
    "Hathaway",
    "Adams",
];

/// Movie genres.
static GENRES: &[&str] = &[
    "Action",
    "Adventure",
    "Animation",
    "Comedy",
    "Crime",
    "Documentary",
    "Drama",
    "Fantasy",
    "Film Noir",
    "Horror",
    "Musical",
    "Mystery",
    "Romance",
    "Science Fiction",
    "Thriller",
    "War",
    "Western",
    "Biographical",
    "Historical",
    "Sports",
    "Superhero",
    "Disaster",
    "Psychological",
    "Heist",
    "Spy",
];

/// Movie ratings.
static RATINGS: &[&str] = &["G", "PG", "PG-13", "R", "NC-17", "NR", "TV-14", "TV-MA"];

/// Generate a random movie title.
pub fn movie_title<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, TITLE_PATTERNS);
    pattern
        .replace("{adj}", pick(rng, TITLE_ADJECTIVES))
        .replace("{noun}", pick(rng, TITLE_NOUNS))
}

/// Generate a random director name.
pub fn movie_director<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{} {}",
        pick(rng, DIRECTOR_FIRST_NAMES),
        pick(rng, DIRECTOR_LAST_NAMES)
    )
}

/// Generate a random actor name.
pub fn movie_actor<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{} {}",
        pick(rng, ACTOR_FIRST_NAMES),
        pick(rng, ACTOR_LAST_NAMES)
    )
}

/// Generate a random movie genre.
pub fn movie_genre<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, GENRES)
}

/// Generate a random movie rating (MPAA).
pub fn movie_rating<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, RATINGS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_movie_title() {
        let mut rng = StdRng::seed_from_u64(42);
        let title = movie_title(&mut rng);
        assert!(!title.is_empty());
        assert!(!title.contains('{'));
    }

    #[test]
    fn test_movie_director() {
        let mut rng = StdRng::seed_from_u64(42);
        let director = movie_director(&mut rng);
        assert!(director.contains(' '));
    }

    #[test]
    fn test_movie_actor() {
        let mut rng = StdRng::seed_from_u64(42);
        let actor = movie_actor(&mut rng);
        assert!(actor.contains(' '));
    }

    #[test]
    fn test_movie_genre() {
        let mut rng = StdRng::seed_from_u64(42);
        let genre = movie_genre(&mut rng);
        assert!(GENRES.contains(&genre));
    }

    #[test]
    fn test_movie_rating() {
        let mut rng = StdRng::seed_from_u64(42);
        let rating = movie_rating(&mut rng);
        assert!(RATINGS.contains(&rating));
    }
}
