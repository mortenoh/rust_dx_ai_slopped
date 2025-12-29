//! Government and civic data generators.
//!
//! Provides generators for government-related fake data including agencies,
//! identification documents, and political data.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Government agencies (US).
static US_AGENCIES: &[&str] = &[
    "Department of State",
    "Department of the Treasury",
    "Department of Defense",
    "Department of Justice",
    "Department of the Interior",
    "Department of Agriculture",
    "Department of Commerce",
    "Department of Labor",
    "Department of Health and Human Services",
    "Department of Housing and Urban Development",
    "Department of Transportation",
    "Department of Energy",
    "Department of Education",
    "Department of Veterans Affairs",
    "Department of Homeland Security",
    "Environmental Protection Agency",
    "National Aeronautics and Space Administration",
    "Federal Bureau of Investigation",
    "Central Intelligence Agency",
    "National Security Agency",
];

/// Political parties (US).
static US_PARTIES: &[&str] = &[
    "Democratic Party",
    "Republican Party",
    "Libertarian Party",
    "Green Party",
    "Independent",
];

/// Government positions.
static GOVERNMENT_POSITIONS: &[&str] = &[
    "Senator",
    "Representative",
    "Governor",
    "Mayor",
    "City Council Member",
    "County Commissioner",
    "State Attorney",
    "District Attorney",
    "Sheriff",
    "Chief of Police",
    "Fire Chief",
    "City Manager",
    "Budget Director",
    "Public Works Director",
    "Planning Commissioner",
];

/// Document types.
static DOCUMENT_TYPES: &[&str] = &[
    "Passport",
    "Driver's License",
    "State ID",
    "Social Security Card",
    "Birth Certificate",
    "Marriage Certificate",
    "Voter Registration Card",
    "Work Permit",
    "Green Card",
    "Visa",
];

/// US states for license plate generation.
static US_STATES: &[&str] = &[
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS",
    "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY",
    "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV",
    "WI", "WY",
];

/// Generate a random US government agency.
pub fn us_agency<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, US_AGENCIES)
}

/// Generate a random political party.
pub fn political_party<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, US_PARTIES)
}

/// Generate a random government position.
pub fn government_position<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, GOVERNMENT_POSITIONS)
}

/// Generate a random document type.
pub fn document_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, DOCUMENT_TYPES)
}

/// Generate a random passport number.
pub fn passport_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    // US passport format: 9 digits
    format!("{:09}", rng.random_range(100000000u32..999999999u32))
}

/// Generate a random driver's license number.
pub fn drivers_license<R: ?Sized + Rng>(rng: &mut R) -> String {
    // Generic format: 1 letter + 12 digits
    let letter = (b'A' + rng.random_range(0..26)) as char;
    let digits: String = (0..12)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect();
    format!("{}{}", letter, digits)
}

/// Generate a random state-specific driver's license.
pub fn drivers_license_state<R: ?Sized + Rng>(rng: &mut R) -> String {
    let state = pick(rng, US_STATES);
    let license = drivers_license(rng);
    format!("{}-{}", state, license)
}

/// Generate a random tax ID / EIN.
pub fn tax_id<R: ?Sized + Rng>(rng: &mut R) -> String {
    // EIN format: XX-XXXXXXX
    let first: u8 = rng.random_range(10..99);
    let second: u32 = rng.random_range(1000000..9999999);
    format!("{:02}-{:07}", first, second)
}

/// Generate a random voter registration number.
pub fn voter_registration_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    let state = pick(rng, US_STATES);
    let number: u32 = rng.random_range(10000000..99999999);
    format!("{}{:08}", state, number)
}

/// Generate a random permit number.
pub fn permit_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = ["PRM", "LIC", "CRT", "AUTH"][rng.random_range(0..4)];
    let year: u16 = rng.random_range(2020..2026);
    let number: u32 = rng.random_range(10000..99999);
    format!("{}-{}-{}", prefix, year, number)
}

/// Generate a random case number.
pub fn case_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    let year: u16 = rng.random_range(2020..2026);
    let court = ["CV", "CR", "FA", "PR", "JV"][rng.random_range(0..5)];
    let number: u32 = rng.random_range(1000..9999);
    format!("{}-{}-{:04}", year, court, number)
}

/// Generate a random bill number.
pub fn bill_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    let chamber = ["H.R.", "S.", "H.J.Res.", "S.J.Res."][rng.random_range(0..4)];
    let number: u16 = rng.random_range(1..9999);
    format!("{} {}", chamber, number)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_us_agency() {
        let mut rng = StdRng::seed_from_u64(42);
        let a = us_agency(&mut rng);
        assert!(US_AGENCIES.contains(&a));
    }

    #[test]
    fn test_political_party() {
        let mut rng = StdRng::seed_from_u64(42);
        let p = political_party(&mut rng);
        assert!(US_PARTIES.contains(&p));
    }

    #[test]
    fn test_passport_number() {
        let mut rng = StdRng::seed_from_u64(42);
        let p = passport_number(&mut rng);
        assert_eq!(p.len(), 9);
        assert!(p.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_drivers_license() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = drivers_license(&mut rng);
        assert_eq!(d.len(), 13);
        assert!(d.chars().next().unwrap().is_ascii_uppercase());
    }

    #[test]
    fn test_tax_id() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = tax_id(&mut rng);
        assert!(t.contains('-'));
    }

    #[test]
    fn test_case_number() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = case_number(&mut rng);
        assert!(c.contains('-'));
    }
}
