//! Animal data generators.
//!
//! Provides generators for animal-related fake data including various species,
//! breeds, and pet names.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// General animals.
static ANIMALS: &[&str] = &[
    "Dog",
    "Cat",
    "Horse",
    "Cow",
    "Pig",
    "Sheep",
    "Goat",
    "Chicken",
    "Duck",
    "Rabbit",
    "Lion",
    "Tiger",
    "Bear",
    "Wolf",
    "Fox",
    "Deer",
    "Elephant",
    "Giraffe",
    "Zebra",
    "Monkey",
    "Gorilla",
    "Chimpanzee",
    "Kangaroo",
    "Koala",
    "Panda",
    "Penguin",
    "Dolphin",
    "Whale",
    "Shark",
    "Eagle",
];

/// Dog breeds.
static DOG_BREEDS: &[&str] = &[
    "Labrador Retriever",
    "German Shepherd",
    "Golden Retriever",
    "French Bulldog",
    "Bulldog",
    "Poodle",
    "Beagle",
    "Rottweiler",
    "German Pointer",
    "Dachshund",
    "Pembroke Welsh Corgi",
    "Australian Shepherd",
    "Yorkshire Terrier",
    "Boxer",
    "Cavalier King Charles Spaniel",
    "Doberman Pinscher",
    "Great Dane",
    "Miniature Schnauzer",
    "Siberian Husky",
    "Bernese Mountain Dog",
    "Shih Tzu",
    "Boston Terrier",
    "Pomeranian",
    "Havanese",
    "Shetland Sheepdog",
    "Border Collie",
    "Cocker Spaniel",
    "Basset Hound",
    "Dalmatian",
    "Samoyed",
];

/// Cat breeds.
static CAT_BREEDS: &[&str] = &[
    "Persian",
    "Maine Coon",
    "Ragdoll",
    "British Shorthair",
    "Exotic Shorthair",
    "Abyssinian",
    "Scottish Fold",
    "Sphynx",
    "Siamese",
    "Devon Rex",
    "Bengal",
    "Russian Blue",
    "Norwegian Forest Cat",
    "Birman",
    "Oriental Shorthair",
    "American Shorthair",
    "Burmese",
    "Ragamuffin",
    "Himalayan",
    "Tonkinese",
    "Chartreux",
    "Turkish Angora",
    "Cornish Rex",
    "Somali",
    "Savannah",
];

/// Birds.
static BIRDS: &[&str] = &[
    "Eagle",
    "Hawk",
    "Falcon",
    "Owl",
    "Parrot",
    "Parakeet",
    "Cockatoo",
    "Macaw",
    "Canary",
    "Finch",
    "Robin",
    "Sparrow",
    "Blue Jay",
    "Cardinal",
    "Crow",
    "Raven",
    "Hummingbird",
    "Woodpecker",
    "Penguin",
    "Flamingo",
    "Pelican",
    "Seagull",
    "Duck",
    "Goose",
    "Swan",
    "Turkey",
    "Peacock",
    "Ostrich",
    "Emu",
    "Toucan",
];

/// Fish.
static FISH: &[&str] = &[
    "Goldfish",
    "Betta",
    "Guppy",
    "Angelfish",
    "Tetra",
    "Molly",
    "Platy",
    "Swordtail",
    "Cichlid",
    "Discus",
    "Oscar",
    "Koi",
    "Clownfish",
    "Tang",
    "Damselfish",
    "Barb",
    "Catfish",
    "Pleco",
    "Loach",
    "Rasbora",
    "Gourami",
    "Arowana",
    "Piranha",
    "Pufferfish",
    "Lionfish",
];

/// Insects.
static INSECTS: &[&str] = &[
    "Ant",
    "Bee",
    "Wasp",
    "Butterfly",
    "Moth",
    "Beetle",
    "Ladybug",
    "Dragonfly",
    "Grasshopper",
    "Cricket",
    "Mosquito",
    "Fly",
    "Firefly",
    "Caterpillar",
    "Cockroach",
    "Termite",
    "Mantis",
    "Cicada",
    "Spider",
    "Scorpion",
];

/// Reptiles.
static REPTILES: &[&str] = &[
    "Snake",
    "Lizard",
    "Gecko",
    "Iguana",
    "Chameleon",
    "Turtle",
    "Tortoise",
    "Crocodile",
    "Alligator",
    "Komodo Dragon",
    "Monitor Lizard",
    "Bearded Dragon",
    "Ball Python",
    "Boa Constrictor",
    "Corn Snake",
    "King Cobra",
    "Rattlesnake",
    "Sea Turtle",
    "Box Turtle",
    "Anole",
];

/// Mammals.
static MAMMALS: &[&str] = &[
    "Dog",
    "Cat",
    "Lion",
    "Tiger",
    "Bear",
    "Wolf",
    "Elephant",
    "Giraffe",
    "Zebra",
    "Horse",
    "Cow",
    "Pig",
    "Sheep",
    "Goat",
    "Deer",
    "Moose",
    "Buffalo",
    "Rhino",
    "Hippo",
    "Gorilla",
    "Chimpanzee",
    "Monkey",
    "Koala",
    "Kangaroo",
    "Panda",
    "Sloth",
    "Bat",
    "Whale",
    "Dolphin",
    "Seal",
];

/// Pet names.
static PET_NAMES: &[&str] = &[
    "Max", "Bella", "Charlie", "Lucy", "Cooper", "Luna", "Buddy", "Daisy", "Rocky", "Sadie",
    "Bear", "Molly", "Duke", "Bailey", "Tucker", "Maggie", "Jack", "Sophie", "Oliver", "Chloe",
    "Riley", "Zoe", "Toby", "Lily", "Winston", "Gracie", "Murphy", "Penny", "Oscar", "Coco",
    "Milo", "Nala", "Zeus", "Stella", "Teddy", "Rosie", "Leo", "Ruby", "Finn", "Lola",
];

/// Generate a random animal.
pub fn animal<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, ANIMALS)
}

/// Generate a random dog breed.
pub fn dog_breed<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, DOG_BREEDS)
}

/// Generate a random cat breed.
pub fn cat_breed<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, CAT_BREEDS)
}

/// Generate a random bird.
pub fn bird<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, BIRDS)
}

/// Generate a random fish.
pub fn fish<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, FISH)
}

/// Generate a random insect.
pub fn insect<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, INSECTS)
}

/// Generate a random reptile.
pub fn reptile<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, REPTILES)
}

/// Generate a random mammal.
pub fn mammal<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, MAMMALS)
}

/// Generate a random pet name.
pub fn pet_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, PET_NAMES)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_animal() {
        let mut rng = StdRng::seed_from_u64(42);
        let a = animal(&mut rng);
        assert!(ANIMALS.contains(&a));
    }

    #[test]
    fn test_dog_breed() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = dog_breed(&mut rng);
        assert!(DOG_BREEDS.contains(&d));
    }

    #[test]
    fn test_cat_breed() {
        let mut rng = StdRng::seed_from_u64(42);
        let c = cat_breed(&mut rng);
        assert!(CAT_BREEDS.contains(&c));
    }

    #[test]
    fn test_bird() {
        let mut rng = StdRng::seed_from_u64(42);
        let b = bird(&mut rng);
        assert!(BIRDS.contains(&b));
    }

    #[test]
    fn test_fish() {
        let mut rng = StdRng::seed_from_u64(42);
        let f = fish(&mut rng);
        assert!(FISH.contains(&f));
    }

    #[test]
    fn test_insect() {
        let mut rng = StdRng::seed_from_u64(42);
        let i = insect(&mut rng);
        assert!(INSECTS.contains(&i));
    }

    #[test]
    fn test_reptile() {
        let mut rng = StdRng::seed_from_u64(42);
        let r = reptile(&mut rng);
        assert!(REPTILES.contains(&r));
    }

    #[test]
    fn test_mammal() {
        let mut rng = StdRng::seed_from_u64(42);
        let m = mammal(&mut rng);
        assert!(MAMMALS.contains(&m));
    }

    #[test]
    fn test_pet_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let p = pet_name(&mut rng);
        assert!(PET_NAMES.contains(&p));
    }
}
