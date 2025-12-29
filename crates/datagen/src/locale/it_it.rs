//! Italian locale data (it_IT).
//!
//! Provides Italian-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// Italian locale marker type.
pub struct ItIt;

/// Common male first names in Italy.
pub const MALE_FIRST_NAMES: &[&str] = &[
    "Leonardo",
    "Francesco",
    "Alessandro",
    "Lorenzo",
    "Mattia",
    "Andrea",
    "Gabriele",
    "Riccardo",
    "Tommaso",
    "Edoardo",
    "Nicolo",
    "Federico",
    "Giuseppe",
    "Marco",
    "Luca",
    "Antonio",
    "Giovanni",
    "Paolo",
    "Filippo",
    "Davide",
    "Simone",
    "Matteo",
    "Pietro",
    "Stefano",
    "Michele",
    "Roberto",
    "Alberto",
    "Claudio",
    "Fabio",
    "Massimo",
    "Gianluca",
    "Emanuele",
    "Daniele",
    "Sergio",
    "Vincenzo",
    "Salvatore",
    "Enrico",
    "Angelo",
    "Franco",
    "Bruno",
];

/// Common female first names in Italy.
pub const FEMALE_FIRST_NAMES: &[&str] = &[
    "Sofia",
    "Giulia",
    "Aurora",
    "Alice",
    "Ginevra",
    "Emma",
    "Giorgia",
    "Greta",
    "Beatrice",
    "Anna",
    "Chiara",
    "Sara",
    "Nicole",
    "Martina",
    "Francesca",
    "Elena",
    "Maria",
    "Alessia",
    "Valentina",
    "Elisa",
    "Federica",
    "Silvia",
    "Claudia",
    "Roberta",
    "Cristina",
    "Laura",
    "Monica",
    "Paola",
    "Barbara",
    "Simona",
    "Ilaria",
    "Michela",
    "Serena",
    "Veronica",
    "Patrizia",
    "Daniela",
    "Lucia",
    "Angela",
    "Carla",
    "Rosa",
];

/// Common last names in Italy.
pub const LAST_NAMES: &[&str] = &[
    "Rossi",
    "Russo",
    "Ferrari",
    "Esposito",
    "Bianchi",
    "Romano",
    "Colombo",
    "Ricci",
    "Marino",
    "Greco",
    "Bruno",
    "Gallo",
    "Conti",
    "De Luca",
    "Mancini",
    "Costa",
    "Giordano",
    "Rizzo",
    "Lombardi",
    "Moretti",
    "Barbieri",
    "Fontana",
    "Santoro",
    "Mariani",
    "Rinaldi",
    "Caruso",
    "Ferrara",
    "Galli",
    "Martini",
    "Leone",
    "Longo",
    "Gentile",
    "Martinelli",
    "Vitale",
    "Lombardo",
    "Serra",
    "Coppola",
    "De Santis",
    "D'Angelo",
    "Marchetti",
    "Parisi",
    "Villa",
    "Conte",
    "Ferraro",
    "Ferri",
    "Fabbri",
    "Bianco",
    "Marini",
    "Grasso",
    "Valentini",
];

/// Italian cities.
pub const CITIES: &[&str] = &[
    "Roma",
    "Milano",
    "Napoli",
    "Torino",
    "Palermo",
    "Genova",
    "Bologna",
    "Firenze",
    "Bari",
    "Catania",
    "Venezia",
    "Verona",
    "Messina",
    "Padova",
    "Trieste",
    "Brescia",
    "Parma",
    "Taranto",
    "Prato",
    "Modena",
    "Reggio Calabria",
    "Reggio Emilia",
    "Perugia",
    "Livorno",
    "Ravenna",
    "Cagliari",
    "Foggia",
    "Rimini",
    "Salerno",
    "Ferrara",
];

/// Italian regions.
pub const REGIONS: &[&str] = &[
    "Lombardia",
    "Lazio",
    "Campania",
    "Sicilia",
    "Veneto",
    "Emilia-Romagna",
    "Piemonte",
    "Puglia",
    "Toscana",
    "Calabria",
    "Sardegna",
    "Liguria",
    "Marche",
    "Abruzzo",
    "Friuli-Venezia Giulia",
    "Trentino-Alto Adige",
    "Umbria",
    "Basilicata",
    "Molise",
    "Valle d'Aosta",
];

/// Street types in Italy.
pub const STREET_SUFFIXES: &[&str] = &[
    "Via",
    "Viale",
    "Piazza",
    "Corso",
    "Largo",
    "Vicolo",
    "Strada",
    "Lungomare",
    "Borgo",
    "Galleria",
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

/// Generate an Italian phone number in +39 XXX XXX XXXX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefixes = [
        "02", "06", "011", "051", "055", "081", "091", "010", "041", "049",
    ];
    let prefix = prefixes[rng.random_range(0..prefixes.len())];
    let local: u32 = rng.random_range(1000000..9999999);
    format!("{} {}", prefix, local)
}

/// Generate an Italian mobile phone number.
pub fn mobile_phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefixes = [
        "320", "328", "329", "330", "331", "333", "334", "335", "336", "337", "338", "339", "340",
        "347", "348", "349",
    ];
    let prefix = prefixes[rng.random_range(0..prefixes.len())];
    let local: u32 = rng.random_range(1000000..9999999);
    format!("{} {}", prefix, local)
}

/// Get a random city.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    CITIES[rng.random_range(0..CITIES.len())]
}

/// Get a random region.
pub fn region<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    REGIONS[rng.random_range(0..REGIONS.len())]
}

/// Get a random street suffix.
pub fn street_suffix<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    STREET_SUFFIXES[rng.random_range(0..STREET_SUFFIXES.len())]
}

/// Generate a street address.
pub fn street_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let tipo = street_suffix(rng);
    let number = rng.random_range(1..200);
    let names = [
        "Roma",
        "Milano",
        "Garibaldi",
        "Dante",
        "Mazzini",
        "Cavour",
        "Vittorio Emanuele",
        "Italia",
        "Europa",
        "Libertà",
    ];
    let name = names[rng.random_range(0..names.len())];
    format!("{} {}, {}", tipo, name, number)
}

/// Generate a postal code (CAP format: 5 digits).
pub fn postal_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{:05}", rng.random_range(10000..99999))
}

/// Generate a full address.
pub fn full_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let street = street_address(rng);
    let cap = postal_code(rng);
    let city_name = city(rng);
    format!("{} - {} {}", street, cap, city_name)
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
        assert!(!phone_num.is_empty());
    }

    #[test]
    fn test_postal_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let code = postal_code(&mut rng);
        assert_eq!(code.len(), 5);
    }
}
