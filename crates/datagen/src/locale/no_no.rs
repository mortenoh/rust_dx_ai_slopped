//! Norwegian locale data.
//!
//! Provides Norwegian-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// Norwegian locale marker type.
pub struct NoNo;

/// Common male first names in Norway.
pub const MALE_FIRST_NAMES: &[&str] = &[
    "Jan",
    "Per",
    "Bjørn",
    "Ole",
    "Lars",
    "Kjell",
    "Knut",
    "Arne",
    "Svein",
    "Hans",
    "Tor",
    "Thomas",
    "Erik",
    "Geir",
    "Jon",
    "Terje",
    "Odd",
    "Morten",
    "Trond",
    "Harald",
    "Olav",
    "Rune",
    "Helge",
    "Magnus",
    "Anders",
    "Øyvind",
    "Frode",
    "Eirik",
    "Steinar",
    "Leif",
    "Gunnar",
    "Vidar",
    "Håkon",
    "Stig",
    "Petter",
    "Kjetil",
    "Espen",
    "Arild",
    "Roy",
    "Dag",
    "Christian",
    "Martin",
    "Andreas",
    "Kristian",
    "Henrik",
    "Daniel",
    "Alexander",
    "Jonas",
    "Sindre",
    "Sondre",
];

/// Common female first names in Norway.
pub const FEMALE_FIRST_NAMES: &[&str] = &[
    "Anne",
    "Inger",
    "Kari",
    "Marit",
    "Ingrid",
    "Liv",
    "Eva",
    "Berit",
    "Astrid",
    "Bjørg",
    "Hilde",
    "Anna",
    "Solveig",
    "Marianne",
    "Randi",
    "Ida",
    "Nina",
    "Elisabeth",
    "Kristin",
    "Bente",
    "Heidi",
    "Silje",
    "Tone",
    "Anita",
    "Grete",
    "Wenche",
    "Hanne",
    "Linda",
    "Monica",
    "Camilla",
    "Ellen",
    "Lene",
    "Mette",
    "Turid",
    "Karin",
    "Tove",
    "Ragnhild",
    "Siri",
    "Gunn",
    "Unni",
    "Emma",
    "Nora",
    "Sara",
    "Sofie",
    "Thea",
    "Julie",
    "Emilie",
    "Maria",
    "Vilde",
    "Martine",
];

/// Common last names in Norway.
pub const LAST_NAMES: &[&str] = &[
    "Hansen",
    "Johansen",
    "Olsen",
    "Larsen",
    "Andersen",
    "Pedersen",
    "Nilsen",
    "Kristiansen",
    "Jensen",
    "Karlsen",
    "Johnsen",
    "Pettersen",
    "Eriksen",
    "Berg",
    "Haugen",
    "Hagen",
    "Johannessen",
    "Andreassen",
    "Jacobsen",
    "Dahl",
    "Jørgensen",
    "Henriksen",
    "Lund",
    "Halvorsen",
    "Sørensen",
    "Jakobsen",
    "Moen",
    "Gundersen",
    "Iversen",
    "Strand",
    "Solberg",
    "Svendsen",
    "Eide",
    "Knutsen",
    "Martinsen",
    "Bøe",
    "Bakken",
    "Kristoffersen",
    "Lie",
    "Rasmussen",
    "Berge",
    "Moe",
    "Nygård",
    "Fredriksen",
    "Bakke",
    "Holm",
    "Lien",
    "Arnesen",
    "Løken",
    "Hauge",
];

/// Norwegian cities.
pub const CITIES: &[&str] = &[
    "Oslo",
    "Bergen",
    "Trondheim",
    "Stavanger",
    "Drammen",
    "Fredrikstad",
    "Kristiansand",
    "Sandnes",
    "Tromsø",
    "Sarpsborg",
    "Skien",
    "Ålesund",
    "Sandefjord",
    "Haugesund",
    "Tønsberg",
    "Moss",
    "Porsgrunn",
    "Bodø",
    "Arendal",
    "Hamar",
    "Larvik",
    "Halden",
    "Steinkjer",
    "Gjøvik",
    "Askøy",
    "Kongsberg",
    "Molde",
    "Harstad",
    "Lillehammer",
    "Horten",
    "Ski",
    "Alta",
    "Kristiansund",
    "Elverum",
    "Grimstad",
    "Narvik",
    "Askim",
    "Rana",
    "Mandal",
    "Lørenskog",
];

/// Norwegian counties (fylker).
pub const COUNTIES: &[&str] = &[
    "Oslo",
    "Rogaland",
    "Møre og Romsdal",
    "Nordland",
    "Viken",
    "Innlandet",
    "Vestfold og Telemark",
    "Agder",
    "Vestland",
    "Trøndelag",
    "Troms og Finnmark",
];

/// Norwegian street suffixes.
pub const STREET_SUFFIXES: &[&str] = &[
    "gata", "gate", "veien", "vei", "allé", "plass", "torg", "bakken", "løkka", "stien", "svingen",
    "tunet", "haugen", "lia",
];

/// Norwegian street names.
pub const STREET_NAMES: &[&str] = &[
    "Karl Johans",
    "Storgata",
    "Kirkegata",
    "Kongensgate",
    "Dronningens",
    "Prinsens",
    "Torggata",
    "Grensen",
    "Akersgata",
    "Bogstadveien",
    "Majorstuen",
    "Frogner",
    "Grünerløkka",
    "Sagene",
    "Bjørvika",
    "Aker Brygge",
    "Tjuvholmen",
    "Vika",
    "Sentrum",
    "Gamle Oslo",
    "Nordstrand",
    "Lambertseter",
    "Manglerud",
    "Helsfyr",
    "Tøyen",
    "Grønland",
    "Gamlebyen",
    "Ekeberg",
    "Holmenkollen",
    "Smestad",
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

/// Generate a Norwegian phone number in XXX XX XXX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    // Norwegian mobile numbers start with 4 or 9
    let prefix = if rng.random_bool(0.5) { 4 } else { 9 };
    let rest: u32 = rng.random_range(0..10_000_000);
    format!("{}{:07}", prefix, rest)
}

/// Generate a Norwegian phone number in +47XXXXXXXX format.
pub fn phone_e164<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = if rng.random_bool(0.5) { 4 } else { 9 };
    let rest: u32 = rng.random_range(0..10_000_000);
    format!("+47{}{:07}", prefix, rest)
}

/// Generate a Norwegian phone number formatted with spaces.
pub fn phone_formatted<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = if rng.random_bool(0.5) { 4 } else { 9 };
    let d1: u32 = rng.random_range(0..100);
    let d2: u32 = rng.random_range(0..100);
    let d3: u32 = rng.random_range(0..1000);
    format!("{}{:02} {:02} {:03}", prefix, d1, d2, d3)
}

/// Get a random city.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    CITIES[rng.random_range(0..CITIES.len())]
}

/// Get a random county (fylke).
pub fn county<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    COUNTIES[rng.random_range(0..COUNTIES.len())]
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

/// Generate a postal code (postnummer).
pub fn postal_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{:04}", rng.random_range(1..9999))
}

/// Generate a full address.
pub fn full_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let street = street_address(rng);
    let postal = postal_code(rng);
    let city_name = city(rng);
    format!("{}, {} {}", street, postal, city_name)
}

/// Generate a Norwegian organization number (organisasjonsnummer).
pub fn org_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    // Norwegian org numbers are 9 digits, first digit is 8 or 9 for companies
    let first = if rng.random_bool(0.5) { 8 } else { 9 };
    let rest: u32 = rng.random_range(0..100_000_000);
    format!("{}{:08}", first, rest)
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
        assert_eq!(phone_num.len(), 8);
        let first = phone_num.chars().next().unwrap();
        assert!(first == '4' || first == '9');
    }

    #[test]
    fn test_phone_e164() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone_num = phone_e164(&mut rng);
        assert!(phone_num.starts_with("+47"));
        assert_eq!(phone_num.len(), 11);
    }

    #[test]
    fn test_phone_formatted() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone_num = phone_formatted(&mut rng);
        assert!(phone_num.contains(' '));
    }

    #[test]
    fn test_city() {
        let mut rng = StdRng::seed_from_u64(42);
        let city_name = city(&mut rng);
        assert!(CITIES.contains(&city_name));
    }

    #[test]
    fn test_county() {
        let mut rng = StdRng::seed_from_u64(42);
        let county_name = county(&mut rng);
        assert!(COUNTIES.contains(&county_name));
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
        assert_eq!(code.len(), 4);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_full_address() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = full_address(&mut rng);
        assert!(addr.contains(','));
    }

    #[test]
    fn test_org_number() {
        let mut rng = StdRng::seed_from_u64(42);
        let org = org_number(&mut rng);
        assert_eq!(org.len(), 9);
        let first = org.chars().next().unwrap();
        assert!(first == '8' || first == '9');
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
