//! Healthcare and medical data generators.
//!
//! Provides generators for healthcare-related fake data including medical
//! conditions, medications, body parts, and healthcare providers.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Medical conditions/diseases.
static CONDITIONS: &[&str] = &[
    "Hypertension",
    "Type 2 Diabetes",
    "Asthma",
    "Arthritis",
    "Migraine",
    "Allergies",
    "Anxiety",
    "Depression",
    "Insomnia",
    "Acid Reflux",
    "Back Pain",
    "Chronic Fatigue",
    "Eczema",
    "High Cholesterol",
    "Hypothyroidism",
    "Anemia",
    "Bronchitis",
    "Sinusitis",
    "Osteoporosis",
    "Fibromyalgia",
    "Irritable Bowel Syndrome",
    "Carpal Tunnel Syndrome",
    "Tendinitis",
    "Vertigo",
    "Sleep Apnea",
];

/// Medications.
static MEDICATIONS: &[&str] = &[
    "Lisinopril",
    "Metformin",
    "Atorvastatin",
    "Amlodipine",
    "Omeprazole",
    "Losartan",
    "Albuterol",
    "Gabapentin",
    "Hydrochlorothiazide",
    "Sertraline",
    "Metoprolol",
    "Levothyroxine",
    "Prednisone",
    "Fluticasone",
    "Montelukast",
    "Escitalopram",
    "Pantoprazole",
    "Furosemide",
    "Tramadol",
    "Duloxetine",
    "Amoxicillin",
    "Azithromycin",
    "Ibuprofen",
    "Acetaminophen",
    "Aspirin",
];

/// Symptoms.
static SYMPTOMS: &[&str] = &[
    "Headache",
    "Fatigue",
    "Fever",
    "Cough",
    "Nausea",
    "Dizziness",
    "Chest Pain",
    "Shortness of Breath",
    "Joint Pain",
    "Back Pain",
    "Abdominal Pain",
    "Sore Throat",
    "Runny Nose",
    "Congestion",
    "Muscle Aches",
    "Loss of Appetite",
    "Insomnia",
    "Anxiety",
    "Sweating",
    "Chills",
    "Rash",
    "Swelling",
    "Numbness",
    "Tingling",
    "Blurred Vision",
];

/// Blood types.
static BLOOD_TYPES: &[&str] = &["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"];

/// Body organs.
static ORGANS: &[&str] = &[
    "Heart",
    "Lungs",
    "Liver",
    "Kidneys",
    "Brain",
    "Stomach",
    "Pancreas",
    "Spleen",
    "Gallbladder",
    "Small Intestine",
    "Large Intestine",
    "Bladder",
    "Thyroid",
    "Adrenal Glands",
    "Appendix",
];

/// Body parts.
static BODY_PARTS: &[&str] = &[
    "Head", "Neck", "Shoulder", "Arm", "Elbow", "Wrist", "Hand", "Finger", "Chest", "Back",
    "Abdomen", "Hip", "Leg", "Knee", "Ankle", "Foot", "Toe", "Spine", "Pelvis", "Rib",
];

/// Medical specialties.
static SPECIALTIES: &[&str] = &[
    "Cardiology",
    "Dermatology",
    "Endocrinology",
    "Gastroenterology",
    "Neurology",
    "Oncology",
    "Ophthalmology",
    "Orthopedics",
    "Pediatrics",
    "Psychiatry",
    "Pulmonology",
    "Radiology",
    "Rheumatology",
    "Urology",
    "Emergency Medicine",
    "Family Medicine",
    "Internal Medicine",
    "Obstetrics",
    "Gynecology",
    "Anesthesiology",
    "Pathology",
    "Surgery",
    "Plastic Surgery",
    "Nephrology",
    "Hematology",
];

/// Hospital name patterns.
static HOSPITAL_PATTERNS: &[&str] = &[
    "{city} General Hospital",
    "{city} Medical Center",
    "St. {saint}'s Hospital",
    "{name} Memorial Hospital",
    "{city} Regional Medical Center",
    "University of {city} Hospital",
    "{name} Healthcare",
    "{city} Community Hospital",
];

static CITIES: &[&str] = &[
    "Springfield",
    "Riverside",
    "Fairview",
    "Madison",
    "Georgetown",
    "Franklin",
    "Clinton",
    "Salem",
    "Bristol",
    "Chester",
];

static SAINTS: &[&str] = &[
    "Mary",
    "Joseph",
    "John",
    "Luke",
    "Francis",
    "Michael",
    "Anthony",
    "Vincent",
    "Thomas",
    "Elizabeth",
];

static MEMORIAL_NAMES: &[&str] = &[
    "Johns Hopkins",
    "Mayo",
    "Cleveland",
    "Mount Sinai",
    "Duke",
    "Stanford",
    "Northwestern",
    "Emory",
    "Cedars-Sinai",
    "Massachusetts General",
];

/// Doctor titles/prefixes.
static DOCTOR_TITLES: &[&str] = &["Dr.", "Dr.", "Dr.", "Dr.", "Prof. Dr.", "Assoc. Prof. Dr."];

/// Generate a random medical condition.
pub fn condition<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, CONDITIONS)
}

/// Generate a random medication name.
pub fn medication<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, MEDICATIONS)
}

/// Generate a random symptom.
pub fn symptom<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, SYMPTOMS)
}

/// Generate a random blood type.
pub fn blood_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, BLOOD_TYPES)
}

/// Generate a random organ.
pub fn organ<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, ORGANS)
}

/// Generate a random body part.
pub fn body_part<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, BODY_PARTS)
}

/// Generate a random medical specialty.
pub fn specialty<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, SPECIALTIES)
}

/// Generate a random hospital name.
pub fn hospital_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pattern = pick(rng, HOSPITAL_PATTERNS);
    pattern
        .replace("{city}", pick(rng, CITIES))
        .replace("{saint}", pick(rng, SAINTS))
        .replace("{name}", pick(rng, MEMORIAL_NAMES))
}

/// Generate a random doctor title.
pub fn doctor_title<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, DOCTOR_TITLES)
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
    fn test_medication() {
        let mut rng = StdRng::seed_from_u64(42);
        let m = medication(&mut rng);
        assert!(MEDICATIONS.contains(&m));
    }

    #[test]
    fn test_blood_type() {
        let mut rng = StdRng::seed_from_u64(42);
        let b = blood_type(&mut rng);
        assert!(BLOOD_TYPES.contains(&b));
    }

    #[test]
    fn test_hospital_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let h = hospital_name(&mut rng);
        assert!(!h.is_empty());
        assert!(!h.contains('{'));
    }
}
