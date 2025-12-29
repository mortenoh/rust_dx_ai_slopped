//! Finance and cryptocurrency data generation.
//!
//! Generate cryptocurrency addresses, bank identifiers, and transaction data.

use rand::Rng;

/// Bitcoin address prefixes (for realistic-looking addresses)
const BTC_PREFIXES: &[&str] = &["1", "3", "bc1q"];

/// Transaction types
pub const TRANSACTION_TYPES: &[&str] = &[
    "deposit",
    "withdrawal",
    "transfer",
    "payment",
    "refund",
    "purchase",
    "sale",
    "fee",
    "dividend",
    "interest",
];

/// Generate a Bitcoin-like address.
///
/// Note: These are realistic-looking but not valid Bitcoin addresses.
///
/// # Example
/// ```
/// use dx_datagen::numeric::finance::bitcoin_address;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let addr = bitcoin_address(&mut rng);
/// assert!(addr.starts_with('1') || addr.starts_with('3') || addr.starts_with("bc1q"));
/// ```
pub fn bitcoin_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = BTC_PREFIXES[rng.random_range(0..BTC_PREFIXES.len())];

    // Bitcoin addresses use Base58 (no 0, O, I, l)
    const BASE58: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    let length = match prefix {
        "bc1q" => 39, // Bech32 addresses are 42 chars total
        "1" => 33,    // P2PKH addresses are 34 chars total
        "3" => 33,    // P2SH addresses are 34 chars total
        _ => 33,
    };

    let mut result = String::from(prefix);
    for _ in 0..length {
        let idx = rng.random_range(0..BASE58.len());
        result.push(BASE58[idx] as char);
    }

    result
}

/// Generate an Ethereum-like address.
///
/// Note: These are realistic-looking but not valid Ethereum addresses.
///
/// # Example
/// ```
/// use dx_datagen::numeric::finance::ethereum_address;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let addr = ethereum_address(&mut rng);
/// assert!(addr.starts_with("0x"));
/// assert_eq!(addr.len(), 42);
/// ```
pub fn ethereum_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let mut result = String::from("0x");
    const HEX: &[u8] = b"0123456789abcdef";

    for _ in 0..40 {
        let idx = rng.random_range(0..HEX.len());
        result.push(HEX[idx] as char);
    }

    result
}

/// Generate a routing number (9 digits, US bank format).
///
/// Note: These are realistic-looking but not valid routing numbers.
///
/// # Example
/// ```
/// use dx_datagen::numeric::finance::routing_number;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let num = routing_number(&mut rng);
/// assert_eq!(num.len(), 9);
/// ```
pub fn routing_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    // First two digits indicate Federal Reserve district (01-12)
    let district: u32 = rng.random_range(1..=12);
    let rest: u32 = rng.random_range(0..10_000_000);
    format!("{:02}{:07}", district, rest)
}

/// Generate a bank account number.
///
/// # Example
/// ```
/// use dx_datagen::numeric::finance::account_number;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let num = account_number(&mut rng);
/// assert!(num.len() >= 8 && num.len() <= 17);
/// ```
pub fn account_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    let length = rng.random_range(8..=17);
    let mut result = String::with_capacity(length);

    // First digit shouldn't be 0
    result.push((b'1' + rng.random_range(0..9u8)) as char);

    for _ in 1..length {
        result.push((b'0' + rng.random_range(0..10u8)) as char);
    }

    result
}

/// Generate a SWIFT/BIC code.
///
/// Format: AAAA BB CC DDD (bank code, country, location, branch)
///
/// # Example
/// ```
/// use dx_datagen::numeric::finance::swift_code;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let code = swift_code(&mut rng);
/// assert!(code.len() == 8 || code.len() == 11);
/// ```
pub fn swift_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    let mut result = String::with_capacity(11);

    // Bank code (4 letters)
    for _ in 0..4 {
        result.push((b'A' + rng.random_range(0..26u8)) as char);
    }

    // Country code (2 letters)
    const COUNTRIES: &[&str] = &["US", "GB", "DE", "FR", "CH", "JP", "CN", "AU", "CA", "NL"];
    result.push_str(COUNTRIES[rng.random_range(0..COUNTRIES.len())]);

    // Location code (2 alphanumeric)
    for _ in 0..2 {
        if rng.random_bool(0.5) {
            result.push((b'A' + rng.random_range(0..26u8)) as char);
        } else {
            result.push((b'0' + rng.random_range(0..10u8)) as char);
        }
    }

    // Optional branch code (3 alphanumeric) - 50% chance
    if rng.random_bool(0.5) {
        for _ in 0..3 {
            if rng.random_bool(0.5) {
                result.push((b'A' + rng.random_range(0..26u8)) as char);
            } else {
                result.push((b'0' + rng.random_range(0..10u8)) as char);
            }
        }
    }

    result
}

/// Generate a BIC code (alias for swift_code).
pub fn bic<R: ?Sized + Rng>(rng: &mut R) -> String {
    swift_code(rng)
}

/// Get a random transaction type.
///
/// # Example
/// ```
/// use dx_datagen::numeric::finance::transaction_type;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let t = transaction_type(&mut rng);
/// assert!(!t.is_empty());
/// ```
pub fn transaction_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    TRANSACTION_TYPES[rng.random_range(0..TRANSACTION_TYPES.len())]
}

/// Generate a transaction description.
///
/// # Example
/// ```
/// use dx_datagen::numeric::finance::transaction_description;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let desc = transaction_description(&mut rng);
/// assert!(!desc.is_empty());
/// ```
pub fn transaction_description<R: ?Sized + Rng>(rng: &mut R) -> String {
    const MERCHANTS: &[&str] = &[
        "Amazon",
        "Walmart",
        "Target",
        "Costco",
        "Starbucks",
        "Netflix",
        "Spotify",
        "Apple",
        "Google",
        "Microsoft",
        "Uber",
        "Lyft",
        "DoorDash",
        "Grubhub",
        "PayPal",
    ];

    const ACTIONS: &[&str] = &[
        "Purchase at",
        "Payment to",
        "Transfer to",
        "Refund from",
        "Subscription",
        "Order from",
        "Transaction with",
    ];

    let action = ACTIONS[rng.random_range(0..ACTIONS.len())];
    let merchant = MERCHANTS[rng.random_range(0..MERCHANTS.len())];
    let ref_num: u32 = rng.random_range(100000..999999);

    format!("{} {} #{}", action, merchant, ref_num)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_bitcoin_address() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = bitcoin_address(&mut rng);
        assert!(addr.starts_with('1') || addr.starts_with('3') || addr.starts_with("bc1q"));
        assert!(addr.len() >= 34);
    }

    #[test]
    fn test_ethereum_address() {
        let mut rng = StdRng::seed_from_u64(42);
        let addr = ethereum_address(&mut rng);
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 42);
        // Should only contain hex chars after 0x
        assert!(addr[2..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_routing_number() {
        let mut rng = StdRng::seed_from_u64(42);
        let num = routing_number(&mut rng);
        assert_eq!(num.len(), 9);
        assert!(num.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_account_number() {
        let mut rng = StdRng::seed_from_u64(42);
        let num = account_number(&mut rng);
        assert!(num.len() >= 8 && num.len() <= 17);
        assert!(num.chars().all(|c| c.is_ascii_digit()));
        assert!(!num.starts_with('0'));
    }

    #[test]
    fn test_swift_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let code = swift_code(&mut rng);
        assert!(code.len() == 8 || code.len() == 11);
    }

    #[test]
    fn test_transaction_type() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = transaction_type(&mut rng);
        assert!(TRANSACTION_TYPES.contains(&t));
    }

    #[test]
    fn test_transaction_description() {
        let mut rng = StdRng::seed_from_u64(42);
        let desc = transaction_description(&mut rng);
        assert!(!desc.is_empty());
        assert!(desc.contains('#'));
    }

    #[test]
    fn test_determinism() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);

        assert_eq!(bitcoin_address(&mut rng1), bitcoin_address(&mut rng2));
        assert_eq!(ethereum_address(&mut rng1), ethereum_address(&mut rng2));
    }

    #[test]
    fn test_trait_object() {
        use rand::RngCore;
        let mut rng: Box<dyn RngCore> = Box::new(StdRng::seed_from_u64(42));
        let addr = ethereum_address(&mut *rng);
        assert!(addr.starts_with("0x"));
    }
}
