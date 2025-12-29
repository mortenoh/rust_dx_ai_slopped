//! Currency data generation.

use rand::Rng;

/// Currency information (code, name, symbol)
pub const CURRENCIES: &[(&str, &str, &str)] = &[
    ("USD", "US Dollar", "$"),
    ("EUR", "Euro", "€"),
    ("GBP", "British Pound Sterling", "£"),
    ("JPY", "Japanese Yen", "¥"),
    ("CNY", "Chinese Yuan", "¥"),
    ("CHF", "Swiss Franc", "CHF"),
    ("CAD", "Canadian Dollar", "CA$"),
    ("AUD", "Australian Dollar", "A$"),
    ("NZD", "New Zealand Dollar", "NZ$"),
    ("HKD", "Hong Kong Dollar", "HK$"),
    ("SGD", "Singapore Dollar", "S$"),
    ("SEK", "Swedish Krona", "kr"),
    ("NOK", "Norwegian Krone", "kr"),
    ("DKK", "Danish Krone", "kr"),
    ("KRW", "South Korean Won", "₩"),
    ("INR", "Indian Rupee", "₹"),
    ("RUB", "Russian Ruble", "₽"),
    ("BRL", "Brazilian Real", "R$"),
    ("MXN", "Mexican Peso", "MX$"),
    ("ZAR", "South African Rand", "R"),
    ("TRY", "Turkish Lira", "₺"),
    ("PLN", "Polish Zloty", "zł"),
    ("THB", "Thai Baht", "฿"),
    ("IDR", "Indonesian Rupiah", "Rp"),
    ("MYR", "Malaysian Ringgit", "RM"),
    ("PHP", "Philippine Peso", "₱"),
    ("CZK", "Czech Koruna", "Kč"),
    ("ILS", "Israeli New Shekel", "₪"),
    ("CLP", "Chilean Peso", "CLP$"),
    ("AED", "UAE Dirham", "د.إ"),
];

/// Get a random currency code.
///
/// # Example
/// ```
/// use dx_datagen::commerce::currency_code;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let code = currency_code(&mut rng);
/// assert_eq!(code.len(), 3);
/// ```
pub fn currency_code<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..CURRENCIES.len());
    CURRENCIES[idx].0
}

/// Get a random currency name.
///
/// # Example
/// ```
/// use dx_datagen::commerce::currency_name;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let name = currency_name(&mut rng);
/// assert!(!name.is_empty());
/// ```
pub fn currency_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..CURRENCIES.len());
    CURRENCIES[idx].1
}

/// Get a random currency symbol.
///
/// # Example
/// ```
/// use dx_datagen::commerce::currency_symbol;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let symbol = currency_symbol(&mut rng);
/// assert!(!symbol.is_empty());
/// ```
pub fn currency_symbol<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..CURRENCIES.len());
    CURRENCIES[idx].2
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_currency_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let code = currency_code(&mut rng);
        assert_eq!(code.len(), 3);
        assert!(CURRENCIES.iter().any(|(c, _, _)| *c == code));
    }

    #[test]
    fn test_currency_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = currency_name(&mut rng);
        assert!(CURRENCIES.iter().any(|(_, n, _)| *n == name));
    }

    #[test]
    fn test_currency_symbol() {
        let mut rng = StdRng::seed_from_u64(42);
        let symbol = currency_symbol(&mut rng);
        assert!(CURRENCIES.iter().any(|(_, _, s)| *s == symbol));
    }

    #[test]
    fn test_determinism() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);

        assert_eq!(currency_code(&mut rng1), currency_code(&mut rng2));
    }
}
