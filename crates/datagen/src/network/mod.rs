//! Network data generation.
//!
//! Generate IP addresses, MAC addresses, domain names, and URLs.

pub mod domain;
pub mod ip;
pub mod mac;
pub mod url;

pub use domain::{domain, subdomain, tld};
pub use ip::{ipv4, ipv4_private, ipv4_public, ipv6};
pub use mac::mac_address;
pub use url::{url, url_https, url_with_path};
