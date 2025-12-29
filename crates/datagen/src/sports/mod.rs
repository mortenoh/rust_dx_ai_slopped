//! Sports data generators.
//!
//! Provides generators for sports-related fake data including sports,
//! teams, leagues, positions, and events.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Sports.
static SPORTS: &[&str] = &[
    "Football",
    "Basketball",
    "Baseball",
    "Soccer",
    "Hockey",
    "Tennis",
    "Golf",
    "Boxing",
    "MMA",
    "Wrestling",
    "Swimming",
    "Track and Field",
    "Volleyball",
    "Rugby",
    "Cricket",
    "Badminton",
    "Table Tennis",
    "Gymnastics",
    "Figure Skating",
    "Skiing",
    "Snowboarding",
    "Surfing",
    "Skateboarding",
    "Cycling",
    "Motorsport",
];

/// Team name patterns.
static TEAM_PATTERNS: &[&str] = &[
    "{city} {mascot}",
    "{city} {mascot}s",
    "FC {city}",
    "{city} United",
    "{city} City",
    "Real {city}",
    "{mascot}s",
];

static TEAM_CITIES: &[&str] = &[
    "New York",
    "Los Angeles",
    "Chicago",
    "Houston",
    "Phoenix",
    "Philadelphia",
    "San Antonio",
    "San Diego",
    "Dallas",
    "San Jose",
    "Austin",
    "Jacksonville",
    "San Francisco",
    "Columbus",
    "Charlotte",
    "Indianapolis",
    "Seattle",
    "Denver",
    "Boston",
    "Nashville",
];

static MASCOTS: &[&str] = &[
    "Eagles",
    "Tigers",
    "Lions",
    "Bears",
    "Wolves",
    "Hawks",
    "Panthers",
    "Sharks",
    "Dragons",
    "Knights",
    "Warriors",
    "Giants",
    "Titans",
    "Kings",
    "Royals",
    "Thunder",
    "Storm",
    "Lightning",
    "Fire",
    "Heat",
];

/// Leagues.
static LEAGUES: &[&str] = &[
    "NFL",
    "NBA",
    "MLB",
    "NHL",
    "MLS",
    "Premier League",
    "La Liga",
    "Serie A",
    "Bundesliga",
    "Ligue 1",
    "Champions League",
    "World Cup",
    "Olympics",
    "UFC",
    "WWE",
    "PGA Tour",
    "ATP Tour",
    "WTA Tour",
    "Formula 1",
    "NASCAR",
];

/// Player positions (generic).
static POSITIONS: &[&str] = &[
    "Quarterback",
    "Running Back",
    "Wide Receiver",
    "Tight End",
    "Linebacker",
    "Point Guard",
    "Shooting Guard",
    "Small Forward",
    "Power Forward",
    "Center",
    "Pitcher",
    "Catcher",
    "Shortstop",
    "Outfielder",
    "Goalkeeper",
    "Defender",
    "Midfielder",
    "Forward",
    "Striker",
    "Winger",
];

/// Tournament/event types.
static TOURNAMENTS: &[&str] = &[
    "Championship",
    "World Series",
    "Super Bowl",
    "Finals",
    "Playoffs",
    "Cup Final",
    "Grand Slam",
    "Masters",
    "Open",
    "Classic",
    "Invitational",
    "Tournament",
    "League",
    "Conference",
    "Division",
];

/// Generate a random sport.
pub fn sport<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, SPORTS)
}

/// Generate a random team name.
pub fn team_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, TEAM_PATTERNS);
    pattern
        .replace("{city}", pick(rng, TEAM_CITIES))
        .replace("{mascot}", pick(rng, MASCOTS))
}

/// Generate a random league name.
pub fn league<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, LEAGUES)
}

/// Generate a random mascot.
pub fn mascot<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, MASCOTS)
}

/// Generate a random player position.
pub fn position<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, POSITIONS)
}

/// Generate a random tournament/event name.
pub fn tournament<R: ?Sized + Rng>(rng: &mut R) -> String {
    let year: u16 = rng.random_range(2020..2030);
    let event = pick(rng, TOURNAMENTS);
    format!("{} {}", year, event)
}

/// Generate a random score (e.g., "3-2").
pub fn score<R: ?Sized + Rng>(rng: &mut R) -> String {
    let home: u8 = rng.random_range(0..10);
    let away: u8 = rng.random_range(0..10);
    format!("{}-{}", home, away)
}

/// Generate a random championship name.
pub fn championship<R: ?Sized + Rng>(rng: &mut R) -> String {
    let league = league(rng);
    let tournaments = ["Championship", "Finals", "Cup", "Trophy", "Title"];
    format!("{} {}", league, pick(rng, &tournaments))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_sport() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = sport(&mut rng);
        assert!(SPORTS.contains(&s));
    }

    #[test]
    fn test_team_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = team_name(&mut rng);
        assert!(!t.is_empty());
        assert!(!t.contains('{'));
    }

    #[test]
    fn test_league() {
        let mut rng = StdRng::seed_from_u64(42);
        let l = league(&mut rng);
        assert!(LEAGUES.contains(&l));
    }

    #[test]
    fn test_score() {
        let mut rng = StdRng::seed_from_u64(42);
        let s = score(&mut rng);
        assert!(s.contains('-'));
    }

    #[test]
    fn test_tournament() {
        let mut rng = StdRng::seed_from_u64(42);
        let t = tournament(&mut rng);
        assert!(!t.is_empty());
    }
}
