//! Astrology and zodiac data generators.
//!
//! Provides generators for astrology-related fake data including zodiac signs,
//! birthstones, planets, and horoscope elements.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Western zodiac signs.
static ZODIAC_SIGNS: &[&str] = &[
    "Aries",
    "Taurus",
    "Gemini",
    "Cancer",
    "Leo",
    "Virgo",
    "Libra",
    "Scorpio",
    "Sagittarius",
    "Capricorn",
    "Aquarius",
    "Pisces",
];

/// Zodiac sign date ranges.
static ZODIAC_DATES: &[(&str, &str)] = &[
    ("March 21", "April 19"),       // Aries
    ("April 20", "May 20"),         // Taurus
    ("May 21", "June 20"),          // Gemini
    ("June 21", "July 22"),         // Cancer
    ("July 23", "August 22"),       // Leo
    ("August 23", "September 22"),  // Virgo
    ("September 23", "October 22"), // Libra
    ("October 23", "November 21"),  // Scorpio
    ("November 22", "December 21"), // Sagittarius
    ("December 22", "January 19"),  // Capricorn
    ("January 20", "February 18"),  // Aquarius
    ("February 19", "March 20"),    // Pisces
];

/// Chinese zodiac animals.
static CHINESE_ZODIAC: &[&str] = &[
    "Rat", "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Goat", "Monkey", "Rooster", "Dog",
    "Pig",
];

/// Chinese zodiac elements.
static CHINESE_ELEMENTS: &[&str] = &["Wood", "Fire", "Earth", "Metal", "Water"];

/// Birthstones by month.
static BIRTHSTONES: &[&str] = &[
    "Garnet",     // January
    "Amethyst",   // February
    "Aquamarine", // March
    "Diamond",    // April
    "Emerald",    // May
    "Pearl",      // June
    "Ruby",       // July
    "Peridot",    // August
    "Sapphire",   // September
    "Opal",       // October
    "Topaz",      // November
    "Turquoise",  // December
];

/// Planets.
static PLANETS: &[&str] = &[
    "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
];

/// Astrological houses.
static HOUSES: &[&str] = &[
    "First House (Self)",
    "Second House (Value)",
    "Third House (Communication)",
    "Fourth House (Home)",
    "Fifth House (Pleasure)",
    "Sixth House (Health)",
    "Seventh House (Partnership)",
    "Eighth House (Transformation)",
    "Ninth House (Philosophy)",
    "Tenth House (Career)",
    "Eleventh House (Friendship)",
    "Twelfth House (Unconscious)",
];

/// Zodiac elements.
static ELEMENTS: &[&str] = &["Fire", "Earth", "Air", "Water"];

/// Zodiac modalities.
static MODALITIES: &[&str] = &["Cardinal", "Fixed", "Mutable"];

/// Moon phases.
static MOON_PHASES: &[&str] = &[
    "New Moon",
    "Waxing Crescent",
    "First Quarter",
    "Waxing Gibbous",
    "Full Moon",
    "Waning Gibbous",
    "Last Quarter",
    "Waning Crescent",
];

/// Generate a random zodiac sign.
pub fn zodiac_sign<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, ZODIAC_SIGNS)
}

/// Generate zodiac sign with date range.
pub fn zodiac_sign_with_dates<R: ?Sized + Rng>(rng: &mut R) -> (String, String, String) {
    let idx = rng.random_range(0..ZODIAC_SIGNS.len());
    let (start, end) = ZODIAC_DATES[idx];
    (
        ZODIAC_SIGNS[idx].to_string(),
        start.to_string(),
        end.to_string(),
    )
}

/// Generate a random Chinese zodiac animal.
pub fn chinese_zodiac<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, CHINESE_ZODIAC)
}

/// Generate a random Chinese zodiac element.
pub fn chinese_element<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, CHINESE_ELEMENTS)
}

/// Generate a full Chinese zodiac sign (element + animal).
pub fn chinese_zodiac_full<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} {}", chinese_element(rng), chinese_zodiac(rng))
}

/// Generate a random birthstone.
pub fn birthstone<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, BIRTHSTONES)
}

/// Get birthstone for a specific month (1-12).
pub fn birthstone_for_month(month: u8) -> Option<&'static str> {
    if month >= 1 && month <= 12 {
        Some(BIRTHSTONES[(month - 1) as usize])
    } else {
        None
    }
}

/// Generate a random planet.
pub fn planet<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, PLANETS)
}

/// Generate a random astrological house.
pub fn astrological_house<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, HOUSES)
}

/// Generate a random zodiac element.
pub fn zodiac_element<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, ELEMENTS)
}

/// Generate a random zodiac modality.
pub fn zodiac_modality<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, MODALITIES)
}

/// Generate a random moon phase.
pub fn moon_phase<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, MOON_PHASES)
}

/// Get the element for a specific zodiac sign.
pub fn element_for_sign(sign: &str) -> Option<&'static str> {
    match sign {
        "Aries" | "Leo" | "Sagittarius" => Some("Fire"),
        "Taurus" | "Virgo" | "Capricorn" => Some("Earth"),
        "Gemini" | "Libra" | "Aquarius" => Some("Air"),
        "Cancer" | "Scorpio" | "Pisces" => Some("Water"),
        _ => None,
    }
}

/// Get the modality for a specific zodiac sign.
pub fn modality_for_sign(sign: &str) -> Option<&'static str> {
    match sign {
        "Aries" | "Cancer" | "Libra" | "Capricorn" => Some("Cardinal"),
        "Taurus" | "Leo" | "Scorpio" | "Aquarius" => Some("Fixed"),
        "Gemini" | "Virgo" | "Sagittarius" | "Pisces" => Some("Mutable"),
        _ => None,
    }
}

/// Generate a simple horoscope reading.
pub fn horoscope<R: ?Sized + Rng>(rng: &mut R) -> String {
    let openings = [
        "Today brings",
        "The stars suggest",
        "You may find",
        "This is a good time for",
        "Pay attention to",
        "Focus on",
    ];
    let themes = [
        "new opportunities in your career",
        "a chance for personal growth",
        "unexpected connections",
        "financial decisions",
        "matters of the heart",
        "creative pursuits",
        "family relationships",
        "self-reflection",
        "adventure and exploration",
        "practical matters",
    ];
    let advice = [
        "Trust your instincts.",
        "Be patient with yourself.",
        "Take time to rest.",
        "Embrace change.",
        "Communicate openly.",
        "Stay positive.",
        "Listen to others.",
        "Follow your heart.",
    ];

    format!(
        "{} {}. {}",
        pick(rng, &openings),
        pick(rng, &themes),
        pick(rng, &advice)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_zodiac_sign() {
        let mut rng = StdRng::seed_from_u64(42);
        let z = zodiac_sign(&mut rng);
        assert!(ZODIAC_SIGNS.contains(&z));
    }

    #[test]
    fn test_chinese_zodiac() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = chinese_zodiac(&mut rng);
        assert!(CHINESE_ZODIAC.contains(&c));
    }

    #[test]
    fn test_birthstone() {
        let mut rng = StdRng::seed_from_u64(42);
        let b = birthstone(&mut rng);
        assert!(BIRTHSTONES.contains(&b));
    }

    #[test]
    fn test_birthstone_for_month() {
        assert_eq!(birthstone_for_month(1), Some("Garnet"));
        assert_eq!(birthstone_for_month(7), Some("Ruby"));
        assert_eq!(birthstone_for_month(12), Some("Turquoise"));
        assert_eq!(birthstone_for_month(13), None);
        assert_eq!(birthstone_for_month(0), None);
    }

    #[test]
    fn test_element_for_sign() {
        assert_eq!(element_for_sign("Aries"), Some("Fire"));
        assert_eq!(element_for_sign("Taurus"), Some("Earth"));
        assert_eq!(element_for_sign("Gemini"), Some("Air"));
        assert_eq!(element_for_sign("Cancer"), Some("Water"));
        assert_eq!(element_for_sign("Unknown"), None);
    }

    #[test]
    fn test_horoscope() {
        let mut rng = StdRng::seed_from_u64(42);
        let h = horoscope(&mut rng);
        assert!(!h.is_empty());
    }

    #[test]
    fn test_chinese_zodiac_full() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = chinese_zodiac_full(&mut rng);
        assert!(c.contains(' '));
    }
}
