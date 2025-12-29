//! Company name generation.

use rand::Rng;

/// Company name suffixes
pub const COMPANY_SUFFIXES: &[&str] = &[
    "Inc",
    "LLC",
    "Ltd",
    "Corp",
    "Corporation",
    "Co",
    "Company",
    "Group",
    "Holdings",
    "Industries",
    "Enterprises",
    "Solutions",
    "Partners",
    "Associates",
    "International",
    "Worldwide",
    "Global",
];

/// Industries
pub const INDUSTRIES: &[&str] = &[
    "Technology",
    "Healthcare",
    "Finance",
    "Manufacturing",
    "Retail",
    "Telecommunications",
    "Energy",
    "Transportation",
    "Construction",
    "Education",
    "Entertainment",
    "Real Estate",
    "Agriculture",
    "Aerospace",
    "Automotive",
    "Biotechnology",
    "Chemicals",
    "Defense",
    "Electronics",
    "Food & Beverage",
    "Hospitality",
    "Insurance",
    "Legal",
    "Logistics",
    "Media",
    "Mining",
    "Pharmaceuticals",
    "Publishing",
    "Software",
    "Tourism",
];

/// Name parts for company name generation
const NAME_PREFIXES: &[&str] = &[
    "Alpha", "Beta", "Cyber", "Data", "Echo", "First", "Global", "Hyper", "Info", "Jet", "Krypton",
    "Logic", "Mega", "Neo", "Omega", "Prime", "Quantum", "Rapid", "Synergy", "Tech", "Ultra",
    "Vertex", "Wave", "Xcel", "Zenith",
];

const NAME_SUFFIXES: &[&str] = &[
    "Corp",
    "Dynamics",
    "Edge",
    "Force",
    "Genesis",
    "Hub",
    "Innovations",
    "Junction",
    "Kinetics",
    "Labs",
    "Matrix",
    "Net",
    "Ops",
    "Platform",
    "Quest",
    "Research",
    "Systems",
    "Tech",
    "United",
    "Ventures",
    "Works",
    "Zone",
];

/// Business buzzwords for catch phrases
const BUZZWORDS_ADJECTIVES: &[&str] = &[
    "Adaptive",
    "Advanced",
    "Automated",
    "Balanced",
    "Centralized",
    "Configurable",
    "Cross-platform",
    "Customizable",
    "Decentralized",
    "Digitized",
    "Distributed",
    "Enhanced",
    "Enterprise-wide",
    "Focused",
    "Horizontal",
    "Implemented",
    "Innovative",
    "Integrated",
    "Intuitive",
    "Managed",
    "Mandatory",
    "Multi-layered",
    "Networked",
    "Object-based",
    "Open-source",
    "Organic",
    "Persistent",
    "Proactive",
    "Progressive",
    "Reactive",
    "Realigned",
    "Re-engineered",
    "Robust",
    "Seamless",
    "Secured",
    "Streamlined",
    "Switchable",
    "Synchronized",
    "Synergized",
    "Total",
    "Triple-buffered",
    "Universal",
    "User-friendly",
    "Versatile",
    "Virtual",
];

const BUZZWORDS_NOUNS: &[&str] = &[
    "ability",
    "access",
    "adapter",
    "algorithm",
    "alliance",
    "analyzer",
    "application",
    "approach",
    "architecture",
    "array",
    "attitude",
    "benchmark",
    "budgetary management",
    "capability",
    "capacity",
    "challenge",
    "circuit",
    "collaboration",
    "complexity",
    "concept",
    "conglomeration",
    "contingency",
    "core",
    "customer loyalty",
    "data-warehouse",
    "database",
    "definition",
    "emulation",
    "encoding",
    "encryption",
    "extranet",
    "firmware",
    "flexibility",
    "forecast",
    "framework",
    "function",
    "functionalities",
    "graphic interface",
    "hardware",
    "help-desk",
    "hierarchy",
    "hub",
    "implementation",
    "infrastructure",
    "initiative",
    "installation",
    "instruction set",
    "interface",
    "intranet",
    "knowledge base",
    "matrices",
];

/// Generate a company name.
///
/// # Example
/// ```
/// use dx_datagen::commerce::company_name;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let name = company_name(&mut rng);
/// assert!(!name.is_empty());
/// ```
pub fn company_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = NAME_PREFIXES[rng.random_range(0..NAME_PREFIXES.len())];
    let suffix = NAME_SUFFIXES[rng.random_range(0..NAME_SUFFIXES.len())];
    format!("{} {}", prefix, suffix)
}

/// Get a random company suffix.
///
/// # Example
/// ```
/// use dx_datagen::commerce::company_suffix;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let suffix = company_suffix(&mut rng);
/// assert!(!suffix.is_empty());
/// ```
pub fn company_suffix<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    COMPANY_SUFFIXES[rng.random_range(0..COMPANY_SUFFIXES.len())]
}

/// Get a random industry.
///
/// # Example
/// ```
/// use dx_datagen::commerce::industry;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let ind = industry(&mut rng);
/// assert!(!ind.is_empty());
/// ```
pub fn industry<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    INDUSTRIES[rng.random_range(0..INDUSTRIES.len())]
}

/// Generate a company catch phrase.
///
/// # Example
/// ```
/// use dx_datagen::commerce::catch_phrase;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let phrase = catch_phrase(&mut rng);
/// assert!(!phrase.is_empty());
/// ```
pub fn catch_phrase<R: ?Sized + Rng>(rng: &mut R) -> String {
    let adj = BUZZWORDS_ADJECTIVES[rng.random_range(0..BUZZWORDS_ADJECTIVES.len())];
    let noun = BUZZWORDS_NOUNS[rng.random_range(0..BUZZWORDS_NOUNS.len())];
    format!("{} {}", adj, noun)
}

/// Generate company BS (buzzword statement).
pub fn company_bs<R: ?Sized + Rng>(rng: &mut R) -> String {
    let verbs = &[
        "leverage",
        "synergize",
        "optimize",
        "streamline",
        "transform",
        "empower",
        "revolutionize",
        "enhance",
        "maximize",
        "integrate",
    ];
    let verb = verbs[rng.random_range(0..verbs.len())];
    let adj = BUZZWORDS_ADJECTIVES[rng.random_range(0..BUZZWORDS_ADJECTIVES.len())];
    let noun = BUZZWORDS_NOUNS[rng.random_range(0..BUZZWORDS_NOUNS.len())];
    format!("{} {} {}", verb, adj, noun)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_company_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = company_name(&mut rng);
        assert!(!name.is_empty());
        assert!(name.contains(' '));
    }

    #[test]
    fn test_company_suffix() {
        let mut rng = StdRng::seed_from_u64(42);
        let suffix = company_suffix(&mut rng);
        assert!(COMPANY_SUFFIXES.contains(&suffix));
    }

    #[test]
    fn test_industry() {
        let mut rng = StdRng::seed_from_u64(42);
        let ind = industry(&mut rng);
        assert!(INDUSTRIES.contains(&ind));
    }

    #[test]
    fn test_catch_phrase() {
        let mut rng = StdRng::seed_from_u64(42);
        let phrase = catch_phrase(&mut rng);
        assert!(phrase.contains(' '));
    }

    #[test]
    fn test_determinism() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);

        assert_eq!(company_name(&mut rng1), company_name(&mut rng2));
    }
}
