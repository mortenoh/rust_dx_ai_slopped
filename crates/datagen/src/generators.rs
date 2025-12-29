//! Core random data generators.
//!
//! All functions support both concrete RNG types and trait objects (`dyn RngCore`).

use rand::seq::SliceRandom;
use rand::Rng;

/// Generate a random integer in the given range (inclusive).
pub fn int_range<R: ?Sized + Rng>(rng: &mut R, min: i64, max: i64) -> i64 {
    rng.random_range(min..=max)
}

/// Generate a random float in the given range (inclusive).
pub fn float_range<R: ?Sized + Rng>(rng: &mut R, min: f64, max: f64) -> f64 {
    rng.random_range(min..=max)
}

/// Generate a random boolean with the given probability of being true.
pub fn boolean<R: ?Sized + Rng>(rng: &mut R, probability: f64) -> bool {
    rng.random_bool(probability)
}

/// Generate a random alphanumeric string of the given length.
pub fn alphanumeric<R: ?Sized + Rng>(rng: &mut R, len: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    (0..len)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Generate a random hex string of the given length (each char is one hex digit).
pub fn hex_string<R: ?Sized + Rng>(rng: &mut R, len: usize) -> String {
    (0..len)
        .map(|_| format!("{:x}", rng.random::<u8>() & 0xf))
        .collect()
}

/// Generate a random hex string representing the given number of bytes.
pub fn hex_bytes<R: ?Sized + Rng>(rng: &mut R, bytes: usize) -> String {
    (0..bytes)
        .map(|_| format!("{:02x}", rng.random::<u8>()))
        .collect()
}

/// Pick a random element from a slice.
pub fn pick_one<'a, R: ?Sized + Rng, T>(rng: &mut R, items: &'a [T]) -> &'a T {
    let idx = rng.random_range(0..items.len());
    &items[idx]
}

/// Shuffle a slice in place.
pub fn shuffle<R: ?Sized + Rng, T>(rng: &mut R, items: &mut [T]) {
    items.shuffle(rng);
}

/// Generate a random value that may be null based on probability.
pub fn maybe_null<R: ?Sized + Rng, T, F>(rng: &mut R, null_prob: f64, generator: F) -> Option<T>
where
    F: FnOnce(&mut R) -> T,
{
    if null_prob > 0.0 && rng.random_bool(null_prob) {
        None
    } else {
        Some(generator(rng))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_int_range() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let v = int_range(&mut rng, 10, 20);
            assert!(v >= 10 && v <= 20);
        }
    }

    #[test]
    fn test_float_range() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let v = float_range(&mut rng, 0.0, 1.0);
            assert!(v >= 0.0 && v <= 1.0);
        }
    }

    #[test]
    fn test_alphanumeric() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = alphanumeric(&mut rng, 10);
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_hex_string() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = hex_string(&mut rng, 8);
        assert_eq!(s.len(), 8);
        assert!(s.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_pick_one() {
        let mut rng = StdRng::seed_from_u64(42);
        let items = vec!["a", "b", "c"];
        let picked = pick_one(&mut rng, &items);
        assert!(items.contains(picked));
    }

    #[test]
    fn test_maybe_null() {
        let mut rng = StdRng::seed_from_u64(42);

        // With 0 probability, should never be null
        for _ in 0..10 {
            let v = maybe_null(&mut rng, 0.0, |r| int_range(r, 1, 10));
            assert!(v.is_some());
        }

        // With 1.0 probability, should always be null
        for _ in 0..10 {
            let v = maybe_null(&mut rng, 1.0, |r| int_range(r, 1, 10));
            assert!(v.is_none());
        }
    }
}
