//! Provider registry for the expression DSL.
//!
//! Maps provider calls like `Name.firstName` to actual generator functions.

use rand::Rng;

use crate::expression::ast::Argument;
use crate::{
    categories, color, commerce, entertainment, file, food, generators, hacker, healthcare,
    network, personal, science, text, travel, vehicle, weather,
};

#[cfg(feature = "temporal")]
use crate::temporal;

/// Provider error type.
#[derive(Debug, Clone)]
pub struct ProviderError {
    pub message: String,
}

impl ProviderError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn unknown_provider(provider: &str) -> Self {
        Self::new(&format!("Unknown provider: {}", provider))
    }

    pub fn unknown_method(provider: &str, method: &str) -> Self {
        Self::new(&format!("Unknown method: {}.{}", provider, method))
    }
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Provider error: {}", self.message)
    }
}

impl std::error::Error for ProviderError {}

/// Call a provider method.
pub fn call_provider<R: Rng + ?Sized>(
    rng: &mut R,
    provider: &str,
    method: &str,
    args: &[Argument],
) -> Result<String, ProviderError> {
    match provider {
        "Name" => call_name_provider(rng, method),
        "Address" => call_address_provider(rng, method),
        "Phone" | "PhoneNumber" => call_phone_provider(rng, method),
        "Internet" => call_internet_provider(rng, method),
        "Commerce" => call_commerce_provider(rng, method),
        "Company" => call_company_provider(rng, method),
        "Number" => call_number_provider(rng, method, args),
        "Date" | "DateAndTime" => call_date_provider(rng, method),
        "Lorem" => call_lorem_provider(rng, method, args),
        "Color" => call_color_provider(rng, method),
        "File" => call_file_provider(rng, method),
        "Vehicle" => call_vehicle_provider(rng, method),
        "Book" => call_book_provider(rng, method),
        "Music" => call_music_provider(rng, method),
        "Movie" => call_movie_provider(rng, method),
        "Food" => call_food_provider(rng, method),
        "Science" => call_science_provider(rng, method),
        "Weather" => call_weather_provider(rng, method),
        "Travel" => call_travel_provider(rng, method),
        "Healthcare" | "Medical" => call_healthcare_provider(rng, method),
        "Hacker" => call_hacker_provider(rng, method),
        "Geo" | "Geography" => call_geo_provider(rng, method),
        _ => Err(ProviderError::unknown_provider(provider)),
    }
}

/// Get list of available providers.
pub fn available_providers() -> Vec<&'static str> {
    vec![
        "Name",
        "Address",
        "Phone",
        "PhoneNumber",
        "Internet",
        "Commerce",
        "Company",
        "Number",
        "Date",
        "DateAndTime",
        "Lorem",
        "Color",
        "File",
        "Vehicle",
        "Book",
        "Music",
        "Movie",
        "Food",
        "Science",
        "Weather",
        "Travel",
        "Healthcare",
        "Medical",
        "Hacker",
        "Geo",
        "Geography",
    ]
}

// Name provider
fn call_name_provider<R: Rng + ?Sized>(rng: &mut R, method: &str) -> Result<String, ProviderError> {
    match method {
        "firstName" | "first_name" => Ok(personal::first_name(rng).to_string()),
        "lastName" | "last_name" => Ok(personal::last_name(rng).to_string()),
        "fullName" | "full_name" | "name" => Ok(personal::full_name(rng)),
        "username" => Ok(personal::username(rng)),
        _ => Err(ProviderError::unknown_method("Name", method)),
    }
}

// Address provider
fn call_address_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "streetAddress" | "street_address" | "streetName" | "street" => {
            Ok(personal::address::street_address(rng))
        }
        "city" => Ok(personal::address::city(rng).to_string()),
        "state" => Ok(personal::address::state(rng).to_string()),
        "zipCode" | "zip_code" | "postcode" | "postalCode" => Ok(personal::address::zip_code(rng)),
        "country" => Ok(generators::pick_one(rng, categories::COUNTRIES).to_string()),
        "fullAddress" | "full_address" | "address" => {
            Ok(personal::address::full_address(rng).format_us())
        }
        "buildingNumber" | "building_number" => Ok(personal::address::building_number(rng)),
        "latitude" => Ok(format!("{:.6}", crate::geo::latitude(rng))),
        "longitude" => Ok(format!("{:.6}", crate::geo::longitude(rng))),
        _ => Err(ProviderError::unknown_method("Address", method)),
    }
}

// Phone provider
fn call_phone_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "phoneNumber" | "phone_number" | "number" | "phone" => Ok(personal::phone(rng)),
        "cellPhone" | "cell_phone" | "mobile" => Ok(personal::phone::mobile_phone(rng)),
        _ => Err(ProviderError::unknown_method("Phone", method)),
    }
}

// Internet provider
fn call_internet_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "email" | "emailAddress" => Ok(personal::email(rng)),
        "url" => Ok(network::url(rng)),
        "domainName" | "domain_name" | "domain" => Ok(network::domain(rng)),
        "ipAddress" | "ip_address" | "ipV4" | "ipv4" => Ok(network::ipv4(rng).to_string()),
        "ipV6" | "ipv6" => Ok(network::ipv6(rng).to_string()),
        "macAddress" | "mac_address" => Ok(network::mac_address(rng)),
        "userAgent" | "user_agent" => Ok(file::user_agent(rng).to_string()),
        "password" => Ok(crate::password::password(rng, 12, true)),
        "uuid" => Ok(crate::uuid::v4().to_string()),
        "slug" => Ok(text::words::slug(rng)),
        _ => Err(ProviderError::unknown_method("Internet", method)),
    }
}

// Commerce provider
fn call_commerce_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "productName" | "product_name" | "product" => Ok(commerce::product_name(rng)),
        "price" => Ok(format!("{:.2}", commerce::price(rng, 1.0, 1000.0))),
        "department" => Ok(commerce::department(rng).to_string()),
        "material" => Ok(commerce::product_material(rng).to_string()),
        "color" => Ok(commerce::product_adjective(rng).to_string()),
        "promotionCode" | "promotion_code" => {
            // Generate a simple promo code
            let code = format!(
                "{}{}",
                generators::pick_one(rng, &["SAVE", "DEAL", "PROMO", "OFF", "DISCOUNT"]),
                rng.random_range(10..99)
            );
            Ok(code)
        }
        _ => Err(ProviderError::unknown_method("Commerce", method)),
    }
}

// Company provider
fn call_company_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "name" | "companyName" | "company_name" => Ok(commerce::company_name(rng)),
        "buzzword" => Ok(commerce::catch_phrase(rng)),
        "bs" | "catchPhrase" | "catch_phrase" => Ok(commerce::catch_phrase(rng)),
        "suffix" => Ok(commerce::company_suffix(rng).to_string()),
        "industry" => Ok(commerce::industry(rng).to_string()),
        _ => Err(ProviderError::unknown_method("Company", method)),
    }
}

// Number provider
fn call_number_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
    args: &[Argument],
) -> Result<String, ProviderError> {
    match method {
        "between" | "numberBetween" => {
            let min = args.first().and_then(|a| a.as_i64()).unwrap_or(0);
            let max = args.get(1).and_then(|a| a.as_i64()).unwrap_or(100);
            Ok(rng.random_range(min..=max).to_string())
        }
        "digit" => Ok(rng.random_range(0..=9).to_string()),
        "digits" => {
            let count = args.first().and_then(|a| a.as_usize()).unwrap_or(5);
            let digits: String = (0..count)
                .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
                .collect();
            Ok(digits)
        }
        "positive" => {
            let max = args.first().and_then(|a| a.as_i64()).unwrap_or(1000);
            Ok(rng.random_range(1..=max).to_string())
        }
        "negative" => {
            let min = args.first().and_then(|a| a.as_i64()).unwrap_or(-1000);
            Ok(rng.random_range(min..0).to_string())
        }
        "decimal" => {
            let min = args.first().and_then(|a| a.as_f64()).unwrap_or(0.0);
            let max = args.get(1).and_then(|a| a.as_f64()).unwrap_or(100.0);
            Ok(format!("{:.2}", rng.random_range(min..max)))
        }
        _ => Err(ProviderError::unknown_method("Number", method)),
    }
}

// Date provider
#[allow(unused_variables)]
fn call_date_provider<R: Rng + ?Sized>(rng: &mut R, method: &str) -> Result<String, ProviderError> {
    #[cfg(feature = "temporal")]
    {
        match method {
            "past" => Ok(temporal::date_past(rng, 365 * 5).to_string()),
            "future" => Ok(temporal::date_future(rng, 365 * 5).to_string()),
            "birthday" => Ok(temporal::date::birth_date(rng, 18, 80).to_string()),
            "date" => Ok(temporal::date::date_recent(rng).to_string()),
            "time" => Ok(temporal::time::time_random(rng).to_string()),
            "datetime" | "dateTime" => Ok(temporal::datetime::datetime_recent(rng).to_string()),
            "timestamp" => Ok(temporal::timestamp_unix(rng).to_string()),
            "month" => Ok(generators::pick_one(rng, categories::MONTHS).to_string()),
            "dayOfWeek" | "day_of_week" | "day" => {
                Ok(generators::pick_one(rng, categories::DAYS).to_string())
            }
            "year" => Ok(rng.random_range(1970..2030).to_string()),
            _ => Err(ProviderError::unknown_method("Date", method)),
        }
    }
    #[cfg(not(feature = "temporal"))]
    {
        match method {
            "month" => Ok(generators::pick_one(rng, categories::MONTHS).to_string()),
            "dayOfWeek" | "day_of_week" | "day" => {
                Ok(generators::pick_one(rng, categories::DAYS).to_string())
            }
            "year" => Ok(rng.random_range(1970..2030).to_string()),
            _ => Err(ProviderError::new(
                "Date provider requires 'temporal' feature",
            )),
        }
    }
}

// Lorem provider
fn call_lorem_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
    args: &[Argument],
) -> Result<String, ProviderError> {
    match method {
        "word" => Ok(text::word(rng).to_string()),
        "words" => {
            let count = args.first().and_then(|a| a.as_usize()).unwrap_or(5);
            Ok(text::lorem::words(rng, count))
        }
        "sentence" => Ok(text::sentence(rng)),
        "sentences" => {
            let count = args.first().and_then(|a| a.as_usize()).unwrap_or(3);
            Ok(text::sentences(rng, count))
        }
        "paragraph" => Ok(text::paragraph(rng)),
        "paragraphs" => {
            let count = args.first().and_then(|a| a.as_usize()).unwrap_or(3);
            Ok(text::paragraphs(rng, count))
        }
        _ => Err(ProviderError::unknown_method("Lorem", method)),
    }
}

// Color provider
fn call_color_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "name" | "colorName" | "color_name" => Ok(color::color_name(rng).to_string()),
        "hex" | "hexColor" | "hex_color" => Ok(color::hex_color(rng)),
        "rgb" | "rgbColor" | "rgb_color" => {
            let (r, g, b) = color::rgb(rng);
            Ok(format!("rgb({}, {}, {})", r, g, b))
        }
        "hsl" => {
            let (h, s, l) = color::hsl(rng);
            Ok(format!("hsl({}, {}%, {}%)", h, s, l))
        }
        "safe" | "safeColor" | "safe_color" => Ok(color::css_color_name(rng).to_string()),
        _ => Err(ProviderError::unknown_method("Color", method)),
    }
}

// File provider
fn call_file_provider<R: Rng + ?Sized>(rng: &mut R, method: &str) -> Result<String, ProviderError> {
    match method {
        "fileName" | "file_name" | "name" => Ok(file::file_name(rng)),
        "extension" | "ext" => Ok(file::file_extension(rng).to_string()),
        "mimeType" | "mime_type" => Ok(file::mime_type(rng).to_string()),
        "path" | "filePath" | "file_path" => Ok(file::file_path(rng)),
        "directoryPath" | "directory_path" | "dir" => Ok(file::directory_path(rng)),
        _ => Err(ProviderError::unknown_method("File", method)),
    }
}

// Vehicle provider
fn call_vehicle_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "make" | "manufacturer" => Ok(vehicle::vehicle_make(rng).to_string()),
        "model" => Ok(vehicle::vehicle_model(rng).to_string()),
        "type" | "vehicleType" | "vehicle_type" => Ok(vehicle::vehicle_type(rng).to_string()),
        "fuel" | "fuelType" | "fuel_type" => Ok(vehicle::fuel_type(rng).to_string()),
        "vin" => Ok(vehicle::vin(rng)),
        "licensePlate" | "license_plate" => Ok(vehicle::license_plate(rng)),
        "color" => Ok(generators::pick_one(rng, categories::COLORS).to_string()),
        _ => Err(ProviderError::unknown_method("Vehicle", method)),
    }
}

// Book provider
fn call_book_provider<R: Rng + ?Sized>(rng: &mut R, method: &str) -> Result<String, ProviderError> {
    match method {
        "title" => Ok(entertainment::book_title(rng)),
        "author" => Ok(entertainment::book_author(rng).to_string()),
        "genre" => Ok(entertainment::book_genre(rng).to_string()),
        "publisher" => Ok(entertainment::book_publisher(rng).to_string()),
        _ => Err(ProviderError::unknown_method("Book", method)),
    }
}

// Music provider
fn call_music_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "genre" => Ok(entertainment::music_genre(rng).to_string()),
        "artist" => Ok(entertainment::music_artist(rng).to_string()),
        "instrument" => Ok(entertainment::music_instrument(rng).to_string()),
        _ => Err(ProviderError::unknown_method("Music", method)),
    }
}

// Movie provider
fn call_movie_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "title" => Ok(entertainment::movie_title(rng).to_string()),
        "genre" => Ok(entertainment::movie_genre(rng).to_string()),
        "actor" => Ok(entertainment::movie_actor(rng).to_string()),
        "director" => Ok(entertainment::movie_director(rng).to_string()),
        _ => Err(ProviderError::unknown_method("Movie", method)),
    }
}

// Food provider
fn call_food_provider<R: Rng + ?Sized>(rng: &mut R, method: &str) -> Result<String, ProviderError> {
    match method {
        "dish" => Ok(food::dish(rng).to_string()),
        "ingredient" => Ok(food::ingredient(rng).to_string()),
        "spice" => Ok(food::spice(rng).to_string()),
        "fruit" => Ok(food::fruit(rng).to_string()),
        "vegetable" => Ok(food::vegetable(rng).to_string()),
        "beverage" | "drink" => Ok(food::beverage(rng).to_string()),
        "cuisine" => Ok(food::cuisine(rng).to_string()),
        _ => Err(ProviderError::unknown_method("Food", method)),
    }
}

// Science provider
fn call_science_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "element" => Ok(science::chemical_element(rng).to_string()),
        "elementSymbol" | "element_symbol" | "symbol" => {
            Ok(science::chemical_symbol(rng).to_string())
        }
        "unit" => Ok(science::unit(rng).to_string()),
        _ => Err(ProviderError::unknown_method("Science", method)),
    }
}

// Weather provider
fn call_weather_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "condition" | "description" => Ok(weather::condition(rng).to_string()),
        "temperature" | "temperatureCelsius" => Ok(format!("{}°C", weather::temperature_c(rng))),
        "temperatureFahrenheit" | "temperature_fahrenheit" => {
            Ok(format!("{}°F", weather::temperature_f(rng)))
        }
        "humidity" => Ok(format!("{}%", weather::humidity(rng))),
        "windSpeed" | "wind_speed" => Ok(format!("{} km/h", weather::wind_speed_kmh(rng))),
        "windDirection" | "wind_direction" => Ok(weather::wind_direction(rng).to_string()),
        "pressure" => Ok(format!("{} hPa", weather::pressure_mb(rng))),
        "uvIndex" | "uv_index" => Ok(weather::uv_index(rng).to_string()),
        _ => Err(ProviderError::unknown_method("Weather", method)),
    }
}

// Travel provider
fn call_travel_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "airline" => Ok(travel::airline(rng).to_string()),
        "airport" => Ok(travel::airport_name(rng).to_string()),
        "airportCode" | "airport_code" => Ok(travel::airport_code(rng).to_string()),
        "flightNumber" | "flight_number" => Ok(travel::flight_number(rng)),
        "hotelName" | "hotel_name" | "hotel" => Ok(travel::hotel_name(rng)),
        "hotelChain" | "hotel_chain" => Ok(travel::hotel_chain(rng).to_string()),
        _ => Err(ProviderError::unknown_method("Travel", method)),
    }
}

// Healthcare provider
fn call_healthcare_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "diagnosis" | "condition" => Ok(healthcare::condition(rng).to_string()),
        "medication" | "medicine" | "drug" => Ok(healthcare::medication(rng).to_string()),
        "specialty" => Ok(healthcare::specialty(rng).to_string()),
        "bloodType" | "blood_type" => Ok(healthcare::blood_type(rng).to_string()),
        "hospital" => Ok(healthcare::hospital_name(rng)),
        _ => Err(ProviderError::unknown_method("Healthcare", method)),
    }
}

// Hacker provider
fn call_hacker_provider<R: Rng + ?Sized>(
    rng: &mut R,
    method: &str,
) -> Result<String, ProviderError> {
    match method {
        "abbreviation" => Ok(hacker::hacker_abbreviation(rng).to_string()),
        "adjective" => Ok(hacker::hacker_adjective(rng).to_string()),
        "noun" => Ok(hacker::hacker_noun(rng).to_string()),
        "verb" => Ok(hacker::hacker_verb(rng).to_string()),
        "phrase" => Ok(hacker::hacker_phrase(rng)),
        _ => Err(ProviderError::unknown_method("Hacker", method)),
    }
}

// Geo provider
fn call_geo_provider<R: Rng + ?Sized>(rng: &mut R, method: &str) -> Result<String, ProviderError> {
    match method {
        "latitude" => Ok(format!("{:.6}", crate::geo::latitude(rng))),
        "longitude" => Ok(format!("{:.6}", crate::geo::longitude(rng))),
        "coordinates" => {
            let (lat, lon) = crate::geo::coordinates(rng);
            Ok(format!("{:.6}, {:.6}", lat, lon))
        }
        "country" => Ok(generators::pick_one(rng, categories::COUNTRIES).to_string()),
        "city" => Ok(personal::address::city(rng).to_string()),
        "timezone" => Ok(generators::pick_one(
            rng,
            &[
                "UTC",
                "America/New_York",
                "America/Los_Angeles",
                "Europe/London",
                "Europe/Paris",
                "Asia/Tokyo",
                "Asia/Shanghai",
                "Australia/Sydney",
            ],
        )
        .to_string()),
        "continent" => Ok(generators::pick_one(
            rng,
            &[
                "Africa",
                "Antarctica",
                "Asia",
                "Europe",
                "North America",
                "Oceania",
                "South America",
            ],
        )
        .to_string()),
        _ => Err(ProviderError::unknown_method("Geo", method)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn test_rng() -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(42)
    }

    #[test]
    fn test_name_provider() {
        let mut rng = test_rng();
        assert!(!call_provider(&mut rng, "Name", "firstName", &[])
            .unwrap()
            .is_empty());
        assert!(!call_provider(&mut rng, "Name", "lastName", &[])
            .unwrap()
            .is_empty());
        assert!(!call_provider(&mut rng, "Name", "fullName", &[])
            .unwrap()
            .is_empty());
    }

    #[test]
    fn test_address_provider() {
        let mut rng = test_rng();
        assert!(!call_provider(&mut rng, "Address", "city", &[])
            .unwrap()
            .is_empty());
        assert!(!call_provider(&mut rng, "Address", "country", &[])
            .unwrap()
            .is_empty());
    }

    #[test]
    fn test_number_provider() {
        let mut rng = test_rng();
        let result = call_provider(
            &mut rng,
            "Number",
            "between",
            &[Argument::Number(1.0), Argument::Number(100.0)],
        )
        .unwrap();
        let num: i64 = result.parse().unwrap();
        assert!((1..=100).contains(&num));
    }

    #[test]
    fn test_lorem_provider() {
        let mut rng = test_rng();
        assert!(!call_provider(&mut rng, "Lorem", "word", &[])
            .unwrap()
            .is_empty());
        assert!(!call_provider(&mut rng, "Lorem", "sentence", &[])
            .unwrap()
            .is_empty());
    }

    #[test]
    fn test_internet_provider() {
        let mut rng = test_rng();
        let email = call_provider(&mut rng, "Internet", "email", &[]).unwrap();
        assert!(email.contains('@'));
    }

    #[test]
    fn test_unknown_provider() {
        let mut rng = test_rng();
        let result = call_provider(&mut rng, "Unknown", "method", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_method() {
        let mut rng = test_rng();
        let result = call_provider(&mut rng, "Name", "unknownMethod", &[]);
        assert!(result.is_err());
    }
}
