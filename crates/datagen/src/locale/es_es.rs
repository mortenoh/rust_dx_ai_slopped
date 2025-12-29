//! Spanish (Spain) locale data.
//!
//! Provides Spanish-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// Spanish locale marker type.
pub struct EsEs;

/// Common male first names in Spain.
pub const MALE_FIRST_NAMES: &[&str] = &[
    "Alejandro",
    "Antonio",
    "Carlos",
    "Daniel",
    "David",
    "Diego",
    "Eduardo",
    "Fernando",
    "Francisco",
    "Gabriel",
    "Hugo",
    "Javier",
    "Jesús",
    "Jorge",
    "José",
    "Juan",
    "Luis",
    "Manuel",
    "Marcos",
    "Mario",
    "Martín",
    "Miguel",
    "Nicolás",
    "Pablo",
    "Pedro",
    "Rafael",
    "Ramón",
    "Roberto",
    "Rubén",
    "Sergio",
    "Adrián",
    "Álvaro",
    "Ángel",
    "Andrés",
    "Alberto",
    "Enrique",
    "Iván",
    "Óscar",
    "Raúl",
    "Víctor",
    "Guillermo",
    "Ignacio",
    "Jaime",
    "Joaquín",
    "Lucas",
    "Mateo",
    "Nicolás",
    "Santiago",
    "Tomás",
    "Vicente",
];

/// Common female first names in Spain.
pub const FEMALE_FIRST_NAMES: &[&str] = &[
    "Ana",
    "Andrea",
    "Ángela",
    "Beatriz",
    "Carmen",
    "Carla",
    "Carolina",
    "Claudia",
    "Cristina",
    "Diana",
    "Elena",
    "Eva",
    "Inés",
    "Isabel",
    "Julia",
    "Laura",
    "Lucía",
    "María",
    "Marta",
    "Mercedes",
    "Natalia",
    "Nuria",
    "Paula",
    "Patricia",
    "Pilar",
    "Raquel",
    "Rosa",
    "Sandra",
    "Sara",
    "Sofía",
    "Silvia",
    "Teresa",
    "Verónica",
    "Alba",
    "Alicia",
    "Blanca",
    "Clara",
    "Esther",
    "Irene",
    "Lola",
    "Manuela",
    "Marina",
    "Miriam",
    "Mónica",
    "Olga",
    "Rocío",
    "Susana",
    "Valeria",
    "Victoria",
    "Yolanda",
];

/// Common last names in Spain.
pub const LAST_NAMES: &[&str] = &[
    "García",
    "Rodríguez",
    "Martínez",
    "López",
    "González",
    "Hernández",
    "Pérez",
    "Sánchez",
    "Ramírez",
    "Torres",
    "Flores",
    "Rivera",
    "Gómez",
    "Díaz",
    "Reyes",
    "Cruz",
    "Morales",
    "Ortiz",
    "Gutiérrez",
    "Chávez",
    "Fernández",
    "Romero",
    "Álvarez",
    "Ruiz",
    "Jiménez",
    "Moreno",
    "Muñoz",
    "Alonso",
    "Navarro",
    "Domínguez",
    "Vázquez",
    "Ramos",
    "Gil",
    "Serrano",
    "Blanco",
    "Molina",
    "Suárez",
    "Castro",
    "Ortega",
    "Delgado",
    "Rubio",
    "Marín",
    "Sanz",
    "Núñez",
    "Iglesias",
    "Medina",
    "Santos",
    "Castillo",
    "Cortés",
    "Garrido",
];

/// Spanish cities.
pub const CITIES: &[&str] = &[
    "Madrid",
    "Barcelona",
    "Valencia",
    "Sevilla",
    "Zaragoza",
    "Málaga",
    "Murcia",
    "Palma",
    "Las Palmas de Gran Canaria",
    "Bilbao",
    "Alicante",
    "Córdoba",
    "Valladolid",
    "Vigo",
    "Gijón",
    "L'Hospitalet de Llobregat",
    "A Coruña",
    "Vitoria-Gasteiz",
    "Granada",
    "Elche",
    "Oviedo",
    "Santa Cruz de Tenerife",
    "Badalona",
    "Cartagena",
    "Terrassa",
    "Jerez de la Frontera",
    "Sabadell",
    "Móstoles",
    "Alcalá de Henares",
    "Pamplona",
    "Fuenlabrada",
    "Almería",
    "San Sebastián",
    "Leganés",
    "Santander",
    "Burgos",
    "Castellón de la Plana",
    "Getafe",
    "Albacete",
    "Alcorcón",
];

/// Spanish autonomous communities.
pub const COMMUNITIES: &[&str] = &[
    "Andalucía",
    "Aragón",
    "Asturias",
    "Islas Baleares",
    "Canarias",
    "Cantabria",
    "Castilla-La Mancha",
    "Castilla y León",
    "Cataluña",
    "Comunidad Valenciana",
    "Extremadura",
    "Galicia",
    "La Rioja",
    "Madrid",
    "Murcia",
    "Navarra",
    "País Vasco",
];

/// Street types (tipos de vía).
pub const STREET_SUFFIXES: &[&str] = &[
    "Calle",
    "Avenida",
    "Plaza",
    "Paseo",
    "Ronda",
    "Camino",
    "Carretera",
    "Travesía",
    "Glorieta",
    "Alameda",
    "Rambla",
    "Vía",
    "Callejón",
    "Sendero",
];

/// Street names (common words used in Spanish street names).
pub const STREET_NAMES: &[&str] = &[
    "Mayor",
    "de la Constitución",
    "Real",
    "del Sol",
    "de la Paz",
    "de España",
    "del Carmen",
    "de San Juan",
    "de la Libertad",
    "Nueva",
    "del Prado",
    "de Cervantes",
    "de Goya",
    "de Velázquez",
    "del Pilar",
    "de la Victoria",
    "de Santa María",
    "del Rey",
    "de la Reina",
    "de San Pedro",
    "de San Miguel",
    "del Ayuntamiento",
    "de la Iglesia",
    "del Mercado",
    "del Parque",
    "de los Reyes Católicos",
    "de Colón",
    "de Gran Vía",
    "del Generalísimo",
    "de la Marina",
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

/// Generate a full name (with two last names, Spanish style).
pub fn full_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} {} {}", first_name(rng), last_name(rng), last_name(rng))
}

/// Generate a Spanish phone number in XXX XXX XXX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    // Spanish mobile prefixes: 6XX, 7XX
    let prefix = if rng.random_bool(0.5) { 6 } else { 7 };
    let d1: u8 = rng.random_range(0..10);
    let d2: u8 = rng.random_range(0..10);
    let rest1: u16 = rng.random_range(0..1000);
    let rest2: u16 = rng.random_range(0..1000);
    format!("{}{}{} {:03} {:03}", prefix, d1, d2, rest1, rest2)
}

/// Generate a Spanish phone number in +34XXXXXXXXX format.
pub fn phone_e164<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = if rng.random_bool(0.5) { 6 } else { 7 };
    let rest: u32 = rng.random_range(0..100_000_000);
    format!("+34{}{:08}", prefix, rest)
}

/// Get a random city.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    CITIES[rng.random_range(0..CITIES.len())]
}

/// Get a random autonomous community.
pub fn community<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    COMMUNITIES[rng.random_range(0..COMMUNITIES.len())]
}

/// Get a random street suffix (tipo de vía).
pub fn street_suffix<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    STREET_SUFFIXES[rng.random_range(0..STREET_SUFFIXES.len())]
}

/// Generate a street address.
pub fn street_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let suffix = street_suffix(rng);
    let street = STREET_NAMES[rng.random_range(0..STREET_NAMES.len())];
    let number = rng.random_range(1..200);
    format!("{} {}, {}", suffix, street, number)
}

/// Generate a postal code (código postal).
pub fn postal_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    // Spanish postal codes range from 01000 to 52999
    format!("{:05}", rng.random_range(1000..52999))
}

/// Generate a full address.
pub fn full_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let street = street_address(rng);
    let postal = postal_code(rng);
    let city_name = city(rng);
    format!("{}, {} {}", street, postal, city_name)
}

/// Generate a Spanish DNI (Documento Nacional de Identidad).
pub fn dni<R: ?Sized + Rng>(rng: &mut R) -> String {
    // DNI is 8 digits + 1 letter
    const DNI_LETTERS: &[char] = &[
        'T', 'R', 'W', 'A', 'G', 'M', 'Y', 'F', 'P', 'D', 'X', 'B', 'N', 'J', 'Z', 'S', 'Q', 'V',
        'H', 'L', 'C', 'K', 'E',
    ];
    let number: u32 = rng.random_range(10_000_000..99_999_999);
    let letter = DNI_LETTERS[(number % 23) as usize];
    format!("{}{}", number, letter)
}

/// Generate a Spanish NIE (Número de Identidad de Extranjero).
pub fn nie<R: ?Sized + Rng>(rng: &mut R) -> String {
    // NIE starts with X, Y, or Z
    const NIE_PREFIXES: &[char] = &['X', 'Y', 'Z'];
    const DNI_LETTERS: &[char] = &[
        'T', 'R', 'W', 'A', 'G', 'M', 'Y', 'F', 'P', 'D', 'X', 'B', 'N', 'J', 'Z', 'S', 'Q', 'V',
        'H', 'L', 'C', 'K', 'E',
    ];
    let prefix = NIE_PREFIXES[rng.random_range(0..NIE_PREFIXES.len())];
    let number: u32 = rng.random_range(0..10_000_000);
    let prefix_value = match prefix {
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => 0,
    };
    let full_number = prefix_value * 10_000_000 + number;
    let letter = DNI_LETTERS[(full_number % 23) as usize];
    format!("{}{:07}{}", prefix, number, letter)
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
        // Spanish names have two last names
        let parts: Vec<&str> = name.split(' ').collect();
        assert!(parts.len() >= 3);
    }

    #[test]
    fn test_phone() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone_num = phone(&mut rng);
        assert!(phone_num.starts_with('6') || phone_num.starts_with('7'));
        assert!(phone_num.contains(' '));
    }

    #[test]
    fn test_phone_e164() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone_num = phone_e164(&mut rng);
        assert!(phone_num.starts_with("+34"));
    }

    #[test]
    fn test_city() {
        let mut rng = StdRng::seed_from_u64(42);
        let city_name = city(&mut rng);
        assert!(CITIES.contains(&city_name));
    }

    #[test]
    fn test_community() {
        let mut rng = StdRng::seed_from_u64(42);
        let community_name = community(&mut rng);
        assert!(COMMUNITIES.contains(&community_name));
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
    fn test_dni() {
        let mut rng = StdRng::seed_from_u64(42);
        let id = dni(&mut rng);
        assert_eq!(id.len(), 9);
        assert!(id.chars().take(8).all(|c| c.is_ascii_digit()));
        assert!(id.chars().last().unwrap().is_ascii_alphabetic());
    }

    #[test]
    fn test_nie() {
        let mut rng = StdRng::seed_from_u64(42);
        let id = nie(&mut rng);
        assert_eq!(id.len(), 9);
        let first = id.chars().next().unwrap();
        assert!(first == 'X' || first == 'Y' || first == 'Z');
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
