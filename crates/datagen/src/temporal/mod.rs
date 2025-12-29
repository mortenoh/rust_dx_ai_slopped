//! Temporal data generation.
//!
//! Generate dates, times, and datetimes with various formats.
//! This module requires the `temporal` feature flag.

pub mod business;
pub mod date;
pub mod datetime;
pub mod time;

pub use business::{business_date, is_business_day, next_business_day};
pub use date::{date_between, date_future, date_past, date_recent};
pub use datetime::{datetime_between, datetime_past, iso8601, timestamp_unix};
pub use time::{time_between, time_of_day, time_random};
