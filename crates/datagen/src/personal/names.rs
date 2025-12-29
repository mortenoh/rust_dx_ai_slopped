//! Name generation.
//!
//! Generate realistic first names, last names, and full names.
//!
//! # Example
//!
//! ```
//! use dx_datagen::personal::names::{first_name, last_name, full_name};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let first = first_name(&mut rng);
//! let last = last_name(&mut rng);
//! let full = full_name(&mut rng);
//! ```

use rand::Rng;

/// Common male first names (US)
pub const FIRST_NAMES_MALE: &[&str] = &[
    "James",
    "John",
    "Robert",
    "Michael",
    "William",
    "David",
    "Richard",
    "Joseph",
    "Thomas",
    "Charles",
    "Christopher",
    "Daniel",
    "Matthew",
    "Anthony",
    "Mark",
    "Donald",
    "Steven",
    "Paul",
    "Andrew",
    "Joshua",
    "Kenneth",
    "Kevin",
    "Brian",
    "George",
    "Timothy",
    "Ronald",
    "Edward",
    "Jason",
    "Jeffrey",
    "Ryan",
    "Jacob",
    "Gary",
    "Nicholas",
    "Eric",
    "Jonathan",
    "Stephen",
    "Larry",
    "Justin",
    "Scott",
    "Brandon",
    "Benjamin",
    "Samuel",
    "Raymond",
    "Gregory",
    "Frank",
    "Alexander",
    "Patrick",
    "Jack",
    "Dennis",
    "Jerry",
    "Tyler",
    "Aaron",
    "Jose",
    "Adam",
    "Nathan",
    "Henry",
    "Douglas",
    "Zachary",
    "Peter",
    "Kyle",
    "Noah",
    "Ethan",
    "Jeremy",
    "Walter",
    "Christian",
    "Keith",
    "Roger",
    "Terry",
    "Austin",
    "Sean",
    "Gerald",
    "Carl",
    "Dylan",
    "Harold",
    "Jordan",
    "Jesse",
    "Bryan",
    "Lawrence",
    "Arthur",
    "Gabriel",
    "Bruce",
    "Logan",
    "Albert",
    "Willie",
    "Alan",
    "Eugene",
    "Russell",
    "Vincent",
    "Philip",
    "Bobby",
    "Johnny",
    "Bradley",
    "Roy",
    "Ralph",
    "Eugene",
];

/// Common female first names (US)
pub const FIRST_NAMES_FEMALE: &[&str] = &[
    "Mary",
    "Patricia",
    "Jennifer",
    "Linda",
    "Barbara",
    "Elizabeth",
    "Susan",
    "Jessica",
    "Sarah",
    "Karen",
    "Lisa",
    "Nancy",
    "Betty",
    "Margaret",
    "Sandra",
    "Ashley",
    "Kimberly",
    "Emily",
    "Donna",
    "Michelle",
    "Dorothy",
    "Carol",
    "Amanda",
    "Melissa",
    "Deborah",
    "Stephanie",
    "Rebecca",
    "Sharon",
    "Laura",
    "Cynthia",
    "Kathleen",
    "Amy",
    "Angela",
    "Shirley",
    "Anna",
    "Brenda",
    "Pamela",
    "Emma",
    "Nicole",
    "Helen",
    "Samantha",
    "Katherine",
    "Christine",
    "Debra",
    "Rachel",
    "Carolyn",
    "Janet",
    "Catherine",
    "Maria",
    "Heather",
    "Diane",
    "Ruth",
    "Julie",
    "Olivia",
    "Joyce",
    "Virginia",
    "Victoria",
    "Kelly",
    "Lauren",
    "Christina",
    "Joan",
    "Evelyn",
    "Judith",
    "Megan",
    "Andrea",
    "Cheryl",
    "Hannah",
    "Jacqueline",
    "Martha",
    "Gloria",
    "Teresa",
    "Ann",
    "Sara",
    "Madison",
    "Frances",
    "Kathryn",
    "Janice",
    "Jean",
    "Abigail",
    "Alice",
    "Judy",
    "Sophia",
    "Grace",
    "Denise",
    "Amber",
    "Doris",
    "Marilyn",
    "Danielle",
    "Beverly",
    "Isabella",
    "Theresa",
    "Diana",
    "Natalie",
];

/// Common last names (US)
pub const LAST_NAMES: &[&str] = &[
    "Smith",
    "Johnson",
    "Williams",
    "Brown",
    "Jones",
    "Garcia",
    "Miller",
    "Davis",
    "Rodriguez",
    "Martinez",
    "Hernandez",
    "Lopez",
    "Gonzalez",
    "Wilson",
    "Anderson",
    "Thomas",
    "Taylor",
    "Moore",
    "Jackson",
    "Martin",
    "Lee",
    "Perez",
    "Thompson",
    "White",
    "Harris",
    "Sanchez",
    "Clark",
    "Ramirez",
    "Lewis",
    "Robinson",
    "Walker",
    "Young",
    "Allen",
    "King",
    "Wright",
    "Scott",
    "Torres",
    "Nguyen",
    "Hill",
    "Flores",
    "Green",
    "Adams",
    "Nelson",
    "Baker",
    "Hall",
    "Rivera",
    "Campbell",
    "Mitchell",
    "Carter",
    "Roberts",
    "Gomez",
    "Phillips",
    "Evans",
    "Turner",
    "Diaz",
    "Parker",
    "Cruz",
    "Edwards",
    "Collins",
    "Reyes",
    "Stewart",
    "Morris",
    "Morales",
    "Murphy",
    "Cook",
    "Rogers",
    "Gutierrez",
    "Ortiz",
    "Morgan",
    "Cooper",
    "Peterson",
    "Bailey",
    "Reed",
    "Kelly",
    "Howard",
    "Ramos",
    "Kim",
    "Cox",
    "Ward",
    "Richardson",
    "Watson",
    "Brooks",
    "Chavez",
    "Wood",
    "James",
    "Bennett",
    "Gray",
    "Mendoza",
    "Ruiz",
    "Hughes",
    "Price",
    "Alvarez",
    "Castillo",
    "Sanders",
];

/// Name suffixes
pub const NAME_SUFFIXES: &[&str] = &["Jr.", "Sr.", "II", "III", "IV"];

/// Name prefixes/titles
pub const NAME_PREFIXES: &[&str] = &["Mr.", "Mrs.", "Ms.", "Dr.", "Prof."];

/// Pick a random first name (any gender).
pub fn first_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    if rng.random_bool(0.5) {
        first_name_male(rng)
    } else {
        first_name_female(rng)
    }
}

/// Pick a random male first name.
pub fn first_name_male<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    FIRST_NAMES_MALE[rng.random_range(0..FIRST_NAMES_MALE.len())]
}

/// Pick a random female first name.
pub fn first_name_female<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    FIRST_NAMES_FEMALE[rng.random_range(0..FIRST_NAMES_FEMALE.len())]
}

/// Pick a random last name.
pub fn last_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    LAST_NAMES[rng.random_range(0..LAST_NAMES.len())]
}

/// Generate a random full name (first + last).
pub fn full_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} {}", first_name(rng), last_name(rng))
}

/// Generate a random male full name.
pub fn full_name_male<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} {}", first_name_male(rng), last_name(rng))
}

/// Generate a random female full name.
pub fn full_name_female<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} {}", first_name_female(rng), last_name(rng))
}

/// Generate a full name with optional prefix.
pub fn full_name_with_prefix<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = NAME_PREFIXES[rng.random_range(0..NAME_PREFIXES.len())];
    format!("{} {} {}", prefix, first_name(rng), last_name(rng))
}

/// Generate a full name with optional suffix (10% chance).
pub fn full_name_with_suffix<R: ?Sized + Rng>(rng: &mut R) -> String {
    let name = full_name(rng);
    if rng.random_bool(0.1) {
        let suffix = NAME_SUFFIXES[rng.random_range(0..NAME_SUFFIXES.len())];
        format!("{} {}", name, suffix)
    } else {
        name
    }
}

/// Generate initials from a name.
pub fn initials<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{}{}",
        &first_name(rng)[..1].to_uppercase(),
        &last_name(rng)[..1].to_uppercase()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_first_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = first_name(&mut rng);
        assert!(
            FIRST_NAMES_MALE.contains(&name) || FIRST_NAMES_FEMALE.contains(&name),
            "Name not found: {}",
            name
        );
    }

    #[test]
    fn test_first_name_male() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = first_name_male(&mut rng);
        assert!(FIRST_NAMES_MALE.contains(&name));
    }

    #[test]
    fn test_first_name_female() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = first_name_female(&mut rng);
        assert!(FIRST_NAMES_FEMALE.contains(&name));
    }

    #[test]
    fn test_last_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = last_name(&mut rng);
        assert!(LAST_NAMES.contains(&name));
    }

    #[test]
    fn test_full_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = full_name(&mut rng);
        let parts: Vec<&str> = name.split(' ').collect();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_initials() {
        let mut rng = StdRng::seed_from_u64(42);
        let init = initials(&mut rng);
        assert_eq!(init.len(), 2);
        assert!(init.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(full_name(&mut rng1), full_name(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let name = first_name(&mut *rng);
        assert!(!name.is_empty());
    }
}
