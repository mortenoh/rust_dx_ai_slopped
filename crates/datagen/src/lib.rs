//! Data generation utilities for test data, random values, and predefined categories.
//!
//! This crate provides:
//! - **categories**: Predefined value sets (cities, countries, fruits, colors, etc.)
//! - **generators**: Core random data generators (int, float, string, boolean)
//! - **password**: Password and charset-based string generation
//! - **uuid**: UUID generation (v4, v7) with formatting options
//! - **selection**: Weighted random selection
//! - **text**: Pattern-based text generation, word lists, lorem ipsum
//! - **personal**: Personal data (names, email, phone, address, username)
//! - **network**: Network data (IP addresses, MAC addresses, domains, URLs)
//! - **numeric**: Formatted numeric identifiers (credit cards, ISBN, SSN, IBAN)
//! - **temporal**: Date and time generation (feature-gated with `temporal`)
//! - **geo**: Geographic coordinates and GeoJSON points (feature-gated with `geo`)
//!
//! # Example
//!
//! ```
//! use dx_datagen::{generators, categories, password, uuid};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! // Generate random values
//! let num = generators::int_range(&mut rng, 1, 100);
//! let name = generators::alphanumeric(&mut rng, 8);
//!
//! // Pick from categories
//! let city = generators::pick_one(&mut rng, categories::CITIES);
//! let fruit = generators::pick_one(&mut rng, categories::FRUITS);
//!
//! // Generate passwords
//! let pwd = password::password(&mut rng, 16, true);
//!
//! // Generate UUIDs
//! let id = uuid::v4();
//! ```

// Core modules
pub mod categories;
pub mod color;
pub mod generators;
pub mod password;
pub mod uuid;

// Extended modules
pub mod animals;
pub mod astrology;
pub mod commerce;
pub mod crypto;
pub mod ecommerce;
pub mod education;
pub mod entertainment;
pub mod expression;
pub mod file;
pub mod food;
pub mod government;
pub mod hacker;
pub mod healthcare;
pub mod locale;
pub mod network;
pub mod numeric;
pub mod personal;
pub mod science;
pub mod selection;
pub mod social;
pub mod sports;
pub mod text;
pub mod travel;
pub mod vehicle;
pub mod weather;

// Feature-gated modules
#[cfg(feature = "temporal")]
pub mod temporal;

// Geo module (always available, but GeoJSON output requires "geo" feature)
pub mod geo;

// Schema module - schema-based data generation (requires "schema" feature)
#[cfg(feature = "schema")]
pub mod schema;

// Re-export commonly used items at crate root
pub use generators::{
    alphanumeric, boolean, float_range, hex_bytes, hex_string, int_range, pick_one, shuffle,
};
pub use uuid::{
    ulid, ulid_from_timestamp, ulid_with_rng, v4, v7, Ulid, Uuid, UuidFormat, UuidVersion,
};

// Re-export selection
pub use selection::{
    generate_batch, generate_batch_map, generate_batch_nullable, generate_batch_unique,
    generate_batch_unique_with_retries, generate_until, weighted_pick, weighted_pick_from,
    UniqueError, UniqueGenerator, UniqueTracker, WeightedItem, WeightedSelector,
};

// Re-export text
pub use text::{
    adjective, from_pattern, noun, render, render_default, verb, word, ProviderRegistry, Template,
};

// Re-export personal
pub use personal::{email, email_with_domain, first_name, full_name, last_name, phone, username};

// Re-export network
pub use network::{
    domain, ipv4, ipv4_private, ipv4_public, ipv6, mac_address, subdomain, tld, url, url_https,
    url_with_path,
};

// Re-export numeric
pub use numeric::{
    account_number, bic, bitcoin_address, credit_card, credit_card_type, ethereum_address, iban,
    iban_for_country, isbn10, isbn13, routing_number, ssn_no, ssn_us, swift_code,
    transaction_description, transaction_type, validate_luhn, CardType,
};

// Re-export locale
pub use locale::{Locale, LocaleData};

// Re-export geo
pub use geo::{
    coordinates, coordinates_in_bounds, coordinates_string, latitude, latitude_in_range, longitude,
    longitude_in_range,
};

#[cfg(feature = "geo")]
pub use geo::{geojson_point, geojson_point_string};

// Re-export color
pub use color::{
    color_name, css_color_name, css_hsl, css_hsla, css_rgb, css_rgba, hex_color, hex_color_alpha,
    hsl, hsla, rgb, rgba,
};

// Re-export file
pub use file::{
    directory_path, file_extension, file_extension_by_category, file_name, file_path, mime_type,
    mime_type_by_category, semver, semver_with_prerelease, user_agent, ExtensionCategory,
    MimeCategory,
};

// Re-export commerce
pub use commerce::{
    catch_phrase, company_bs, company_name, company_suffix, currency_code, currency_name,
    currency_symbol, department, industry, job_area, job_descriptor, job_title, job_type, price,
    price_formatted, product_adjective, product_category, product_material, product_name,
};

// Re-export vehicle
pub use vehicle::{
    fuel_type, license_plate, vehicle_full, vehicle_make, vehicle_model, vehicle_type,
    vehicle_year, vin,
};

// Re-export science
pub use science::{
    chemical_element, chemical_symbol, derived_unit, derived_unit_symbol, element_full,
    scientific_notation, unit, unit_symbol,
};

// Re-export entertainment
pub use entertainment::{
    book_author, book_genre, book_publisher, book_series, book_title, game_genre, game_platform,
    game_studio, game_title, movie_actor, movie_director, movie_genre, movie_rating, movie_title,
    music_album, music_artist, music_genre, music_instrument, music_song, tv_channel, tv_genre,
    tv_network, tv_show,
};

// Re-export food
pub use food::{
    beer_style, beverage, coffee_drink, cuisine, dessert, dish, fruit, ingredient, meal_type, meat,
    restaurant_name, restaurant_type, spice, tea_type, vegetable, wine_variety,
};

// Re-export animals
pub use animals::{animal, bird, cat_breed, dog_breed, fish, insect, mammal, pet_name, reptile};

// Re-export travel
pub use travel::{
    aircraft_type, airline, airport_code, airport_name, destination, flight_number, hotel_chain,
    hotel_name, landmark, room_type, seat, seat_class,
};

// Re-export healthcare
pub use healthcare::{
    blood_type, body_part, condition, doctor_title, hospital_name, medication, organ, specialty,
    symptom,
};

// Re-export sports
pub use sports::{championship, league, mascot, position, score, sport, team_name, tournament};

// Re-export hacker
pub use hacker::{
    cloud_provider, database, devops_tool, framework, git_branch, git_commit_message, git_sha,
    git_sha_full, hacker_abbreviation, hacker_adjective, hacker_noun, hacker_phrase, hacker_verb,
    programming_language,
};

// Re-export education
pub use education::{
    classroom, course_code, course_name, degree, degree_type, gpa, grade, major, school_name,
    subject, university,
};

// Re-export government
pub use government::{
    bill_number, case_number, document_type, drivers_license, drivers_license_state,
    government_position, passport_number, permit_number, political_party, tax_id, us_agency,
    voter_registration_number,
};

// Re-export weather
pub use weather::condition as weather_condition;
pub use weather::{
    air_quality, cloud_coverage, dew_point_f, forecast_summary, humidity, precipitation_chance,
    pressure_mb, season, temperature_c, temperature_f, temperature_f_season, uv_index, uv_level,
    visibility_miles, wind_direction, wind_speed_kmh, wind_speed_mph,
};

// Re-export astrology
pub use astrology::{
    astrological_house, birthstone, birthstone_for_month, chinese_element, chinese_zodiac,
    chinese_zodiac_full, element_for_sign, horoscope, modality_for_sign,
    moon_phase as astro_moon_phase, planet, zodiac_element, zodiac_modality, zodiac_sign,
    zodiac_sign_with_dates,
};
