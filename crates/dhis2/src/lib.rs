//! # dx-dhis2 - DHIS2 API Client Library
//!
//! A Rust client library for interacting with DHIS2 (District Health Information
//! System 2) instances.
//!
//! ## Features
//!
//! - DHIS2 UID generation and validation
//! - System information fetching
//! - Organisation unit queries
//! - Data element queries
//! - Data set queries
//! - Optional TUI browser (with `tui` feature)
//!
//! ## Quick Start
//!
//! ```no_run
//! use dx_dhis2::{Dhis2Client, uid};
//!
//! // Generate DHIS2 UIDs
//! let uid = uid::generate();
//! println!("Generated UID: {}", uid);
//!
//! // Connect to a DHIS2 instance
//! let client = Dhis2Client::new(
//!     "https://play.im.dhis2.org/demo",
//!     "admin",
//!     "district"
//! ).unwrap();
//!
//! // Fetch system info
//! let info = dx_dhis2::info::fetch(&client).unwrap();
//! println!("DHIS2 version: {:?}", info.version);
//! ```
//!
//! ## DHIS2 UID Format
//!
//! DHIS2 UIDs are 11-character alphanumeric identifiers:
//! - First character: a-zA-Z (must be a letter)
//! - Remaining 10: a-zA-Z0-9 (alphanumeric)

mod client;
pub mod data_elements;
pub mod data_sets;
pub mod data_values;
pub mod info;
pub mod org_unit_group_sets;
pub mod org_unit_groups;
pub mod org_units;
#[cfg(feature = "tui")]
pub mod tui;
pub mod uid;

pub use client::Dhis2Client;

/// Default DHIS2 demo server URL (redirects to latest version).
pub const DEFAULT_SERVER: &str = "https://play.dhis2.org/demo";

/// Default username for the demo server.
pub const DEFAULT_USER: &str = "admin";

/// Default password for the demo server.
pub const DEFAULT_PASSWORD: &str = "district";
