//! Numeric pattern generation.
//!
//! Generate formatted numeric identifiers like credit cards, ISBN, SSN, and IBAN.

pub mod credit_card;
pub mod finance;
pub mod iban;
pub mod isbn;
pub mod ssn;

pub use credit_card::{credit_card, credit_card_type, validate_luhn, CardType};
pub use finance::{
    account_number, bic, bitcoin_address, ethereum_address, routing_number, swift_code,
    transaction_description, transaction_type,
};
pub use iban::{iban, iban_for_country};
pub use isbn::{isbn10, isbn13};
pub use ssn::{ssn_no, ssn_us};
