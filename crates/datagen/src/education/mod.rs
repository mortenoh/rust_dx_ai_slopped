//! Education-related data generators.
//!
//! Provides generators for education-related fake data including universities,
//! degrees, majors, and courses.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Universities.
static UNIVERSITIES: &[&str] = &[
    "Harvard University",
    "Stanford University",
    "MIT",
    "Yale University",
    "Princeton University",
    "Columbia University",
    "University of Chicago",
    "Duke University",
    "Northwestern University",
    "Cornell University",
    "University of Pennsylvania",
    "Brown University",
    "Dartmouth College",
    "UCLA",
    "UC Berkeley",
    "University of Michigan",
    "NYU",
    "Carnegie Mellon",
    "Georgia Tech",
    "Caltech",
    "Oxford University",
    "Cambridge University",
    "Imperial College London",
    "ETH Zurich",
    "University of Tokyo",
];

/// Degree types.
static DEGREE_TYPES: &[&str] = &[
    "Associate of Arts",
    "Associate of Science",
    "Bachelor of Arts",
    "Bachelor of Science",
    "Bachelor of Engineering",
    "Bachelor of Fine Arts",
    "Master of Arts",
    "Master of Science",
    "Master of Business Administration",
    "Master of Engineering",
    "Doctor of Philosophy",
    "Doctor of Medicine",
    "Juris Doctor",
    "Doctor of Education",
];

/// Academic majors.
static MAJORS: &[&str] = &[
    "Computer Science",
    "Mathematics",
    "Physics",
    "Chemistry",
    "Biology",
    "Engineering",
    "Economics",
    "Business Administration",
    "Psychology",
    "Sociology",
    "Political Science",
    "History",
    "English Literature",
    "Philosophy",
    "Art History",
    "Music",
    "Theater",
    "Communications",
    "Journalism",
    "Marketing",
    "Finance",
    "Accounting",
    "Nursing",
    "Medicine",
    "Law",
];

/// Academic subjects.
static SUBJECTS: &[&str] = &[
    "Mathematics",
    "Science",
    "English",
    "History",
    "Geography",
    "Art",
    "Music",
    "Physical Education",
    "Computer Science",
    "Foreign Languages",
    "Social Studies",
    "Economics",
    "Psychology",
    "Philosophy",
    "Literature",
];

/// Course level prefixes.
static COURSE_LEVELS: &[&str] = &[
    "Introduction to",
    "Fundamentals of",
    "Principles of",
    "Advanced",
    "Intermediate",
    "Applied",
    "Theoretical",
    "Modern",
    "Classical",
    "Contemporary",
];

/// Course topics.
static COURSE_TOPICS: &[&str] = &[
    "Algorithms",
    "Data Structures",
    "Machine Learning",
    "Artificial Intelligence",
    "Database Systems",
    "Operating Systems",
    "Computer Networks",
    "Software Engineering",
    "Discrete Mathematics",
    "Linear Algebra",
    "Calculus",
    "Statistics",
    "Economics",
    "Psychology",
    "Sociology",
    "Philosophy",
    "Ethics",
    "Literature",
    "World History",
    "Political Science",
];

/// Academic grades.
static GRADES: &[&str] = &[
    "A+", "A", "A-", "B+", "B", "B-", "C+", "C", "C-", "D+", "D", "D-", "F",
];

/// GPA values corresponding to letter grades.
#[allow(dead_code)]
static GPA_VALUES: &[(f32, f32)] = &[
    (4.0, 4.0), // A+
    (4.0, 4.0), // A
    (3.7, 3.7), // A-
    (3.3, 3.3), // B+
    (3.0, 3.0), // B
    (2.7, 2.7), // B-
    (2.3, 2.3), // C+
    (2.0, 2.0), // C
    (1.7, 1.7), // C-
    (1.3, 1.3), // D+
    (1.0, 1.0), // D
    (0.7, 0.7), // D-
    (0.0, 0.0), // F
];

/// Generate a random university name.
pub fn university<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, UNIVERSITIES)
}

/// Generate a random degree type.
pub fn degree_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, DEGREE_TYPES)
}

/// Generate a random academic major.
pub fn major<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, MAJORS)
}

/// Generate a random academic subject.
pub fn subject<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, SUBJECTS)
}

/// Generate a random course name.
pub fn course_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    let level = pick(rng, COURSE_LEVELS);
    let topic = pick(rng, COURSE_TOPICS);
    format!("{} {}", level, topic)
}

/// Generate a random course code.
pub fn course_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefixes = [
        "CS", "MATH", "PHYS", "CHEM", "BIO", "ECON", "PSYC", "ENG", "HIST", "PHIL",
    ];
    let prefix = pick(rng, &prefixes);
    let number: u16 = rng.random_range(100..500);
    format!("{}{}", prefix, number)
}

/// Generate a random letter grade.
pub fn grade<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, GRADES)
}

/// Generate a random GPA.
pub fn gpa<R: ?Sized + Rng>(rng: &mut R) -> f32 {
    // Generate with normal distribution around 3.0
    let base: f32 = rng.random_range(0.0..4.0);
    (base * 10.0).round() / 10.0 // Round to 1 decimal
}

/// Generate a random classroom.
pub fn classroom<R: ?Sized + Rng>(rng: &mut R) -> String {
    let buildings = ["Hall", "Building", "Center", "Annex", "Wing"];
    let building = pick(rng, &buildings);
    let building_letter = ['A', 'B', 'C', 'D', 'E'][rng.random_range(0..5)];
    let room_number: u16 = rng.random_range(100..400);
    format!("{} {} {}", building_letter, building, room_number)
}

/// Generate a random school name (K-12).
pub fn school_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    let names = [
        "Washington",
        "Lincoln",
        "Roosevelt",
        "Jefferson",
        "Kennedy",
        "Madison",
        "Hamilton",
        "Franklin",
        "Adams",
        "Monroe",
    ];
    let types = [
        "Elementary School",
        "Middle School",
        "High School",
        "Academy",
        "Preparatory School",
    ];
    format!("{} {}", pick(rng, &names), pick(rng, &types))
}

/// Generate a random degree with major.
pub fn degree<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} in {}", degree_type(rng), major(rng))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_university() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = university(&mut rng);
        assert!(UNIVERSITIES.contains(&u));
    }

    #[test]
    fn test_degree_type() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = degree_type(&mut rng);
        assert!(DEGREE_TYPES.contains(&d));
    }

    #[test]
    fn test_course_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = course_name(&mut rng);
        assert!(!c.is_empty());
    }

    #[test]
    fn test_course_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = course_code(&mut rng);
        assert!(c.len() >= 5);
    }

    #[test]
    fn test_gpa() {
        let mut rng = StdRng::seed_from_u64(42);
        let g = gpa(&mut rng);
        assert!(g >= 0.0 && g <= 4.0);
    }

    #[test]
    fn test_degree() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = degree(&mut rng);
        assert!(d.contains(" in "));
    }
}
