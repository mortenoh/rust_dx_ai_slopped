//! Travel and transportation data generators.
//!
//! Provides generators for travel-related fake data including airlines,
//! airports, hotels, and destinations.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Airlines.
static AIRLINES: &[&str] = &[
    "American Airlines",
    "Delta Air Lines",
    "United Airlines",
    "Southwest Airlines",
    "JetBlue Airways",
    "Alaska Airlines",
    "Spirit Airlines",
    "Frontier Airlines",
    "Hawaiian Airlines",
    "British Airways",
    "Lufthansa",
    "Air France",
    "KLM",
    "Emirates",
    "Qatar Airways",
    "Singapore Airlines",
    "Cathay Pacific",
    "Japan Airlines",
    "ANA",
    "Qantas",
    "Air Canada",
    "Turkish Airlines",
    "Etihad Airways",
    "Virgin Atlantic",
    "Iberia",
];

/// Airport codes (IATA).
static AIRPORT_CODES: &[&str] = &[
    "JFK", "LAX", "ORD", "ATL", "DFW", "DEN", "SFO", "SEA", "MIA", "BOS", "LHR", "CDG", "FRA",
    "AMS", "DXB", "SIN", "HKG", "NRT", "ICN", "SYD", "YYZ", "PEK", "PVG", "IST", "MAD",
];

/// Airport names.
static AIRPORT_NAMES: &[&str] = &[
    "John F. Kennedy International",
    "Los Angeles International",
    "O'Hare International",
    "Hartsfield-Jackson Atlanta International",
    "Dallas/Fort Worth International",
    "Denver International",
    "San Francisco International",
    "Seattle-Tacoma International",
    "Miami International",
    "Boston Logan International",
    "London Heathrow",
    "Paris Charles de Gaulle",
    "Frankfurt Airport",
    "Amsterdam Schiphol",
    "Dubai International",
    "Singapore Changi",
    "Hong Kong International",
    "Tokyo Narita",
    "Incheon International",
    "Sydney Kingsford Smith",
];

/// Aircraft types.
static AIRCRAFT_TYPES: &[&str] = &[
    "Boeing 737",
    "Boeing 747",
    "Boeing 757",
    "Boeing 767",
    "Boeing 777",
    "Boeing 787 Dreamliner",
    "Airbus A320",
    "Airbus A321",
    "Airbus A330",
    "Airbus A340",
    "Airbus A350",
    "Airbus A380",
    "Embraer E175",
    "Embraer E190",
    "Bombardier CRJ900",
    "ATR 72",
];

/// Seat classes.
static SEAT_CLASSES: &[&str] = &["Economy", "Premium Economy", "Business", "First Class"];

/// Hotel chains.
static HOTEL_CHAINS: &[&str] = &[
    "Marriott",
    "Hilton",
    "Hyatt",
    "InterContinental",
    "Sheraton",
    "Westin",
    "Four Seasons",
    "Ritz-Carlton",
    "Fairmont",
    "Radisson",
    "Best Western",
    "Holiday Inn",
    "Hampton Inn",
    "DoubleTree",
    "Embassy Suites",
    "Courtyard",
    "Residence Inn",
    "W Hotels",
    "St. Regis",
    "Waldorf Astoria",
    "Mandarin Oriental",
    "Peninsula",
    "Park Hyatt",
    "Conrad",
    "Sofitel",
];

/// Room types.
static ROOM_TYPES: &[&str] = &[
    "Standard Room",
    "Deluxe Room",
    "Superior Room",
    "Executive Room",
    "Junior Suite",
    "Suite",
    "Executive Suite",
    "Presidential Suite",
    "Penthouse",
    "Studio",
    "Family Room",
    "Connecting Rooms",
    "Accessible Room",
    "Ocean View",
    "City View",
];

/// Landmarks/attractions.
static LANDMARKS: &[&str] = &[
    "Eiffel Tower",
    "Colosseum",
    "Great Wall of China",
    "Machu Picchu",
    "Taj Mahal",
    "Statue of Liberty",
    "Big Ben",
    "Sydney Opera House",
    "Christ the Redeemer",
    "Pyramids of Giza",
    "Stonehenge",
    "Angkor Wat",
    "Petra",
    "Acropolis",
    "Sagrada Familia",
    "Burj Khalifa",
    "Golden Gate Bridge",
    "Grand Canyon",
    "Niagara Falls",
    "Mount Fuji",
    "Great Barrier Reef",
    "Northern Lights",
    "Victoria Falls",
    "Santorini",
    "Venice Canals",
];

/// Travel destinations.
static DESTINATIONS: &[&str] = &[
    "Paris, France",
    "London, UK",
    "New York City, USA",
    "Tokyo, Japan",
    "Rome, Italy",
    "Barcelona, Spain",
    "Amsterdam, Netherlands",
    "Sydney, Australia",
    "Dubai, UAE",
    "Singapore",
    "Hong Kong",
    "Bangkok, Thailand",
    "Bali, Indonesia",
    "Maldives",
    "Santorini, Greece",
    "Cancun, Mexico",
    "Miami, USA",
    "Las Vegas, USA",
    "Hawaii, USA",
    "Iceland",
    "New Zealand",
    "Cape Town, South Africa",
    "Rio de Janeiro, Brazil",
    "Istanbul, Turkey",
    "Prague, Czech Republic",
];

/// Generate a random airline name.
pub fn airline<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, AIRLINES)
}

/// Generate a random flight number.
pub fn flight_number<R: ?Sized + Rng>(rng: &mut R) -> String {
    let airline_code = match rng.random_range(0..5) {
        0 => "AA",
        1 => "DL",
        2 => "UA",
        3 => "SW",
        _ => "BA",
    };
    let number: u16 = rng.random_range(100..9999);
    format!("{}{}", airline_code, number)
}

/// Generate a random airport code (IATA).
pub fn airport_code<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, AIRPORT_CODES)
}

/// Generate a random airport name.
pub fn airport_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, AIRPORT_NAMES)
}

/// Generate a random aircraft type.
pub fn aircraft_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, AIRCRAFT_TYPES)
}

/// Generate a random seat assignment (e.g., "12A").
pub fn seat<R: ?Sized + Rng>(rng: &mut R) -> String {
    let row: u8 = rng.random_range(1..40);
    let letter = ['A', 'B', 'C', 'D', 'E', 'F'][rng.random_range(0..6)];
    format!("{}{}", row, letter)
}

/// Generate a random seat class.
pub fn seat_class<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, SEAT_CLASSES)
}

/// Generate a random hotel chain name.
pub fn hotel_chain<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, HOTEL_CHAINS)
}

/// Generate a random hotel name.
pub fn hotel_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    let chain = hotel_chain(rng);
    let locations = [
        "Downtown",
        "Airport",
        "Beach Resort",
        "City Center",
        "Convention Center",
        "Waterfront",
        "Garden",
        "Plaza",
    ];
    format!("{} {}", chain, pick(rng, &locations))
}

/// Generate a random room type.
pub fn room_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, ROOM_TYPES)
}

/// Generate a random landmark.
pub fn landmark<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, LANDMARKS)
}

/// Generate a random travel destination.
pub fn destination<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, DESTINATIONS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_airline() {
        let mut rng = StdRng::seed_from_u64(42);
        let a = airline(&mut rng);
        assert!(AIRLINES.contains(&a));
    }

    #[test]
    fn test_flight_number() {
        let mut rng = StdRng::seed_from_u64(42);
        let f = flight_number(&mut rng);
        assert!(f.len() >= 4);
    }

    #[test]
    fn test_airport_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = airport_code(&mut rng);
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn test_seat() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = seat(&mut rng);
        assert!(s.len() >= 2);
    }

    #[test]
    fn test_hotel_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let h = hotel_name(&mut rng);
        assert!(!h.is_empty());
    }
}
