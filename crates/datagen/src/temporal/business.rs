//! Business day utilities.
//!
//! Generate business dates and check business day status.
//!
//! # Example
//!
//! ```
//! use dx_datagen::temporal::business::{business_date, is_business_day, next_business_day};
//! use chrono::NaiveDate;
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let bd = business_date(&mut rng, 30);
//! assert!(is_business_day(bd));
//! ```

use chrono::{Datelike, Days, NaiveDate, Utc, Weekday};
use rand::Rng;

/// Check if a date is a business day (Monday-Friday).
pub fn is_business_day(date: NaiveDate) -> bool {
    matches!(
        date.weekday(),
        Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri
    )
}

/// Check if a date is a weekend (Saturday or Sunday).
pub fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

/// Get the next business day from a given date.
pub fn next_business_day(date: NaiveDate) -> NaiveDate {
    let mut next = date.checked_add_days(Days::new(1)).unwrap();
    while !is_business_day(next) {
        next = next.checked_add_days(Days::new(1)).unwrap();
    }
    next
}

/// Get the previous business day from a given date.
pub fn prev_business_day(date: NaiveDate) -> NaiveDate {
    let mut prev = date.checked_sub_days(Days::new(1)).unwrap();
    while !is_business_day(prev) {
        prev = prev.checked_sub_days(Days::new(1)).unwrap();
    }
    prev
}

/// Generate a random business day within the last N days.
pub fn business_date<R: ?Sized + Rng>(rng: &mut R, days_ago: u64) -> NaiveDate {
    let today = Utc::now().date_naive();
    let start = today.checked_sub_days(Days::new(days_ago)).unwrap_or(today);

    // Collect business days in range
    let mut business_days: Vec<NaiveDate> = Vec::new();
    let mut current = start;
    while current <= today {
        if is_business_day(current) {
            business_days.push(current);
        }
        current = current.checked_add_days(Days::new(1)).unwrap();
    }

    if business_days.is_empty() {
        // Fallback to next business day
        next_business_day(start)
    } else {
        business_days[rng.random_range(0..business_days.len())]
    }
}

/// Generate a random business day within the next N days.
pub fn business_date_future<R: ?Sized + Rng>(rng: &mut R, days_ahead: u64) -> NaiveDate {
    let today = Utc::now().date_naive();
    let end = today
        .checked_add_days(Days::new(days_ahead))
        .unwrap_or(today);

    // Collect business days in range
    let mut business_days: Vec<NaiveDate> = Vec::new();
    let mut current = today;
    while current <= end {
        if is_business_day(current) {
            business_days.push(current);
        }
        current = current.checked_add_days(Days::new(1)).unwrap();
    }

    if business_days.is_empty() {
        next_business_day(today)
    } else {
        business_days[rng.random_range(0..business_days.len())]
    }
}

/// Add N business days to a date.
pub fn add_business_days(date: NaiveDate, days: u32) -> NaiveDate {
    let mut result = date;
    let mut added = 0;
    while added < days {
        result = result.checked_add_days(Days::new(1)).unwrap();
        if is_business_day(result) {
            added += 1;
        }
    }
    result
}

/// Subtract N business days from a date.
pub fn sub_business_days(date: NaiveDate, days: u32) -> NaiveDate {
    let mut result = date;
    let mut subtracted = 0;
    while subtracted < days {
        result = result.checked_sub_days(Days::new(1)).unwrap();
        if is_business_day(result) {
            subtracted += 1;
        }
    }
    result
}

/// Count business days between two dates (exclusive of end date).
pub fn business_days_between(start: NaiveDate, end: NaiveDate) -> u32 {
    if start >= end {
        return 0;
    }

    let mut count = 0;
    let mut current = start;
    while current < end {
        if is_business_day(current) {
            count += 1;
        }
        current = current.checked_add_days(Days::new(1)).unwrap();
    }
    count
}

/// Get the weekday name.
pub fn weekday_name(date: NaiveDate) -> &'static str {
    match date.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_is_business_day() {
        let monday = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(); // Monday
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).unwrap(); // Saturday

        assert!(is_business_day(monday));
        assert!(!is_business_day(saturday));
    }

    #[test]
    fn test_is_weekend() {
        let saturday = NaiveDate::from_ymd_opt(2024, 1, 6).unwrap();
        let sunday = NaiveDate::from_ymd_opt(2024, 1, 7).unwrap();
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();

        assert!(is_weekend(saturday));
        assert!(is_weekend(sunday));
        assert!(!is_weekend(monday));
    }

    #[test]
    fn test_next_business_day() {
        let friday = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap();
        let next = next_business_day(friday);
        assert_eq!(next.weekday(), Weekday::Mon);
    }

    #[test]
    fn test_prev_business_day() {
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        let prev = prev_business_day(monday);
        assert_eq!(prev.weekday(), Weekday::Fri);
    }

    #[test]
    fn test_business_date() {
        let mut rng = StdRng::seed_from_u64(42);
        let bd = business_date(&mut rng, 30);
        assert!(is_business_day(bd));
    }

    #[test]
    fn test_add_business_days() {
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        let result = add_business_days(monday, 5);
        // Monday + 5 business days = next Monday
        assert_eq!(result.weekday(), Weekday::Mon);
    }

    #[test]
    fn test_sub_business_days() {
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        let result = sub_business_days(monday, 5);
        // Monday - 5 business days = previous Monday
        assert_eq!(result.weekday(), Weekday::Mon);
    }

    #[test]
    fn test_business_days_between() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(); // Next Monday
        let count = business_days_between(start, end);
        assert_eq!(count, 5); // Mon, Tue, Wed, Thu, Fri
    }

    #[test]
    fn test_weekday_name() {
        let monday = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        assert_eq!(weekday_name(monday), "Monday");
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(business_date(&mut rng1, 30), business_date(&mut rng2, 30));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let bd = business_date(&mut *rng, 30);
        assert!(is_business_day(bd));
    }
}
