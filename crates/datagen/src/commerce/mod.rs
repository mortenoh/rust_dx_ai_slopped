//! Commerce and business data generation.
//!
//! This module provides generators for:
//! - Company names and suffixes
//! - Product names and categories
//! - Job titles and departments
//! - Currency codes and symbols
//!
//! # Example
//!
//! ```
//! use dx_datagen::commerce;
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//! let company = commerce::company_name(&mut rng);
//! let product = commerce::product_name(&mut rng);
//! let job = commerce::job_title(&mut rng);
//! ```

mod company;
mod currency;
mod job;
mod product;

pub use company::{
    catch_phrase, company_bs, company_name, company_suffix, industry, COMPANY_SUFFIXES, INDUSTRIES,
};
pub use currency::{currency_code, currency_name, currency_symbol, CURRENCIES};
pub use job::{
    department, job_area, job_descriptor, job_title, job_type, DEPARTMENTS, JOB_AREAS,
    JOB_DESCRIPTORS, JOB_TYPES,
};
pub use product::{
    price, price_formatted, product_adjective, product_category, product_material, product_name,
    PRODUCT_ADJECTIVES, PRODUCT_CATEGORIES, PRODUCT_MATERIALS,
};
