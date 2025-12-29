//! Japanese locale data (ja_JP).
//!
//! Provides Japanese-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// Japanese locale marker type.
pub struct JaJp;

/// Common male first names in Japan.
pub const MALE_FIRST_NAMES: &[&str] = &[
    "Haruto", "Yuto", "Sota", "Yuki", "Hayato", "Haruki", "Ren", "Riku", "Kota", "Asahi", "Minato",
    "Ryusei", "Kaito", "Sora", "Yamato", "Takumi", "Hinata", "Daiki", "Sosuke", "Tsubasa", "Kenta",
    "Shota", "Kenji", "Takeshi", "Masato", "Ryota", "Naoki", "Yusuke", "Daisuke", "Akira",
    "Hiroshi", "Kazuki", "Satoshi", "Makoto", "Tetsuya", "Koji", "Shin", "Ryo", "Ken", "Taro",
];

/// Common female first names in Japan.
pub const FEMALE_FIRST_NAMES: &[&str] = &[
    "Yui", "Hana", "Sakura", "Aoi", "Rin", "Koharu", "Yuna", "Mei", "Mio", "Himari", "Yuzuki",
    "Rio", "Sara", "Mana", "Emma", "Saki", "Akari", "Riko", "Nana", "Miyu", "Ayaka", "Misaki",
    "Haruka", "Yuki", "Aya", "Mai", "Kaori", "Emi", "Yoko", "Keiko", "Tomoko", "Naomi", "Michiko",
    "Chieko", "Megumi", "Kumiko", "Noriko", "Sachiko", "Reiko", "Mariko",
];

/// Common last names in Japan.
pub const LAST_NAMES: &[&str] = &[
    "Sato",
    "Suzuki",
    "Takahashi",
    "Tanaka",
    "Watanabe",
    "Ito",
    "Yamamoto",
    "Nakamura",
    "Kobayashi",
    "Kato",
    "Yoshida",
    "Yamada",
    "Sasaki",
    "Yamaguchi",
    "Matsumoto",
    "Inoue",
    "Kimura",
    "Hayashi",
    "Shimizu",
    "Yamazaki",
    "Mori",
    "Abe",
    "Ikeda",
    "Hashimoto",
    "Ishikawa",
    "Yamashita",
    "Ogawa",
    "Ishii",
    "Hasegawa",
    "Goto",
    "Okada",
    "Kondo",
    "Murata",
    "Fujita",
    "Endo",
    "Aoki",
    "Sakamoto",
    "Fukuda",
    "Nishimura",
    "Fujii",
    "Miura",
    "Okamoto",
    "Matsuda",
    "Nakagawa",
    "Nakano",
    "Harada",
    "Ono",
    "Tamura",
    "Takeuchi",
    "Kaneko",
];

/// Japanese cities.
pub const CITIES: &[&str] = &[
    "Tokyo",
    "Yokohama",
    "Osaka",
    "Nagoya",
    "Sapporo",
    "Fukuoka",
    "Kobe",
    "Kawasaki",
    "Kyoto",
    "Saitama",
    "Hiroshima",
    "Sendai",
    "Chiba",
    "Kitakyushu",
    "Sakai",
    "Niigata",
    "Hamamatsu",
    "Kumamoto",
    "Sagamihara",
    "Okayama",
    "Shizuoka",
    "Hachioji",
    "Funabashi",
    "Kagoshima",
    "Kawaguchi",
    "Gifu",
    "Himeji",
    "Matsuyama",
    "Higashiosaka",
    "Nishinomiya",
];

/// Japanese prefectures.
pub const PREFECTURES: &[&str] = &[
    "Hokkaido",
    "Aomori",
    "Iwate",
    "Miyagi",
    "Akita",
    "Yamagata",
    "Fukushima",
    "Ibaraki",
    "Tochigi",
    "Gunma",
    "Saitama",
    "Chiba",
    "Tokyo",
    "Kanagawa",
    "Niigata",
    "Toyama",
    "Ishikawa",
    "Fukui",
    "Yamanashi",
    "Nagano",
    "Gifu",
    "Shizuoka",
    "Aichi",
    "Mie",
    "Shiga",
    "Kyoto",
    "Osaka",
    "Hyogo",
    "Nara",
    "Wakayama",
    "Tottori",
    "Shimane",
    "Okayama",
    "Hiroshima",
    "Yamaguchi",
    "Tokushima",
    "Kagawa",
    "Ehime",
    "Kochi",
    "Fukuoka",
    "Saga",
    "Nagasaki",
    "Kumamoto",
    "Oita",
    "Miyazaki",
    "Kagoshima",
    "Okinawa",
];

/// Street suffixes in Japanese addresses.
pub const STREET_SUFFIXES: &[&str] = &[
    "dori", "cho", "machi", "oji", "zaka", "bashi", "mae", "eki", "koen", "hama",
];

/// Get a random first name (male or female).
pub fn first_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    if rng.random_bool(0.5) {
        first_name_male(rng)
    } else {
        first_name_female(rng)
    }
}

/// Get a random male first name.
pub fn first_name_male<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    MALE_FIRST_NAMES[rng.random_range(0..MALE_FIRST_NAMES.len())]
}

/// Get a random female first name.
pub fn first_name_female<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    FEMALE_FIRST_NAMES[rng.random_range(0..FEMALE_FIRST_NAMES.len())]
}

/// Get a random last name.
pub fn last_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    LAST_NAMES[rng.random_range(0..LAST_NAMES.len())]
}

/// Generate a full name (Japanese order: last name first).
pub fn full_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} {}", last_name(rng), first_name(rng))
}

/// Generate a Japanese phone number in 0XX-XXXX-XXXX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let area_codes = [
        "03", "06", "011", "045", "052", "078", "092", "082", "022", "075",
    ];
    let area = area_codes[rng.random_range(0..area_codes.len())];
    let local1 = rng.random_range(1000..10000);
    let local2 = rng.random_range(1000..10000);
    format!("{}-{:04}-{:04}", area, local1, local2)
}

/// Generate a Japanese mobile phone number.
pub fn mobile_phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefixes = ["070", "080", "090"];
    let prefix = prefixes[rng.random_range(0..prefixes.len())];
    let local1 = rng.random_range(1000..10000);
    let local2 = rng.random_range(1000..10000);
    format!("{}-{:04}-{:04}", prefix, local1, local2)
}

/// Get a random city.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    CITIES[rng.random_range(0..CITIES.len())]
}

/// Get a random prefecture.
pub fn prefecture<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    PREFECTURES[rng.random_range(0..PREFECTURES.len())]
}

/// Get a random street suffix.
pub fn street_suffix<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    STREET_SUFFIXES[rng.random_range(0..STREET_SUFFIXES.len())]
}

/// Generate a street address.
pub fn street_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let chome = rng.random_range(1..10);
    let banchi = rng.random_range(1..30);
    let go = rng.random_range(1..20);
    format!("{}-{}-{}", chome, banchi, go)
}

/// Generate a postal code (Japanese format: XXX-XXXX).
pub fn postal_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!(
        "{:03}-{:04}",
        rng.random_range(100..999),
        rng.random_range(0..9999)
    )
}

/// Generate a full address.
pub fn full_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let pref = prefecture(rng);
    let city_name = city(rng);
    let street = street_address(rng);
    let postal = postal_code(rng);
    format!("〒{} {}{} {}", postal, pref, city_name, street)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_first_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = first_name(&mut rng);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_last_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = last_name(&mut rng);
        assert!(LAST_NAMES.contains(&name));
    }

    #[test]
    fn test_phone() {
        let mut rng = StdRng::seed_from_u64(42);
        let phone_num = phone(&mut rng);
        assert!(phone_num.contains('-'));
    }

    #[test]
    fn test_postal_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let code = postal_code(&mut rng);
        assert!(code.contains('-'));
        assert_eq!(code.len(), 8);
    }
}
