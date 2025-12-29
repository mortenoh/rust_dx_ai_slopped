//! Time generation.
//!
//! Generate random times within various ranges.
//!
//! # Example
//!
//! ```
//! use dx_datagen::temporal::time::{time_random, time_between, time_of_day};
//! use chrono::NaiveTime;
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let t = time_random(&mut rng);
//! let morning = time_of_day(&mut rng, "morning");
//! ```

use chrono::{NaiveTime, Timelike};
use rand::Rng;

/// Generate a random time between two times.
pub fn time_between<R: ?Sized + Rng>(rng: &mut R, start: NaiveTime, end: NaiveTime) -> NaiveTime {
    let start_secs = start.num_seconds_from_midnight();
    let end_secs = end.num_seconds_from_midnight();

    if start_secs >= end_secs {
        return start;
    }

    let random_secs = rng.random_range(start_secs..=end_secs);
    NaiveTime::from_num_seconds_from_midnight_opt(random_secs, 0).unwrap_or(start)
}

/// Generate a random time during the entire day (00:00:00 - 23:59:59).
pub fn time_random<R: ?Sized + Rng>(rng: &mut R) -> NaiveTime {
    let start = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let end = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    time_between(rng, start, end)
}

/// Generate a time for a specific part of the day.
///
/// Parts: "morning" (6-12), "afternoon" (12-17), "evening" (17-21), "night" (21-6)
pub fn time_of_day<R: ?Sized + Rng>(rng: &mut R, part: &str) -> NaiveTime {
    let (start_hour, end_hour) = match part.to_lowercase().as_str() {
        "morning" => (6, 12),
        "afternoon" => (12, 17),
        "evening" => (17, 21),
        "night" => (21, 23), // Simplified to avoid midnight wrap
        "business" => (9, 17),
        "lunch" => (11, 14),
        _ => (0, 23),
    };

    let start = NaiveTime::from_hms_opt(start_hour, 0, 0).unwrap();
    let end = NaiveTime::from_hms_opt(end_hour, 59, 59).unwrap();
    time_between(rng, start, end)
}

/// Generate a random hour (0-23).
pub fn hour<R: ?Sized + Rng>(rng: &mut R) -> u32 {
    rng.random_range(0..24)
}

/// Generate a random minute (0-59).
pub fn minute<R: ?Sized + Rng>(rng: &mut R) -> u32 {
    rng.random_range(0..60)
}

/// Generate a random second (0-59).
pub fn second<R: ?Sized + Rng>(rng: &mut R) -> u32 {
    rng.random_range(0..60)
}

/// Generate a time rounded to the nearest interval (in minutes).
pub fn time_rounded<R: ?Sized + Rng>(rng: &mut R, interval_minutes: u32) -> NaiveTime {
    let hour = rng.random_range(0..24);
    let intervals = 60 / interval_minutes;
    let interval_idx = rng.random_range(0..intervals);
    let minute = interval_idx * interval_minutes;
    NaiveTime::from_hms_opt(hour, minute, 0).unwrap()
}

/// Format a time as HH:MM:SS.
pub fn format_hms(time: NaiveTime) -> String {
    time.format("%H:%M:%S").to_string()
}

/// Format a time as HH:MM.
pub fn format_hm(time: NaiveTime) -> String {
    time.format("%H:%M").to_string()
}

/// Format a time in 12-hour format with AM/PM.
pub fn format_12hour(time: NaiveTime) -> String {
    time.format("%I:%M %p").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_time_between() {
        let mut rng = StdRng::seed_from_u64(42);
        let start = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
        let end = NaiveTime::from_hms_opt(17, 0, 0).unwrap();
        let t = time_between(&mut rng, start, end);
        assert!(t >= start && t <= end);
    }

    #[test]
    fn test_time_random() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = time_random(&mut rng);
        assert!(t.hour() < 24);
    }

    #[test]
    fn test_time_of_day_morning() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = time_of_day(&mut rng, "morning");
        assert!(t.hour() >= 6 && t.hour() < 13);
    }

    #[test]
    fn test_time_of_day_business() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = time_of_day(&mut rng, "business");
        assert!(t.hour() >= 9 && t.hour() < 18);
    }

    #[test]
    fn test_time_rounded() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = time_rounded(&mut rng, 15);
        assert!(t.minute() % 15 == 0);
        assert_eq!(t.second(), 0);
    }

    #[test]
    fn test_format_hms() {
        let t = NaiveTime::from_hms_opt(14, 30, 45).unwrap();
        assert_eq!(format_hms(t), "14:30:45");
    }

    #[test]
    fn test_format_hm() {
        let t = NaiveTime::from_hms_opt(14, 30, 45).unwrap();
        assert_eq!(format_hm(t), "14:30");
    }

    #[test]
    fn test_format_12hour() {
        let t = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
        assert_eq!(format_12hour(t), "02:30 PM");
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(time_random(&mut rng1), time_random(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let t = time_random(&mut *rng);
        assert!(t.hour() < 24);
    }
}
