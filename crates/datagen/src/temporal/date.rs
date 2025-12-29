//! Date generation.
//!
//! Generate random dates within various ranges.
//!
//! # Example
//!
//! ```
//! use dx_datagen::temporal::date::{date_between, date_past, date_future};
//! use chrono::NaiveDate;
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
//! let end = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
//! let d = date_between(&mut rng, start, end);
//! ```

use chrono::{Datelike, Days, NaiveDate, Utc};
use rand::Rng;

/// Generate a random date between two dates (inclusive).
pub fn date_between<R: ?Sized + Rng>(rng: &mut R, start: NaiveDate, end: NaiveDate) -> NaiveDate {
    let start_days = start.num_days_from_ce();
    let end_days = end.num_days_from_ce();

    if start_days >= end_days {
        return start;
    }

    let random_days = rng.random_range(start_days..=end_days);
    NaiveDate::from_num_days_from_ce_opt(random_days).unwrap_or(start)
}

/// Generate a random date in the past (within the last N days).
pub fn date_past<R: ?Sized + Rng>(rng: &mut R, days_ago: u64) -> NaiveDate {
    let today = Utc::now().date_naive();
    let past = today.checked_sub_days(Days::new(days_ago)).unwrap_or(today);
    date_between(rng, past, today)
}

/// Generate a random date in the future (within the next N days).
pub fn date_future<R: ?Sized + Rng>(rng: &mut R, days_ahead: u64) -> NaiveDate {
    let today = Utc::now().date_naive();
    let future = today
        .checked_add_days(Days::new(days_ahead))
        .unwrap_or(today);
    date_between(rng, today, future)
}

/// Generate a recent date (within the last 7 days).
pub fn date_recent<R: ?Sized + Rng>(rng: &mut R) -> NaiveDate {
    date_past(rng, 7)
}

/// Generate a date within a year range.
pub fn date_in_year<R: ?Sized + Rng>(rng: &mut R, year: i32) -> NaiveDate {
    let start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
    date_between(rng, start, end)
}

/// Generate a birthdate for a person of given age.
pub fn birth_date<R: ?Sized + Rng>(rng: &mut R, min_age: u32, max_age: u32) -> NaiveDate {
    let today = Utc::now().date_naive();
    let min_date = today
        .checked_sub_days(Days::new(max_age as u64 * 365))
        .unwrap_or(today);
    let max_date = today
        .checked_sub_days(Days::new(min_age as u64 * 365))
        .unwrap_or(today);
    date_between(rng, min_date, max_date)
}

/// Generate a random month (1-12).
pub fn month<R: ?Sized + Rng>(rng: &mut R) -> u32 {
    rng.random_range(1..=12)
}

/// Generate a random day of month (1-28, safe for all months).
pub fn day_of_month<R: ?Sized + Rng>(rng: &mut R) -> u32 {
    rng.random_range(1..=28)
}

/// Generate a random weekday (1-7, where 1=Monday, 7=Sunday).
pub fn weekday<R: ?Sized + Rng>(rng: &mut R) -> u32 {
    rng.random_range(1..=7)
}

/// Format a date as ISO 8601 string (YYYY-MM-DD).
pub fn format_iso(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Format a date in US format (MM/DD/YYYY).
pub fn format_us(date: NaiveDate) -> String {
    date.format("%m/%d/%Y").to_string()
}

/// Format a date in European format (DD/MM/YYYY).
pub fn format_eu(date: NaiveDate) -> String {
    date.format("%d/%m/%Y").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_date_between() {
        let mut rng = StdRng::seed_from_u64(42);
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap();
        let d = date_between(&mut rng, start, end);
        assert!(d >= start && d <= end);
    }

    #[test]
    fn test_date_past() {
        let mut rng = StdRng::seed_from_u64(42);
        let today = Utc::now().date_naive();
        let d = date_past(&mut rng, 30);
        assert!(d <= today);
    }

    #[test]
    fn test_date_future() {
        let mut rng = StdRng::seed_from_u64(42);
        let today = Utc::now().date_naive();
        let d = date_future(&mut rng, 30);
        assert!(d >= today);
    }

    #[test]
    fn test_date_in_year() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = date_in_year(&mut rng, 2023);
        assert_eq!(d.year(), 2023);
    }

    #[test]
    fn test_birth_date() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = birth_date(&mut rng, 18, 65);
        let today = Utc::now().date_naive();
        let age_days = (today - d).num_days();
        let age_years = age_days / 365;
        assert!(age_years >= 18 && age_years <= 65);
    }

    #[test]
    fn test_format_iso() {
        let d = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
        assert_eq!(format_iso(d), "2023-06-15");
    }

    #[test]
    fn test_format_us() {
        let d = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
        assert_eq!(format_us(d), "06/15/2023");
    }

    #[test]
    fn test_format_eu() {
        let d = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
        assert_eq!(format_eu(d), "15/06/2023");
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap();

        assert_eq!(
            date_between(&mut rng1, start, end),
            date_between(&mut rng2, start, end)
        );
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap();
        let d = date_between(&mut *rng, start, end);
        assert!(d >= start && d <= end);
    }
}
