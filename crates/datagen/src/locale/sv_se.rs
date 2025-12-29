//! Swedish locale data (sv_SE).
//!
//! Provides Swedish-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// Swedish locale marker type.
pub struct SvSe;

/// Common male first names in Sweden.
pub const MALE_FIRST_NAMES: &[&str] = &[
    "Lucas",
    "Liam",
    "William",
    "Elias",
    "Noah",
    "Hugo",
    "Oliver",
    "Oscar",
    "Adam",
    "Matteo",
    "Alexander",
    "Viktor",
    "Erik",
    "Filip",
    "Emil",
    "Axel",
    "Leo",
    "Theo",
    "Isak",
    "Gustav",
    "Lars",
    "Erik",
    "Karl",
    "Anders",
    "Johan",
    "Per",
    "Nils",
    "Sven",
    "Magnus",
    "Fredrik",
    "Henrik",
    "Mikael",
    "Stefan",
    "Jonas",
    "Daniel",
    "Tobias",
    "Marcus",
    "Patrik",
    "Ulf",
    "Olof",
];

/// Common female first names in Sweden.
pub const FEMALE_FIRST_NAMES: &[&str] = &[
    "Alice",
    "Maja",
    "Elsa",
    "Ella",
    "Wilma",
    "Olivia",
    "Astrid",
    "Ebba",
    "Julia",
    "Alma",
    "Saga",
    "Freja",
    "Agnes",
    "Lilly",
    "Vera",
    "Selma",
    "Klara",
    "Stella",
    "Molly",
    "Emilia",
    "Anna",
    "Maria",
    "Eva",
    "Karin",
    "Sara",
    "Emma",
    "Lena",
    "Ingrid",
    "Elisabeth",
    "Kristina",
    "Birgitta",
    "Margareta",
    "Ulrika",
    "Christina",
    "Helena",
    "Marianne",
    "Monica",
    "Annika",
    "Elin",
    "Johanna",
];

/// Common last names in Sweden.
pub const LAST_NAMES: &[&str] = &[
    "Andersson",
    "Johansson",
    "Karlsson",
    "Nilsson",
    "Eriksson",
    "Larsson",
    "Olsson",
    "Persson",
    "Svensson",
    "Gustafsson",
    "Pettersson",
    "Jonsson",
    "Jansson",
    "Hansson",
    "Bengtsson",
    "Jönsson",
    "Lindberg",
    "Lindström",
    "Lindqvist",
    "Mattsson",
    "Berglund",
    "Fredriksson",
    "Sandberg",
    "Henriksson",
    "Forsberg",
    "Sjöberg",
    "Wallin",
    "Engström",
    "Eklund",
    "Danielsson",
    "Lundgren",
    "Lindgren",
    "Björk",
    "Bergström",
    "Nordström",
    "Nyström",
    "Holmberg",
    "Sundberg",
    "Blomqvist",
    "Söderberg",
    "Bergqvist",
    "Nyberg",
    "Lundqvist",
    "Norberg",
    "Fransson",
    "Magnusson",
    "Lundberg",
    "Åberg",
    "Holm",
    "Samuelsson",
];

/// Swedish cities.
pub const CITIES: &[&str] = &[
    "Stockholm",
    "Göteborg",
    "Malmö",
    "Uppsala",
    "Linköping",
    "Västerås",
    "Örebro",
    "Helsingborg",
    "Norrköping",
    "Jönköping",
    "Umeå",
    "Lund",
    "Borås",
    "Huddinge",
    "Eskilstuna",
    "Gävle",
    "Sundsvall",
    "Södertälje",
    "Halmstad",
    "Växjö",
    "Karlstad",
    "Täby",
    "Kristianstad",
    "Kalmar",
    "Luleå",
    "Borlänge",
    "Falun",
    "Skövde",
    "Trollhättan",
    "Visby",
];

/// Swedish counties (län).
pub const COUNTIES: &[&str] = &[
    "Stockholms län",
    "Uppsala län",
    "Södermanlands län",
    "Östergötlands län",
    "Jönköpings län",
    "Kronobergs län",
    "Kalmar län",
    "Gotlands län",
    "Blekinge län",
    "Skåne län",
    "Hallands län",
    "Västra Götalands län",
    "Värmlands län",
    "Örebro län",
    "Västmanlands län",
    "Dalarnas län",
    "Gävleborgs län",
    "Västernorrlands län",
    "Jämtlands län",
    "Västerbottens län",
    "Norrbottens län",
];

/// Street types in Sweden.
pub const STREET_SUFFIXES: &[&str] = &[
    "gatan", "vägen", "stigen", "allén", "torget", "platsen", "gränd", "backen", "parken", "leden",
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

/// Generate a Swedish phone number in 0XX-XXX XX XX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let area_codes = [
        "08", "031", "040", "018", "013", "021", "019", "042", "011", "036",
    ];
    let area = area_codes[rng.random_range(0..area_codes.len())];
    let local: u32 = rng.random_range(100000..999999);
    format!("{}-{}", area, local)
}

/// Generate a Swedish mobile phone number.
pub fn mobile_phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefixes = ["070", "072", "073", "076", "079"];
    let prefix = prefixes[rng.random_range(0..prefixes.len())];
    let local: u32 = rng.random_range(1000000..9999999);
    format!("{}-{}", prefix, local)
}

/// Get a random city.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    CITIES[rng.random_range(0..CITIES.len())]
}

/// Get a random county.
pub fn county<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    COUNTIES[rng.random_range(0..COUNTIES.len())]
}

/// Get a random street suffix.
pub fn street_suffix<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    STREET_SUFFIXES[rng.random_range(0..STREET_SUFFIXES.len())]
}

/// Generate a street address.
pub fn street_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let names = [
        "Stor", "Lilla", "Kungs", "Drott", "Norra", "Södra", "Östra", "Västra", "Ny", "Gamla",
    ];
    let name = names[rng.random_range(0..names.len())];
    let suffix = street_suffix(rng);
    let number = rng.random_range(1..200);
    format!("{}{} {}", name, suffix, number)
}

/// Generate a postal code (Swedish format: XXX XX).
pub fn postal_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{:03} {:02}",
        rng.random_range(100..999),
        rng.random_range(10..99)
    )
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
        assert_eq!(code.len(), 6);
    }
}
