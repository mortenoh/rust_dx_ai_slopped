//! US English locale data.
//!
//! Provides US-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// US English locale marker type.
pub struct EnUs;

/// Common male first names in the US.
pub const MALE_FIRST_NAMES: &[&str] = &[
    "James",
    "Robert",
    "John",
    "Michael",
    "David",
    "William",
    "Richard",
    "Joseph",
    "Thomas",
    "Christopher",
    "Charles",
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
    "Raymond",
    "Jack",
    "Dennis",
];

/// Common female first names in the US.
pub const FEMALE_FIRST_NAMES: &[&str] = &[
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
];

/// Common last names in the US.
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
];

/// US cities.
pub const CITIES: &[&str] = &[
    "New York",
    "Los Angeles",
    "Chicago",
    "Houston",
    "Phoenix",
    "Philadelphia",
    "San Antonio",
    "San Diego",
    "Dallas",
    "San Jose",
    "Austin",
    "Jacksonville",
    "Fort Worth",
    "Columbus",
    "Charlotte",
    "San Francisco",
    "Indianapolis",
    "Seattle",
    "Denver",
    "Washington",
    "Boston",
    "El Paso",
    "Nashville",
    "Detroit",
    "Oklahoma City",
    "Portland",
    "Las Vegas",
    "Memphis",
    "Louisville",
    "Baltimore",
    "Milwaukee",
    "Albuquerque",
    "Tucson",
    "Fresno",
    "Sacramento",
    "Kansas City",
    "Mesa",
    "Atlanta",
    "Omaha",
    "Colorado Springs",
];

/// US states with abbreviations.
pub const STATES: &[(&str, &str)] = &[
    ("Alabama", "AL"),
    ("Alaska", "AK"),
    ("Arizona", "AZ"),
    ("Arkansas", "AR"),
    ("California", "CA"),
    ("Colorado", "CO"),
    ("Connecticut", "CT"),
    ("Delaware", "DE"),
    ("Florida", "FL"),
    ("Georgia", "GA"),
    ("Hawaii", "HI"),
    ("Idaho", "ID"),
    ("Illinois", "IL"),
    ("Indiana", "IN"),
    ("Iowa", "IA"),
    ("Kansas", "KS"),
    ("Kentucky", "KY"),
    ("Louisiana", "LA"),
    ("Maine", "ME"),
    ("Maryland", "MD"),
    ("Massachusetts", "MA"),
    ("Michigan", "MI"),
    ("Minnesota", "MN"),
    ("Mississippi", "MS"),
    ("Missouri", "MO"),
    ("Montana", "MT"),
    ("Nebraska", "NE"),
    ("Nevada", "NV"),
    ("New Hampshire", "NH"),
    ("New Jersey", "NJ"),
    ("New Mexico", "NM"),
    ("New York", "NY"),
    ("North Carolina", "NC"),
    ("North Dakota", "ND"),
    ("Ohio", "OH"),
    ("Oklahoma", "OK"),
    ("Oregon", "OR"),
    ("Pennsylvania", "PA"),
    ("Rhode Island", "RI"),
    ("South Carolina", "SC"),
    ("South Dakota", "SD"),
    ("Tennessee", "TN"),
    ("Texas", "TX"),
    ("Utah", "UT"),
    ("Vermont", "VT"),
    ("Virginia", "VA"),
    ("Washington", "WA"),
    ("West Virginia", "WV"),
    ("Wisconsin", "WI"),
    ("Wyoming", "WY"),
];

/// Street suffixes.
pub const STREET_SUFFIXES: &[&str] = &[
    "Street",
    "St",
    "Avenue",
    "Ave",
    "Boulevard",
    "Blvd",
    "Drive",
    "Dr",
    "Lane",
    "Ln",
    "Road",
    "Rd",
    "Court",
    "Ct",
    "Circle",
    "Cir",
    "Place",
    "Pl",
    "Way",
    "Terrace",
    "Ter",
    "Trail",
    "Trl",
];

/// Street names (common words used in street names).
pub const STREET_NAMES: &[&str] = &[
    "Main",
    "Oak",
    "Maple",
    "Cedar",
    "Pine",
    "Elm",
    "Washington",
    "Park",
    "Lake",
    "Hill",
    "First",
    "Second",
    "Third",
    "Fourth",
    "Fifth",
    "Sunset",
    "Spring",
    "River",
    "Church",
    "Center",
    "Market",
    "Union",
    "Liberty",
    "Franklin",
    "Jefferson",
    "Adams",
    "Madison",
    "Jackson",
    "Lincoln",
    "Grant",
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

/// Generate a US phone number in (XXX) XXX-XXXX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let area = rng.random_range(200..1000);
    let exchange = rng.random_range(200..1000);
    let subscriber = rng.random_range(0..10000);
    format!("({:03}) {:03}-{:04}", area, exchange, subscriber)
}

/// Generate a US phone number in +1XXXXXXXXXX format.
pub fn phone_e164<R: ?Sized + Rng>(rng: &mut R) -> String {
    let area = rng.random_range(200..1000);
    let exchange = rng.random_range(200..1000);
    let subscriber = rng.random_range(0..10000);
    format!("+1{:03}{:03}{:04}", area, exchange, subscriber)
}

/// Get a random city.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    CITIES[rng.random_range(0..CITIES.len())]
}

/// Get a random state.
pub fn state<R: ?Sized + Rng>(rng: &mut R) -> (&'static str, &'static str) {
    STATES[rng.random_range(0..STATES.len())]
}

/// Get a random street suffix.
pub fn street_suffix<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    STREET_SUFFIXES[rng.random_range(0..STREET_SUFFIXES.len())]
}

/// Generate a street address.
pub fn street_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let number = rng.random_range(1..10000);
    let street = STREET_NAMES[rng.random_range(0..STREET_NAMES.len())];
    let suffix = street_suffix(rng);
    format!("{} {} {}", number, street, suffix)
}

/// Generate a zip code.
pub fn postal_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{:05}", rng.random_range(10000..99999))
}

/// Generate a zip code with +4 extension.
pub fn postal_code_plus4<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{:05}-{:04}",
        rng.random_range(10000..99999),
        rng.random_range(0..9999)
    )
}

/// Generate a full address.
pub fn full_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let street = street_address(rng);
    let city_name = city(rng);
    let (_, state_abbr) = state(rng);
    let zip = postal_code(rng);
    format!("{}, {}, {} {}", street, city_name, state_abbr, zip)
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
        assert!(phone_num.starts_with('('));
        assert!(phone_num.contains(')'));
        assert!(phone_num.contains('-'));
    }

    #[test]
    fn test_phone_e164() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone_num = phone_e164(&mut rng);
        assert!(phone_num.starts_with("+1"));
        assert_eq!(phone_num.len(), 12);
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
        let zip = postal_code(&mut rng);
        assert_eq!(zip.len(), 5);
        assert!(zip.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_postal_code_plus4() {
        let mut rng = StdRng::seed_from_u64(42);
        let zip = postal_code_plus4(&mut rng);
        assert!(zip.contains('-'));
        assert_eq!(zip.len(), 10);
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
