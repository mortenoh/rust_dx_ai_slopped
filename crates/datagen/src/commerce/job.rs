//! Job and employment data generation.

use rand::Rng;

/// Job descriptors (prefixes)
pub const JOB_DESCRIPTORS: &[&str] = &[
    "Lead",
    "Senior",
    "Direct",
    "Corporate",
    "Dynamic",
    "Future",
    "Product",
    "National",
    "Regional",
    "District",
    "Central",
    "Global",
    "Principal",
    "Internal",
    "International",
    "Chief",
    "Junior",
    "Associate",
    "Staff",
    "Executive",
    "Head",
];

/// Job areas/domains
pub const JOB_AREAS: &[&str] = &[
    "Accountability",
    "Accounts",
    "Assurance",
    "Brand",
    "Communications",
    "Configuration",
    "Creative",
    "Data",
    "Directives",
    "Division",
    "Factors",
    "Functionality",
    "Group",
    "Identity",
    "Implementation",
    "Infrastructure",
    "Integration",
    "Intranet",
    "Marketing",
    "Metrics",
    "Mobility",
    "Operations",
    "Optimization",
    "Paradigm",
    "Program",
    "Quality",
    "Research",
    "Response",
    "Security",
    "Solutions",
    "Tactics",
    "Usability",
    "Web",
    "Applications",
    "Branding",
];

/// Job types
pub const JOB_TYPES: &[&str] = &[
    "Agent",
    "Analyst",
    "Architect",
    "Assistant",
    "Associate",
    "Consultant",
    "Coordinator",
    "Designer",
    "Developer",
    "Director",
    "Engineer",
    "Executive",
    "Facilitator",
    "Liaison",
    "Manager",
    "Officer",
    "Orchestrator",
    "Planner",
    "Producer",
    "Representative",
    "Specialist",
    "Strategist",
    "Supervisor",
    "Technician",
    "Administrator",
];

/// Departments
pub const DEPARTMENTS: &[&str] = &[
    "Engineering",
    "Marketing",
    "Sales",
    "Human Resources",
    "Finance",
    "Legal",
    "Operations",
    "Customer Support",
    "Research & Development",
    "IT",
    "Product",
    "Quality Assurance",
    "Procurement",
    "Logistics",
    "Administration",
    "Business Development",
    "Public Relations",
    "Design",
    "Accounting",
    "Security",
];

/// Generate a job title.
///
/// # Example
/// ```
/// use dx_datagen::commerce::job_title;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let title = job_title(&mut rng);
/// assert!(!title.is_empty());
/// ```
pub fn job_title<R: ?Sized + Rng>(rng: &mut R) -> String {
    let descriptor = JOB_DESCRIPTORS[rng.random_range(0..JOB_DESCRIPTORS.len())];
    let area = JOB_AREAS[rng.random_range(0..JOB_AREAS.len())];
    let job_type = JOB_TYPES[rng.random_range(0..JOB_TYPES.len())];
    format!("{} {} {}", descriptor, area, job_type)
}

/// Get a random job descriptor.
pub fn job_descriptor<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    JOB_DESCRIPTORS[rng.random_range(0..JOB_DESCRIPTORS.len())]
}

/// Get a random job area.
pub fn job_area<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    JOB_AREAS[rng.random_range(0..JOB_AREAS.len())]
}

/// Get a random job type.
pub fn job_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    JOB_TYPES[rng.random_range(0..JOB_TYPES.len())]
}

/// Get a random department.
///
/// # Example
/// ```
/// use dx_datagen::commerce::department;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let dept = department(&mut rng);
/// assert!(!dept.is_empty());
/// ```
pub fn department<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    DEPARTMENTS[rng.random_range(0..DEPARTMENTS.len())]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_job_title() {
        let mut rng = StdRng::seed_from_u64(42);
        let title = job_title(&mut rng);
        assert!(!title.is_empty());
        // Should have 3 words
        assert_eq!(title.split(' ').count(), 3);
    }

    #[test]
    fn test_job_descriptor() {
        let mut rng = StdRng::seed_from_u64(42);
        let desc = job_descriptor(&mut rng);
        assert!(JOB_DESCRIPTORS.contains(&desc));
    }

    #[test]
    fn test_department() {
        let mut rng = StdRng::seed_from_u64(42);
        let dept = department(&mut rng);
        assert!(DEPARTMENTS.contains(&dept));
    }

    #[test]
    fn test_determinism() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);

        assert_eq!(job_title(&mut rng1), job_title(&mut rng2));
    }

    #[test]
    fn test_trait_object() {
        use rand::RngCore;
        let mut rng: Box<dyn RngCore> = Box::new(StdRng::seed_from_u64(42));
        let title = job_title(&mut *rng);
        assert!(!title.is_empty());
    }
}
