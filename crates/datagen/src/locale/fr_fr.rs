//! French (France) locale data.
//!
//! Provides French-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// French locale marker type.
pub struct FrFr;

/// Common male first names in France.
pub const MALE_FIRST_NAMES: &[&str] = &[
    "Adam",
    "Alexandre",
    "Antoine",
    "Arthur",
    "Baptiste",
    "Benjamin",
    "Charles",
    "Christophe",
    "Clément",
    "David",
    "Éric",
    "Étienne",
    "Fabien",
    "François",
    "Gabriel",
    "Guillaume",
    "Hugo",
    "Jacques",
    "Jean",
    "Julien",
    "Laurent",
    "Léo",
    "Louis",
    "Lucas",
    "Marc",
    "Mathieu",
    "Maxime",
    "Michel",
    "Nathan",
    "Nicolas",
    "Olivier",
    "Pascal",
    "Patrick",
    "Paul",
    "Philippe",
    "Pierre",
    "Raphaël",
    "Romain",
    "Samuel",
    "Sébastien",
    "Simon",
    "Théo",
    "Thomas",
    "Victor",
    "Vincent",
    "Xavier",
    "Yann",
    "Yves",
    "Adrien",
    "Alexis",
];

/// Common female first names in France.
pub const FEMALE_FIRST_NAMES: &[&str] = &[
    "Alice",
    "Amélie",
    "Anna",
    "Aurélie",
    "Camille",
    "Caroline",
    "Catherine",
    "Charlotte",
    "Chloé",
    "Christine",
    "Claire",
    "Clémence",
    "Émilie",
    "Emma",
    "Élodie",
    "Florence",
    "Françoise",
    "Isabelle",
    "Jade",
    "Julie",
    "Juliette",
    "Laure",
    "Laura",
    "Léa",
    "Louise",
    "Lucie",
    "Manon",
    "Margot",
    "Marie",
    "Marion",
    "Mathilde",
    "Nathalie",
    "Nicole",
    "Océane",
    "Pauline",
    "Sarah",
    "Sophie",
    "Stéphanie",
    "Sylvie",
    "Valérie",
    "Virginie",
    "Zoé",
    "Élise",
    "Inès",
    "Clara",
    "Anaïs",
    "Marine",
    "Céline",
    "Mélanie",
    "Audrey",
];

/// Common last names in France.
pub const LAST_NAMES: &[&str] = &[
    "Martin",
    "Bernard",
    "Thomas",
    "Petit",
    "Robert",
    "Richard",
    "Durand",
    "Dubois",
    "Moreau",
    "Laurent",
    "Simon",
    "Michel",
    "Lefebvre",
    "Leroy",
    "Roux",
    "David",
    "Bertrand",
    "Morel",
    "Fournier",
    "Girard",
    "Bonnet",
    "Dupont",
    "Lambert",
    "Fontaine",
    "Rousseau",
    "Vincent",
    "Muller",
    "Lefevre",
    "Faure",
    "Andre",
    "Mercier",
    "Blanc",
    "Guerin",
    "Boyer",
    "Garnier",
    "Chevalier",
    "François",
    "Legrand",
    "Gauthier",
    "Garcia",
    "Perrin",
    "Robin",
    "Clement",
    "Morin",
    "Nicolas",
    "Henry",
    "Roussel",
    "Mathieu",
    "Gautier",
    "Masson",
];

/// French cities.
pub const CITIES: &[&str] = &[
    "Paris",
    "Marseille",
    "Lyon",
    "Toulouse",
    "Nice",
    "Nantes",
    "Montpellier",
    "Strasbourg",
    "Bordeaux",
    "Lille",
    "Rennes",
    "Reims",
    "Saint-Étienne",
    "Le Havre",
    "Toulon",
    "Grenoble",
    "Dijon",
    "Angers",
    "Nîmes",
    "Villeurbanne",
    "Clermont-Ferrand",
    "Le Mans",
    "Aix-en-Provence",
    "Brest",
    "Tours",
    "Amiens",
    "Limoges",
    "Annecy",
    "Perpignan",
    "Boulogne-Billancourt",
    "Metz",
    "Besançon",
    "Orléans",
    "Rouen",
    "Mulhouse",
    "Caen",
    "Nancy",
    "Saint-Denis",
    "Argenteuil",
    "Montreuil",
];

/// French regions.
pub const REGIONS: &[&str] = &[
    "Auvergne-Rhône-Alpes",
    "Bourgogne-Franche-Comté",
    "Bretagne",
    "Centre-Val de Loire",
    "Corse",
    "Grand Est",
    "Hauts-de-France",
    "Île-de-France",
    "Normandie",
    "Nouvelle-Aquitaine",
    "Occitanie",
    "Pays de la Loire",
    "Provence-Alpes-Côte d'Azur",
];

/// Street types (types de voie).
pub const STREET_SUFFIXES: &[&str] = &[
    "rue",
    "avenue",
    "boulevard",
    "place",
    "allée",
    "chemin",
    "impasse",
    "passage",
    "quai",
    "square",
    "cours",
    "voie",
    "route",
    "sentier",
];

/// Street names (common words used in French street names).
pub const STREET_NAMES: &[&str] = &[
    "de la République",
    "de la Liberté",
    "Victor Hugo",
    "Jean Jaurès",
    "du Général de Gaulle",
    "Pasteur",
    "Gambetta",
    "de Paris",
    "de la Gare",
    "du Maréchal Foch",
    "Voltaire",
    "Jean Moulin",
    "des Fleurs",
    "du Commerce",
    "Saint-Jacques",
    "de l'Église",
    "de la Mairie",
    "du Château",
    "des Écoles",
    "du Parc",
    "Nationale",
    "de Verdun",
    "Lafayette",
    "Molière",
    "Émile Zola",
    "Pierre Curie",
    "Marie Curie",
    "du 8 Mai 1945",
    "du 11 Novembre",
    "des Lilas",
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

/// Generate a French phone number in XX XX XX XX XX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    // French mobile prefixes: 06, 07
    let prefix = if rng.random_bool(0.5) { "06" } else { "07" };
    let d1: u8 = rng.random_range(0..100);
    let d2: u8 = rng.random_range(0..100);
    let d3: u8 = rng.random_range(0..100);
    let d4: u8 = rng.random_range(0..100);
    format!("{} {:02} {:02} {:02} {:02}", prefix, d1, d2, d3, d4)
}

/// Generate a French phone number in +33XXXXXXXXX format.
pub fn phone_e164<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = if rng.random_bool(0.5) { "6" } else { "7" };
    let rest: u32 = rng.random_range(0..100_000_000);
    format!("+33{}{:08}", prefix, rest)
}

/// Get a random city.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    CITIES[rng.random_range(0..CITIES.len())]
}

/// Get a random region.
pub fn region<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    REGIONS[rng.random_range(0..REGIONS.len())]
}

/// Get a random street suffix (type de voie).
pub fn street_suffix<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    STREET_SUFFIXES[rng.random_range(0..STREET_SUFFIXES.len())]
}

/// Generate a street address.
pub fn street_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let number = rng.random_range(1..200);
    let suffix = street_suffix(rng);
    let street = STREET_NAMES[rng.random_range(0..STREET_NAMES.len())];
    format!("{} {} {}", number, suffix, street)
}

/// Generate a postal code (code postal).
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

/// Generate a French SIRET number (14 digits).
pub fn siret<R: ?Sized + Rng>(rng: &mut R) -> String {
    // SIRET = SIREN (9 digits) + NIC (5 digits)
    let siren: u64 = rng.random_range(100_000_000..999_999_999);
    let nic: u32 = rng.random_range(10000..99999);
    format!("{}{}", siren, nic)
}

/// Generate a French SIREN number (9 digits).
pub fn siren<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{:09}", rng.random_range(100_000_000u64..999_999_999))
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
        assert!(phone_num.starts_with("06") || phone_num.starts_with("07"));
        assert!(phone_num.contains(' '));
    }

    #[test]
    fn test_phone_e164() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone_num = phone_e164(&mut rng);
        assert!(phone_num.starts_with("+33"));
    }

    #[test]
    fn test_city() {
        let mut rng = StdRng::seed_from_u64(42);
        let city_name = city(&mut rng);
        assert!(CITIES.contains(&city_name));
    }

    #[test]
    fn test_region() {
        let mut rng = StdRng::seed_from_u64(42);
        let region_name = region(&mut rng);
        assert!(REGIONS.contains(&region_name));
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
    fn test_siret() {
        let mut rng = StdRng::seed_from_u64(42);
        let num = siret(&mut rng);
        assert_eq!(num.len(), 14);
        assert!(num.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_siren() {
        let mut rng = StdRng::seed_from_u64(42);
        let num = siren(&mut rng);
        assert_eq!(num.len(), 9);
        assert!(num.chars().all(|c| c.is_ascii_digit()));
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
