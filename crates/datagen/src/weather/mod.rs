//! Weather data generators.
//!
//! Provides generators for weather-related fake data including conditions,
//! temperatures, and forecasts.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Weather conditions.
static CONDITIONS: &[&str] = &[
    "Sunny",
    "Partly Cloudy",
    "Cloudy",
    "Overcast",
    "Light Rain",
    "Rain",
    "Heavy Rain",
    "Thunderstorm",
    "Drizzle",
    "Fog",
    "Mist",
    "Haze",
    "Snow",
    "Light Snow",
    "Heavy Snow",
    "Sleet",
    "Freezing Rain",
    "Hail",
    "Windy",
    "Breezy",
    "Clear",
    "Fair",
];

/// Weather descriptions.
static DESCRIPTIONS: &[&str] = &[
    "Expect clear skies throughout the day",
    "Scattered clouds with occasional sunshine",
    "Overcast with a chance of precipitation",
    "Light showers expected in the afternoon",
    "Thunderstorms likely later today",
    "Fog clearing by mid-morning",
    "Snow accumulation of 2-4 inches expected",
    "Warm and humid conditions",
    "Cool and breezy with gusts up to 20 mph",
    "Perfect weather for outdoor activities",
    "Stay indoors due to severe weather warnings",
    "Gradually improving conditions expected",
];

/// Wind directions.
static WIND_DIRECTIONS: &[&str] = &[
    "N", "NE", "E", "SE", "S", "SW", "W", "NW", "NNE", "ENE", "ESE", "SSE", "SSW", "WSW", "WNW",
    "NNW",
];

/// Seasons.
static SEASONS: &[&str] = &["Spring", "Summer", "Fall", "Winter"];

/// UV index levels.
static UV_LEVELS: &[&str] = &["Low", "Moderate", "High", "Very High", "Extreme"];

/// Air quality levels.
static AIR_QUALITY: &[&str] = &[
    "Good",
    "Moderate",
    "Unhealthy for Sensitive Groups",
    "Unhealthy",
    "Very Unhealthy",
    "Hazardous",
];

/// Generate a random weather condition.
pub fn condition<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, CONDITIONS)
}

/// Generate a random weather description.
pub fn description<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, DESCRIPTIONS)
}

/// Generate a random temperature in Fahrenheit.
pub fn temperature_f<R: ?Sized + Rng>(rng: &mut R) -> i8 {
    rng.random_range(-20..110)
}

/// Generate a random temperature in Celsius.
pub fn temperature_c<R: ?Sized + Rng>(rng: &mut R) -> i8 {
    rng.random_range(-30..45)
}

/// Generate a temperature in Fahrenheit for a specific season.
pub fn temperature_f_season<R: ?Sized + Rng>(rng: &mut R, season: &str) -> i8 {
    match season.to_lowercase().as_str() {
        "winter" => rng.random_range(10..40),
        "spring" => rng.random_range(45..70),
        "summer" => rng.random_range(70..100),
        "fall" | "autumn" => rng.random_range(40..65),
        _ => rng.random_range(30..80),
    }
}

/// Generate a random humidity percentage.
pub fn humidity<R: ?Sized + Rng>(rng: &mut R) -> u8 {
    rng.random_range(20..100)
}

/// Generate a random wind speed in mph.
pub fn wind_speed_mph<R: ?Sized + Rng>(rng: &mut R) -> u8 {
    rng.random_range(0..50)
}

/// Generate a random wind speed in km/h.
pub fn wind_speed_kmh<R: ?Sized + Rng>(rng: &mut R) -> u8 {
    rng.random_range(0..80)
}

/// Generate a random wind direction.
pub fn wind_direction<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, WIND_DIRECTIONS)
}

/// Generate a random atmospheric pressure in millibars.
pub fn pressure_mb<R: ?Sized + Rng>(rng: &mut R) -> u16 {
    rng.random_range(980..1040)
}

/// Generate a random visibility in miles.
pub fn visibility_miles<R: ?Sized + Rng>(rng: &mut R) -> u8 {
    rng.random_range(1..15)
}

/// Generate a random UV index.
pub fn uv_index<R: ?Sized + Rng>(rng: &mut R) -> u8 {
    rng.random_range(0..12)
}

/// Generate a random UV level description.
pub fn uv_level<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, UV_LEVELS)
}

/// Generate a random season.
pub fn season<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, SEASONS)
}

/// Generate a random air quality description.
pub fn air_quality<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, AIR_QUALITY)
}

/// Generate a random precipitation chance percentage.
pub fn precipitation_chance<R: ?Sized + Rng>(rng: &mut R) -> u8 {
    // Round to nearest 10
    (rng.random_range(0..11) * 10) as u8
}

/// Generate a weather forecast summary.
pub fn forecast_summary<R: ?Sized + Rng>(rng: &mut R) -> String {
    let cond = condition(rng);
    let temp_high = rng.random_range(50..90);
    let temp_low = temp_high - rng.random_range(10..25);
    let precip = precipitation_chance(rng);

    format!(
        "{}, High {}°F, Low {}°F, {}% chance of precipitation",
        cond, temp_high, temp_low, precip
    )
}

/// Generate a random cloud coverage percentage.
pub fn cloud_coverage<R: ?Sized + Rng>(rng: &mut R) -> u8 {
    rng.random_range(0..101)
}

/// Generate a random dew point in Fahrenheit.
pub fn dew_point_f<R: ?Sized + Rng>(rng: &mut R) -> i8 {
    rng.random_range(20..75)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_condition() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = condition(&mut rng);
        assert!(CONDITIONS.contains(&c));
    }

    #[test]
    fn test_temperature_f() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = temperature_f(&mut rng);
        assert!(t >= -20 && t < 110);
    }

    #[test]
    fn test_temperature_c() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = temperature_c(&mut rng);
        assert!(t >= -30 && t < 45);
    }

    #[test]
    fn test_humidity() {
        let mut rng = StdRng::seed_from_u64(42);
        let h = humidity(&mut rng);
        assert!(h >= 20 && h < 100);
    }

    #[test]
    fn test_wind_direction() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = wind_direction(&mut rng);
        assert!(WIND_DIRECTIONS.contains(&d));
    }

    #[test]
    fn test_forecast_summary() {
        let mut rng = StdRng::seed_from_u64(42);
        let f = forecast_summary(&mut rng);
        assert!(f.contains("High"));
        assert!(f.contains("Low"));
    }

    #[test]
    fn test_season_temperature() {
        let mut rng = StdRng::seed_from_u64(42);
        let winter_temp = temperature_f_season(&mut rng, "winter");
        assert!(winter_temp >= 10 && winter_temp < 40);

        let summer_temp = temperature_f_season(&mut rng, "summer");
        assert!(summer_temp >= 70 && summer_temp < 100);
    }
}
