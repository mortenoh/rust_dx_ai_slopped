//! DateTime generation.
//!
//! Generate random datetimes with various formats.
//!
//! # Example
//!
//! ```
//! use dx_datagen::temporal::datetime::{datetime_between, timestamp_unix, iso8601};
//! use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let start = NaiveDateTime::new(
//!     NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
//!     NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
//! );
//! let end = NaiveDateTime::new(
//!     NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
//!     NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
//! );
//! let dt = datetime_between(&mut rng, start, end);
//! ```

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rand::Rng;

/// Generate a random datetime between two datetimes.
pub fn datetime_between<R: ?Sized + Rng>(
    rng: &mut R,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> NaiveDateTime {
    let start_ts = start.and_utc().timestamp();
    let end_ts = end.and_utc().timestamp();

    if start_ts >= end_ts {
        return start;
    }

    let random_ts = rng.random_range(start_ts..=end_ts);
    DateTime::from_timestamp(random_ts, 0)
        .map(|dt| dt.naive_utc())
        .unwrap_or(start)
}

/// Generate a random datetime in the past (within last N days).
pub fn datetime_past<R: ?Sized + Rng>(rng: &mut R, days_ago: i64) -> NaiveDateTime {
    let now = Utc::now().naive_utc();
    let past = now - Duration::days(days_ago);
    datetime_between(rng, past, now)
}

/// Generate a random datetime in the future (within next N days).
pub fn datetime_future<R: ?Sized + Rng>(rng: &mut R, days_ahead: i64) -> NaiveDateTime {
    let now = Utc::now().naive_utc();
    let future = now + Duration::days(days_ahead);
    datetime_between(rng, now, future)
}

/// Generate a recent datetime (within last 7 days).
pub fn datetime_recent<R: ?Sized + Rng>(rng: &mut R) -> NaiveDateTime {
    datetime_past(rng, 7)
}

/// Generate a random unix timestamp (seconds since epoch).
pub fn timestamp_unix<R: ?Sized + Rng>(rng: &mut R) -> i64 {
    // Range: 2000-01-01 to 2030-12-31
    let start = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    );
    let end = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2030, 12, 31).unwrap(),
        NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
    );

    datetime_between(rng, start, end).and_utc().timestamp()
}

/// Generate a unix timestamp in milliseconds.
pub fn timestamp_unix_ms<R: ?Sized + Rng>(rng: &mut R) -> i64 {
    timestamp_unix(rng) * 1000 + rng.random_range(0..1000)
}

/// Generate an ISO 8601 formatted datetime string.
pub fn iso8601<R: ?Sized + Rng>(rng: &mut R) -> String {
    let dt = datetime_past(rng, 365);
    dt.format("%Y-%m-%dT%H:%M:%S").to_string()
}

/// Generate an ISO 8601 datetime with timezone indicator.
pub fn iso8601_utc<R: ?Sized + Rng>(rng: &mut R) -> String {
    let dt = datetime_past(rng, 365);
    format!("{}Z", dt.format("%Y-%m-%dT%H:%M:%S"))
}

/// Generate an RFC 2822 formatted datetime string.
pub fn rfc2822<R: ?Sized + Rng>(rng: &mut R) -> String {
    let dt = datetime_past(rng, 365);
    // Simple RFC 2822 approximation
    dt.format("%a, %d %b %Y %H:%M:%S +0000").to_string()
}

/// Format a datetime in a custom format.
pub fn format_datetime(dt: NaiveDateTime, format: &str) -> String {
    dt.format(format).to_string()
}

/// Generate a datetime within a specific year.
pub fn datetime_in_year<R: ?Sized + Rng>(rng: &mut R, year: i32) -> NaiveDateTime {
    let start = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    );
    let end = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(year, 12, 31).unwrap(),
        NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
    );
    datetime_between(rng, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_datetime_between() {
        let mut rng = StdRng::seed_from_u64(42);
        let start = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );
        let end = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2020, 12, 31).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );
        let dt = datetime_between(&mut rng, start, end);
        assert!(dt >= start && dt <= end);
    }

    #[test]
    fn test_datetime_past() {
        let mut rng = StdRng::seed_from_u64(42);
        let now = Utc::now().naive_utc();
        let dt = datetime_past(&mut rng, 30);
        assert!(dt <= now);
    }

    #[test]
    fn test_datetime_future() {
        let mut rng = StdRng::seed_from_u64(42);
        let now = Utc::now().naive_utc();
        let dt = datetime_future(&mut rng, 30);
        assert!(dt >= now);
    }

    #[test]
    fn test_timestamp_unix() {
        let mut rng = StdRng::seed_from_u64(42);
        let ts = timestamp_unix(&mut rng);
        assert!(ts > 946684800); // After 2000-01-01
        assert!(ts < 1924991999); // Before 2030-12-31
    }

    #[test]
    fn test_timestamp_unix_ms() {
        let mut rng = StdRng::seed_from_u64(42);
        let ts_ms = timestamp_unix_ms(&mut rng);
        assert!(ts_ms > 946684800000);
    }

    #[test]
    fn test_iso8601() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = iso8601(&mut rng);
        assert!(s.contains("T"));
        assert!(s.len() == 19);
    }

    #[test]
    fn test_iso8601_utc() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = iso8601_utc(&mut rng);
        assert!(s.ends_with("Z"));
    }

    #[test]
    fn test_datetime_in_year() {
        let mut rng = StdRng::seed_from_u64(42);
        let dt = datetime_in_year(&mut rng, 2023);
        assert_eq!(dt.date().year(), 2023);
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(timestamp_unix(&mut rng1), timestamp_unix(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let ts = timestamp_unix(&mut *rng);
        assert!(ts > 0);
    }
}
