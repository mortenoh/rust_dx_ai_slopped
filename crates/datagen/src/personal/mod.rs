//! Personal data generation.
//!
//! Generate realistic personal information including names, emails, phone numbers, and addresses.

pub mod address;
pub mod email;
pub mod names;
pub mod phone;
pub mod username;

pub use address::{full_address, street_address, zip_code, Address};
pub use email::{email, email_from_name, email_with_domain};
pub use names::{first_name, first_name_female, first_name_male, full_name, last_name};
pub use phone::{phone, phone_e164, phone_us};
pub use username::username;
