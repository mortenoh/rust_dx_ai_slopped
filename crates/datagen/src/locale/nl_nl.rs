//! Dutch locale data (nl_NL).
//!
//! Provides Dutch-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// Dutch locale marker type.
pub struct NlNl;

/// Common male first names in the Netherlands.
pub const MALE_FIRST_NAMES: &[&str] = &[
    "Noah", "Daan", "Sem", "Lucas", "Levi", "Finn", "Luuk", "Milan", "Jesse", "Bram", "Liam",
    "Max", "Thomas", "Tim", "Lars", "Ruben", "Julian", "Sven", "Thijs", "Stan", "Jan", "Pieter",
    "Willem", "Hendrik", "Cornelis", "Johannes", "Gerrit", "Jacob", "Dirk", "Adriaan", "Maarten",
    "Jeroen", "Bas", "Mark", "Frank", "Erik", "Paul", "Rob", "Dennis", "Michiel",
];

/// Common female first names in the Netherlands.
pub const FEMALE_FIRST_NAMES: &[&str] = &[
    "Emma",
    "Julia",
    "Sophie",
    "Tessa",
    "Anna",
    "Mila",
    "Sara",
    "Fleur",
    "Zoey",
    "Noor",
    "Lotte",
    "Eva",
    "Lisa",
    "Sanne",
    "Lynn",
    "Amber",
    "Nina",
    "Femke",
    "Anouk",
    "Iris",
    "Maria",
    "Johanna",
    "Elisabeth",
    "Cornelia",
    "Wilhelmina",
    "Hendrika",
    "Margaretha",
    "Adriana",
    "Catharina",
    "Christina",
    "Marieke",
    "Linda",
    "Monique",
    "Sandra",
    "Patricia",
    "Nicole",
    "Esther",
    "Annemarie",
    "Bianca",
    "Ingrid",
];

/// Common last names in the Netherlands.
pub const LAST_NAMES: &[&str] = &[
    "De Jong",
    "Jansen",
    "De Vries",
    "Van den Berg",
    "Van Dijk",
    "Bakker",
    "Janssen",
    "Visser",
    "Smit",
    "Meijer",
    "De Boer",
    "Mulder",
    "De Groot",
    "Bos",
    "Vos",
    "Peters",
    "Hendriks",
    "Van Leeuwen",
    "Dekker",
    "Brouwer",
    "De Wit",
    "Dijkstra",
    "Smits",
    "De Graaf",
    "Van der Linden",
    "Kok",
    "Jacobs",
    "De Haan",
    "Vermeer",
    "Van den Heuvel",
    "Van der Veen",
    "Van den Broek",
    "De Bruijn",
    "De Bruin",
    "Van der Heijden",
    "Schouten",
    "Van Beek",
    "Willems",
    "Van Vliet",
    "Van de Ven",
    "Hoekstra",
    "Maas",
    "Verhoeven",
    "Koster",
    "Van Dam",
    "Van der Wal",
    "Prins",
    "Blom",
    "Huisman",
    "Peeters",
];

/// Dutch cities.
pub const CITIES: &[&str] = &[
    "Amsterdam",
    "Rotterdam",
    "Den Haag",
    "Utrecht",
    "Eindhoven",
    "Tilburg",
    "Groningen",
    "Almere",
    "Breda",
    "Nijmegen",
    "Enschede",
    "Haarlem",
    "Arnhem",
    "Zaanstad",
    "Amersfoort",
    "Apeldoorn",
    "Hoofddorp",
    "Maastricht",
    "Leiden",
    "Dordrecht",
    "Zoetermeer",
    "Zwolle",
    "Deventer",
    "Delft",
    "Alkmaar",
    "Heerlen",
    "Venlo",
    "Leeuwarden",
    "Hilversum",
    "Amstelveen",
];

/// Dutch provinces.
pub const PROVINCES: &[&str] = &[
    "Noord-Holland",
    "Zuid-Holland",
    "Utrecht",
    "Noord-Brabant",
    "Gelderland",
    "Overijssel",
    "Limburg",
    "Friesland",
    "Groningen",
    "Drenthe",
    "Zeeland",
    "Flevoland",
];

/// Street types in the Netherlands.
pub const STREET_SUFFIXES: &[&str] = &[
    "straat", "weg", "laan", "plein", "gracht", "kade", "steeg", "pad", "singel", "dreef",
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

/// Generate a Dutch phone number in 0XX-XXX XXXX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let area_codes = [
        "020", "010", "070", "030", "040", "050", "038", "043", "071", "013",
    ];
    let area = area_codes[rng.random_range(0..area_codes.len())];
    let local: u32 = rng.random_range(1000000..9999999);
    format!("{}-{}", area, local)
}

/// Generate a Dutch mobile phone number.
pub fn mobile_phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = "06";
    let local: u32 = rng.random_range(10000000..99999999);
    format!("{}-{}", prefix, local)
}

/// Get a random city.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    CITIES[rng.random_range(0..CITIES.len())]
}

/// Get a random province.
pub fn province<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    PROVINCES[rng.random_range(0..PROVINCES.len())]
}

/// Get a random street suffix.
pub fn street_suffix<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    STREET_SUFFIXES[rng.random_range(0..STREET_SUFFIXES.len())]
}

/// Generate a street address.
pub fn street_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let names = [
        "Koning", "Prins", "Dam", "Oost", "West", "Noord", "Zuid", "Hoog", "Laag", "Nieuw",
    ];
    let name = names[rng.random_range(0..names.len())];
    let suffix = street_suffix(rng);
    let number = rng.random_range(1..200);
    format!("{}{} {}", name, suffix, number)
}

/// Generate a postal code (Dutch format: XXXX XX).
pub fn postal_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    let digits: u16 = rng.random_range(1000..9999);
    let letters: String = (0..2)
        .map(|_| (b'A' + rng.random_range(0..26)) as char)
        .collect();
    format!("{} {}", digits, letters)
}

/// Generate a full address.
pub fn full_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let street = street_address(rng);
    let post = postal_code(rng);
    let city_name = city(rng);
    format!("{}\n{} {}", street, post, city_name)
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
    fn test_phone() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone_num = phone(&mut rng);
        assert!(phone_num.contains('-'));
    }

    #[test]
    fn test_postal_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let code = postal_code(&mut rng);
        assert!(code.contains(' '));
        assert_eq!(code.len(), 7);
    }
}
