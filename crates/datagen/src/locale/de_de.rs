//! German (Germany) locale data.
//!
//! Provides German-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// German locale marker type.
pub struct DeDe;

/// Common male first names in Germany.
pub const MALE_FIRST_NAMES: &[&str] = &[
    "Alexander",
    "Andreas",
    "Benjamin",
    "Christian",
    "Daniel",
    "David",
    "Dennis",
    "Dominik",
    "Erik",
    "Fabian",
    "Felix",
    "Florian",
    "Frank",
    "Hans",
    "Jan",
    "Jens",
    "Johannes",
    "Jonas",
    "Julian",
    "Kevin",
    "Klaus",
    "Leon",
    "Lukas",
    "Marcel",
    "Marco",
    "Markus",
    "Martin",
    "Matthias",
    "Max",
    "Maximilian",
    "Michael",
    "Niklas",
    "Patrick",
    "Paul",
    "Peter",
    "Philipp",
    "Sebastian",
    "Simon",
    "Stefan",
    "Thomas",
    "Tim",
    "Tobias",
    "Wolfgang",
    "Nico",
    "Oliver",
    "Ralf",
    "Robert",
    "Sven",
    "Uwe",
    "Werner",
];

/// Common female first names in Germany.
pub const FEMALE_FIRST_NAMES: &[&str] = &[
    "Anna",
    "Andrea",
    "Angela",
    "Angelika",
    "Anja",
    "Birgit",
    "Brigitte",
    "Carolin",
    "Christina",
    "Claudia",
    "Daniela",
    "Elena",
    "Emma",
    "Franziska",
    "Gabriele",
    "Hannah",
    "Heike",
    "Jana",
    "Jennifer",
    "Jessica",
    "Julia",
    "Katharina",
    "Katrin",
    "Kerstin",
    "Laura",
    "Lea",
    "Lena",
    "Lisa",
    "Manuela",
    "Maria",
    "Marie",
    "Martina",
    "Melanie",
    "Mia",
    "Monika",
    "Nadine",
    "Nicole",
    "Petra",
    "Sabine",
    "Sandra",
    "Sarah",
    "Silke",
    "Simone",
    "Sophia",
    "Stefanie",
    "Susanne",
    "Tanja",
    "Ursula",
    "Vanessa",
    "Yvonne",
];

/// Common last names in Germany.
pub const LAST_NAMES: &[&str] = &[
    "Müller",
    "Schmidt",
    "Schneider",
    "Fischer",
    "Weber",
    "Meyer",
    "Wagner",
    "Becker",
    "Schulz",
    "Hoffmann",
    "Schäfer",
    "Koch",
    "Bauer",
    "Richter",
    "Klein",
    "Wolf",
    "Schröder",
    "Neumann",
    "Schwarz",
    "Zimmermann",
    "Braun",
    "Krüger",
    "Hofmann",
    "Hartmann",
    "Lange",
    "Schmitt",
    "Werner",
    "Schmitz",
    "Krause",
    "Meier",
    "Lehmann",
    "Schmid",
    "Schulze",
    "Maier",
    "Köhler",
    "Herrmann",
    "König",
    "Walter",
    "Mayer",
    "Huber",
    "Kaiser",
    "Fuchs",
    "Peters",
    "Lang",
    "Scholz",
    "Möller",
    "Weiß",
    "Jung",
    "Hahn",
    "Schubert",
];

/// German cities.
pub const CITIES: &[&str] = &[
    "Berlin",
    "Hamburg",
    "München",
    "Köln",
    "Frankfurt am Main",
    "Stuttgart",
    "Düsseldorf",
    "Leipzig",
    "Dortmund",
    "Essen",
    "Bremen",
    "Dresden",
    "Hannover",
    "Nürnberg",
    "Duisburg",
    "Bochum",
    "Wuppertal",
    "Bielefeld",
    "Bonn",
    "Münster",
    "Mannheim",
    "Karlsruhe",
    "Augsburg",
    "Wiesbaden",
    "Mönchengladbach",
    "Gelsenkirchen",
    "Aachen",
    "Braunschweig",
    "Chemnitz",
    "Kiel",
    "Halle",
    "Magdeburg",
    "Freiburg",
    "Krefeld",
    "Mainz",
    "Lübeck",
    "Erfurt",
    "Rostock",
    "Kassel",
    "Oberhausen",
];

/// German federal states (Bundesländer).
pub const STATES: &[(&str, &str)] = &[
    ("Baden-Württemberg", "BW"),
    ("Bayern", "BY"),
    ("Berlin", "BE"),
    ("Brandenburg", "BB"),
    ("Bremen", "HB"),
    ("Hamburg", "HH"),
    ("Hessen", "HE"),
    ("Mecklenburg-Vorpommern", "MV"),
    ("Niedersachsen", "NI"),
    ("Nordrhein-Westfalen", "NW"),
    ("Rheinland-Pfalz", "RP"),
    ("Saarland", "SL"),
    ("Sachsen", "SN"),
    ("Sachsen-Anhalt", "ST"),
    ("Schleswig-Holstein", "SH"),
    ("Thüringen", "TH"),
];

/// Street suffixes.
pub const STREET_SUFFIXES: &[&str] = &[
    "straße", "str.", "weg", "allee", "platz", "ring", "gasse", "damm", "ufer", "chaussee",
    "steig", "pfad", "park", "hof",
];

/// Street names (common words used in German street names).
pub const STREET_NAMES: &[&str] = &[
    "Haupt",
    "Bahn",
    "Berg",
    "Kirch",
    "Schul",
    "Markt",
    "Post",
    "Rathaus",
    "Park",
    "Garten",
    "Wald",
    "Wiesen",
    "Linden",
    "Eichen",
    "Buchen",
    "Tannen",
    "Birken",
    "Ahorn",
    "Rosen",
    "Blumen",
    "Sonnen",
    "Mond",
    "Stern",
    "Nord",
    "Süd",
    "Ost",
    "West",
    "Friedrich",
    "Wilhelm",
    "Kaiser",
];

/// Get a random first name (male or female).
pub fn first_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    if rng.random_bool(0.5) {
        first_name_male(rng)
    } else {
        first_name_female(rng)
    }
}

/// Get a random male first name.
pub fn first_name_male<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    MALE_FIRST_NAMES[rng.random_range(0..MALE_FIRST_NAMES.len())]
}

/// Get a random female first name.
pub fn first_name_female<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    FEMALE_FIRST_NAMES[rng.random_range(0..FEMALE_FIRST_NAMES.len())]
}

/// Get a random last name.
pub fn last_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    LAST_NAMES[rng.random_range(0..LAST_NAMES.len())]
}

/// Generate a full name.
pub fn full_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} {}", first_name(rng), last_name(rng))
}

/// Generate a German phone number in XXXXX XXXXXXX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    // German mobile prefixes: 015x, 016x, 017x
    let prefixes = ["015", "016", "017"];
    let prefix = prefixes[rng.random_range(0..prefixes.len())];
    let d1: u8 = rng.random_range(0..10);
    let rest: u32 = rng.random_range(0..10_000_000);
    format!("{}{} {:07}", prefix, d1, rest)
}

/// Generate a German phone number in +49XXXXXXXXXXX format.
pub fn phone_e164<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefixes = ["15", "16", "17"];
    let prefix = prefixes[rng.random_range(0..prefixes.len())];
    let d1: u8 = rng.random_range(0..10);
    let rest: u32 = rng.random_range(0..10_000_000);
    format!("+49{}{}{:07}", prefix, d1, rest)
}

/// Get a random city.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    CITIES[rng.random_range(0..CITIES.len())]
}

/// Get a random state (Bundesland).
pub fn state<R: ?Sized + Rng>(rng: &mut R) -> (&'static str, &'static str) {
    STATES[rng.random_range(0..STATES.len())]
}

/// Get a random street suffix.
pub fn street_suffix<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    STREET_SUFFIXES[rng.random_range(0..STREET_SUFFIXES.len())]
}

/// Generate a street address.
pub fn street_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let street = STREET_NAMES[rng.random_range(0..STREET_NAMES.len())];
    let suffix = street_suffix(rng);
    let number = rng.random_range(1..200);
    format!("{}{} {}", street, suffix, number)
}

/// Generate a postal code (Postleitzahl).
pub fn postal_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{:05}", rng.random_range(1000..99999))
}

/// Generate a full address.
pub fn full_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let street = street_address(rng);
    let postal = postal_code(rng);
    let city_name = city(rng);
    format!("{}, {} {}", street, postal, city_name)
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
        assert!(!name.is_empty());
    }

    #[test]
    fn test_first_name_male() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = first_name_male(&mut rng);
        assert!(MALE_FIRST_NAMES.contains(&name));
    }

    #[test]
    fn test_first_name_female() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = first_name_female(&mut rng);
        assert!(FEMALE_FIRST_NAMES.contains(&name));
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
        assert!(name.contains(' '));
    }

    #[test]
    fn test_phone() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone_num = phone(&mut rng);
        assert!(phone_num.starts_with("01"));
        assert!(phone_num.contains(' '));
    }

    #[test]
    fn test_phone_e164() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone_num = phone_e164(&mut rng);
        assert!(phone_num.starts_with("+49"));
    }

    #[test]
    fn test_city() {
        let mut rng = StdRng::seed_from_u64(42);
        let city_name = city(&mut rng);
        assert!(CITIES.contains(&city_name));
    }

    #[test]
    fn test_state() {
        let mut rng = StdRng::seed_from_u64(42);
        let (name, abbr) = state(&mut rng);
        assert!(!name.is_empty());
        assert_eq!(abbr.len(), 2);
    }

    #[test]
    fn test_street_address() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = street_address(&mut rng);
        assert!(!addr.is_empty());
    }

    #[test]
    fn test_postal_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let code = postal_code(&mut rng);
        assert_eq!(code.len(), 5);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_full_address() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = full_address(&mut rng);
        assert!(addr.contains(','));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        assert_eq!(first_name(&mut rng1), first_name(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let name = first_name(&mut *rng);
        assert!(!name.is_empty());
    }
}
