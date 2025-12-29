//! Brazilian Portuguese locale data (pt_BR).
//!
//! Provides Brazilian-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// Brazilian Portuguese locale marker type.
pub struct PtBr;

/// Common male first names in Brazil.
pub const MALE_FIRST_NAMES: &[&str] = &[
    "Miguel",
    "Arthur",
    "Heitor",
    "Bernardo",
    "Theo",
    "Davi",
    "Gabriel",
    "Pedro",
    "Samuel",
    "Lorenzo",
    "Benjamin",
    "Matheus",
    "Lucas",
    "Henrique",
    "Joaquim",
    "Enzo",
    "Nicolas",
    "Rafael",
    "Isaac",
    "Gustavo",
    "Felipe",
    "João",
    "Murilo",
    "Eduardo",
    "Daniel",
    "Guilherme",
    "Bruno",
    "Caio",
    "Vinicius",
    "André",
    "Ricardo",
    "Carlos",
    "Fernando",
    "Thiago",
    "Leonardo",
    "Diego",
    "Rodrigo",
    "Marcelo",
    "Paulo",
    "Roberto",
];

/// Common female first names in Brazil.
pub const FEMALE_FIRST_NAMES: &[&str] = &[
    "Helena",
    "Alice",
    "Laura",
    "Maria",
    "Valentina",
    "Sophia",
    "Isabella",
    "Manuela",
    "Julia",
    "Heloisa",
    "Luisa",
    "Lorena",
    "Livia",
    "Giovanna",
    "Maria Luiza",
    "Beatriz",
    "Maria Clara",
    "Cecilia",
    "Eloah",
    "Lara",
    "Ana",
    "Gabriela",
    "Camila",
    "Leticia",
    "Mariana",
    "Fernanda",
    "Juliana",
    "Patricia",
    "Adriana",
    "Amanda",
    "Bruna",
    "Carolina",
    "Daniela",
    "Tatiana",
    "Raquel",
    "Natalia",
    "Renata",
    "Vanessa",
    "Priscila",
    "Monica",
];

/// Common last names in Brazil.
pub const LAST_NAMES: &[&str] = &[
    "Silva",
    "Santos",
    "Oliveira",
    "Souza",
    "Rodrigues",
    "Ferreira",
    "Alves",
    "Pereira",
    "Lima",
    "Gomes",
    "Costa",
    "Ribeiro",
    "Martins",
    "Carvalho",
    "Almeida",
    "Lopes",
    "Soares",
    "Fernandes",
    "Vieira",
    "Barbosa",
    "Rocha",
    "Dias",
    "Nascimento",
    "Andrade",
    "Moreira",
    "Nunes",
    "Marques",
    "Machado",
    "Mendes",
    "Freitas",
    "Cardoso",
    "Ramos",
    "Gonçalves",
    "Santana",
    "Teixeira",
    "Reis",
    "Moura",
    "Araújo",
    "Melo",
    "Correia",
    "Pinto",
    "Batista",
    "Campos",
    "Mello",
    "Castro",
    "Azevedo",
    "Barros",
    "Miranda",
    "Cunha",
    "Monteiro",
];

/// Brazilian cities.
pub const CITIES: &[&str] = &[
    "São Paulo",
    "Rio de Janeiro",
    "Brasília",
    "Salvador",
    "Fortaleza",
    "Belo Horizonte",
    "Manaus",
    "Curitiba",
    "Recife",
    "Porto Alegre",
    "Belém",
    "Goiânia",
    "Guarulhos",
    "Campinas",
    "São Luís",
    "São Gonçalo",
    "Maceió",
    "Duque de Caxias",
    "Natal",
    "Campo Grande",
    "Teresina",
    "São Bernardo do Campo",
    "Nova Iguaçu",
    "João Pessoa",
    "Santo André",
    "São José dos Campos",
    "Osasco",
    "Ribeirão Preto",
    "Jaboatão dos Guararapes",
    "Uberlândia",
];

/// Brazilian states.
pub const STATES: &[(&str, &str)] = &[
    ("Acre", "AC"),
    ("Alagoas", "AL"),
    ("Amapá", "AP"),
    ("Amazonas", "AM"),
    ("Bahia", "BA"),
    ("Ceará", "CE"),
    ("Distrito Federal", "DF"),
    ("Espírito Santo", "ES"),
    ("Goiás", "GO"),
    ("Maranhão", "MA"),
    ("Mato Grosso", "MT"),
    ("Mato Grosso do Sul", "MS"),
    ("Minas Gerais", "MG"),
    ("Pará", "PA"),
    ("Paraíba", "PB"),
    ("Paraná", "PR"),
    ("Pernambuco", "PE"),
    ("Piauí", "PI"),
    ("Rio de Janeiro", "RJ"),
    ("Rio Grande do Norte", "RN"),
    ("Rio Grande do Sul", "RS"),
    ("Rondônia", "RO"),
    ("Roraima", "RR"),
    ("Santa Catarina", "SC"),
    ("São Paulo", "SP"),
    ("Sergipe", "SE"),
    ("Tocantins", "TO"),
];

/// Street types in Brazil.
pub const STREET_SUFFIXES: &[&str] = &[
    "Rua", "Avenida", "Travessa", "Praça", "Alameda", "Largo", "Estrada", "Rodovia", "Via", "Beco",
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

/// Generate a Brazilian phone number in (XX) XXXXX-XXXX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let area_codes = ["11", "21", "31", "41", "51", "61", "71", "81", "85", "92"];
    let area = area_codes[rng.random_range(0..area_codes.len())];
    let prefix = rng.random_range(90000..99999);
    let suffix = rng.random_range(0..10000);
    format!("({}) {}-{:04}", area, prefix, suffix)
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
    let tipo = street_suffix(rng);
    let number = rng.random_range(1..2000);
    let names = [
        "das Flores",
        "Brasil",
        "São Paulo",
        "Rio Branco",
        "Presidente Vargas",
        "Santos Dumont",
        "Dom Pedro",
        "Getúlio Vargas",
        "Tiradentes",
        "XV de Novembro",
    ];
    let name = names[rng.random_range(0..names.len())];
    format!("{} {}, {}", tipo, name, number)
}

/// Generate a postal code (CEP format: XXXXX-XXX).
pub fn postal_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{:05}-{:03}",
        rng.random_range(10000..99999),
        rng.random_range(0..999)
    )
}

/// Generate a full address.
pub fn full_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let street = street_address(rng);
    let city_name = city(rng);
    let (_, state_abbr) = state(rng);
    let cep = postal_code(rng);
    format!("{} - {} - {} - CEP {}", street, city_name, state_abbr, cep)
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
        assert!(phone_num.starts_with('('));
    }

    #[test]
    fn test_postal_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let code = postal_code(&mut rng);
        assert!(code.contains('-'));
        assert_eq!(code.len(), 9);
    }
}
