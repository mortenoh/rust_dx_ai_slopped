//! Locale-specific data generation.
//!
//! Provides locale-aware data for names, addresses, phone numbers, and more.
//!
//! # Example
//!
//! ```
//! use dx_datagen::locale::{Locale, LocaleData};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//! let locale = Locale::EnUs;
//!
//! let first = locale.first_name(&mut rng);
//! let last = locale.last_name(&mut rng);
//! let phone = locale.phone(&mut rng);
//! ```

pub mod de_de;
pub mod en_us;
pub mod es_es;
pub mod fr_fr;
pub mod it_it;
pub mod ja_jp;
pub mod nl_nl;
pub mod no_no;
pub mod pt_br;
pub mod sv_se;
pub mod zh_cn;

use rand::Rng;

/// Supported locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locale {
    /// US English (default)
    #[default]
    EnUs,
    /// Norwegian (Bokmaal)
    NoNo,
    /// German (Germany)
    DeDe,
    /// French (France)
    FrFr,
    /// Spanish (Spain)
    EsEs,
    /// Japanese (Japan)
    JaJp,
    /// Chinese Simplified (China)
    ZhCn,
    /// Portuguese (Brazil)
    PtBr,
    /// Italian (Italy)
    ItIt,
    /// Dutch (Netherlands)
    NlNl,
    /// Swedish (Sweden)
    SvSe,
}

impl Locale {
    /// Get locale from string code (e.g., "en_US", "no_NO").
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().replace('-', "_").as_str() {
            "en_us" | "en" | "us" => Some(Locale::EnUs),
            "no_no" | "no" | "nb" | "nb_no" => Some(Locale::NoNo),
            "de_de" | "de" => Some(Locale::DeDe),
            "fr_fr" | "fr" => Some(Locale::FrFr),
            "es_es" | "es" => Some(Locale::EsEs),
            "ja_jp" | "ja" | "jp" => Some(Locale::JaJp),
            "zh_cn" | "zh" | "cn" => Some(Locale::ZhCn),
            "pt_br" | "pt" | "br" => Some(Locale::PtBr),
            "it_it" | "it" => Some(Locale::ItIt),
            "nl_nl" | "nl" => Some(Locale::NlNl),
            "sv_se" | "sv" | "se" => Some(Locale::SvSe),
            _ => None,
        }
    }

    /// Get the locale code.
    pub fn code(&self) -> &'static str {
        match self {
            Locale::EnUs => "en_US",
            Locale::NoNo => "no_NO",
            Locale::DeDe => "de_DE",
            Locale::FrFr => "fr_FR",
            Locale::EsEs => "es_ES",
            Locale::JaJp => "ja_JP",
            Locale::ZhCn => "zh_CN",
            Locale::PtBr => "pt_BR",
            Locale::ItIt => "it_IT",
            Locale::NlNl => "nl_NL",
            Locale::SvSe => "sv_SE",
        }
    }

    /// Get the language name.
    pub fn language(&self) -> &'static str {
        match self {
            Locale::EnUs => "English",
            Locale::NoNo => "Norwegian",
            Locale::DeDe => "German",
            Locale::FrFr => "French",
            Locale::EsEs => "Spanish",
            Locale::JaJp => "Japanese",
            Locale::ZhCn => "Chinese",
            Locale::PtBr => "Portuguese",
            Locale::ItIt => "Italian",
            Locale::NlNl => "Dutch",
            Locale::SvSe => "Swedish",
        }
    }

    /// Get the country name.
    pub fn country(&self) -> &'static str {
        match self {
            Locale::EnUs => "United States",
            Locale::NoNo => "Norway",
            Locale::DeDe => "Germany",
            Locale::FrFr => "France",
            Locale::EsEs => "Spain",
            Locale::JaJp => "Japan",
            Locale::ZhCn => "China",
            Locale::PtBr => "Brazil",
            Locale::ItIt => "Italy",
            Locale::NlNl => "Netherlands",
            Locale::SvSe => "Sweden",
        }
    }

    /// Get all available locales.
    pub fn all() -> &'static [Locale] {
        &[
            Locale::EnUs,
            Locale::NoNo,
            Locale::DeDe,
            Locale::FrFr,
            Locale::EsEs,
            Locale::JaJp,
            Locale::ZhCn,
            Locale::PtBr,
            Locale::ItIt,
            Locale::NlNl,
            Locale::SvSe,
        ]
    }
}

/// Trait for locale-specific data generation.
pub trait LocaleData {
    /// Get a random first name for this locale.
    fn first_name<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Get a random male first name for this locale.
    fn first_name_male<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Get a random female first name for this locale.
    fn first_name_female<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Get a random last name for this locale.
    fn last_name<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Generate a full name for this locale.
    fn full_name<R: ?Sized + Rng>(&self, rng: &mut R) -> String {
        format!("{} {}", self.first_name(rng), self.last_name(rng))
    }

    /// Generate a phone number for this locale.
    fn phone<R: ?Sized + Rng>(&self, rng: &mut R) -> String;

    /// Get a random city for this locale.
    fn city<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Get a random street suffix (Street, Avenue, etc.).
    fn street_suffix<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str;

    /// Generate a street address for this locale.
    fn street_address<R: ?Sized + Rng>(&self, rng: &mut R) -> String;

    /// Generate a postal/zip code for this locale.
    fn postal_code<R: ?Sized + Rng>(&self, rng: &mut R) -> String;
}

impl LocaleData for Locale {
    fn first_name<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::first_name(rng),
            Locale::NoNo => no_no::first_name(rng),
            Locale::DeDe => de_de::first_name(rng),
            Locale::FrFr => fr_fr::first_name(rng),
            Locale::EsEs => es_es::first_name(rng),
            Locale::JaJp => ja_jp::first_name(rng),
            Locale::ZhCn => zh_cn::first_name(rng),
            Locale::PtBr => pt_br::first_name(rng),
            Locale::ItIt => it_it::first_name(rng),
            Locale::NlNl => nl_nl::first_name(rng),
            Locale::SvSe => sv_se::first_name(rng),
        }
    }

    fn first_name_male<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::first_name_male(rng),
            Locale::NoNo => no_no::first_name_male(rng),
            Locale::DeDe => de_de::first_name_male(rng),
            Locale::FrFr => fr_fr::first_name_male(rng),
            Locale::EsEs => es_es::first_name_male(rng),
            Locale::JaJp => ja_jp::first_name_male(rng),
            Locale::ZhCn => zh_cn::first_name_male(rng),
            Locale::PtBr => pt_br::first_name_male(rng),
            Locale::ItIt => it_it::first_name_male(rng),
            Locale::NlNl => nl_nl::first_name_male(rng),
            Locale::SvSe => sv_se::first_name_male(rng),
        }
    }

    fn first_name_female<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::first_name_female(rng),
            Locale::NoNo => no_no::first_name_female(rng),
            Locale::DeDe => de_de::first_name_female(rng),
            Locale::FrFr => fr_fr::first_name_female(rng),
            Locale::EsEs => es_es::first_name_female(rng),
            Locale::JaJp => ja_jp::first_name_female(rng),
            Locale::ZhCn => zh_cn::first_name_female(rng),
            Locale::PtBr => pt_br::first_name_female(rng),
            Locale::ItIt => it_it::first_name_female(rng),
            Locale::NlNl => nl_nl::first_name_female(rng),
            Locale::SvSe => sv_se::first_name_female(rng),
        }
    }

    fn last_name<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::last_name(rng),
            Locale::NoNo => no_no::last_name(rng),
            Locale::DeDe => de_de::last_name(rng),
            Locale::FrFr => fr_fr::last_name(rng),
            Locale::EsEs => es_es::last_name(rng),
            Locale::JaJp => ja_jp::last_name(rng),
            Locale::ZhCn => zh_cn::last_name(rng),
            Locale::PtBr => pt_br::last_name(rng),
            Locale::ItIt => it_it::last_name(rng),
            Locale::NlNl => nl_nl::last_name(rng),
            Locale::SvSe => sv_se::last_name(rng),
        }
    }

    fn phone<R: ?Sized + Rng>(&self, rng: &mut R) -> String {
        match self {
            Locale::EnUs => en_us::phone(rng),
            Locale::NoNo => no_no::phone(rng),
            Locale::DeDe => de_de::phone(rng),
            Locale::FrFr => fr_fr::phone(rng),
            Locale::EsEs => es_es::phone(rng),
            Locale::JaJp => ja_jp::phone(rng),
            Locale::ZhCn => zh_cn::phone(rng),
            Locale::PtBr => pt_br::phone(rng),
            Locale::ItIt => it_it::phone(rng),
            Locale::NlNl => nl_nl::phone(rng),
            Locale::SvSe => sv_se::phone(rng),
        }
    }

    fn city<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::city(rng),
            Locale::NoNo => no_no::city(rng),
            Locale::DeDe => de_de::city(rng),
            Locale::FrFr => fr_fr::city(rng),
            Locale::EsEs => es_es::city(rng),
            Locale::JaJp => ja_jp::city(rng),
            Locale::ZhCn => zh_cn::city(rng),
            Locale::PtBr => pt_br::city(rng),
            Locale::ItIt => it_it::city(rng),
            Locale::NlNl => nl_nl::city(rng),
            Locale::SvSe => sv_se::city(rng),
        }
    }

    fn street_suffix<R: ?Sized + Rng>(&self, rng: &mut R) -> &'static str {
        match self {
            Locale::EnUs => en_us::street_suffix(rng),
            Locale::NoNo => no_no::street_suffix(rng),
            Locale::DeDe => de_de::street_suffix(rng),
            Locale::FrFr => fr_fr::street_suffix(rng),
            Locale::EsEs => es_es::street_suffix(rng),
            Locale::JaJp => ja_jp::street_suffix(rng),
            Locale::ZhCn => zh_cn::street_suffix(rng),
            Locale::PtBr => pt_br::street_suffix(rng),
            Locale::ItIt => it_it::street_suffix(rng),
            Locale::NlNl => nl_nl::street_suffix(rng),
            Locale::SvSe => sv_se::street_suffix(rng),
        }
    }

    fn street_address<R: ?Sized + Rng>(&self, rng: &mut R) -> String {
        match self {
            Locale::EnUs => en_us::street_address(rng),
            Locale::NoNo => no_no::street_address(rng),
            Locale::DeDe => de_de::street_address(rng),
            Locale::FrFr => fr_fr::street_address(rng),
            Locale::EsEs => es_es::street_address(rng),
            Locale::JaJp => ja_jp::street_address(rng),
            Locale::ZhCn => zh_cn::street_address(rng),
            Locale::PtBr => pt_br::street_address(rng),
            Locale::ItIt => it_it::street_address(rng),
            Locale::NlNl => nl_nl::street_address(rng),
            Locale::SvSe => sv_se::street_address(rng),
        }
    }

    fn postal_code<R: ?Sized + Rng>(&self, rng: &mut R) -> String {
        match self {
            Locale::EnUs => en_us::postal_code(rng),
            Locale::NoNo => no_no::postal_code(rng),
            Locale::DeDe => de_de::postal_code(rng),
            Locale::FrFr => fr_fr::postal_code(rng),
            Locale::EsEs => es_es::postal_code(rng),
            Locale::JaJp => ja_jp::postal_code(rng),
            Locale::ZhCn => zh_cn::postal_code(rng),
            Locale::PtBr => pt_br::postal_code(rng),
            Locale::ItIt => it_it::postal_code(rng),
            Locale::NlNl => nl_nl::postal_code(rng),
            Locale::SvSe => sv_se::postal_code(rng),
        }
    }
}

pub use de_de::DeDe;
pub use en_us::EnUs;
pub use es_es::EsEs;
pub use fr_fr::FrFr;
pub use it_it::ItIt;
pub use ja_jp::JaJp;
pub use nl_nl::NlNl;
pub use no_no::NoNo;
pub use pt_br::PtBr;
pub use sv_se::SvSe;
pub use zh_cn::ZhCn;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_locale_from_code() {
        assert_eq!(Locale::from_code("en_US"), Some(Locale::EnUs));
        assert_eq!(Locale::from_code("en-US"), Some(Locale::EnUs));
        assert_eq!(Locale::from_code("no_NO"), Some(Locale::NoNo));
        assert_eq!(Locale::from_code("nb"), Some(Locale::NoNo));
        assert_eq!(Locale::from_code("de_DE"), Some(Locale::DeDe));
        assert_eq!(Locale::from_code("de"), Some(Locale::DeDe));
        assert_eq!(Locale::from_code("fr_FR"), Some(Locale::FrFr));
        assert_eq!(Locale::from_code("fr"), Some(Locale::FrFr));
        assert_eq!(Locale::from_code("es_ES"), Some(Locale::EsEs));
        assert_eq!(Locale::from_code("es"), Some(Locale::EsEs));
        assert_eq!(Locale::from_code("ja_JP"), Some(Locale::JaJp));
        assert_eq!(Locale::from_code("ja"), Some(Locale::JaJp));
        assert_eq!(Locale::from_code("zh_CN"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_code("zh"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_code("pt_BR"), Some(Locale::PtBr));
        assert_eq!(Locale::from_code("pt"), Some(Locale::PtBr));
        assert_eq!(Locale::from_code("it_IT"), Some(Locale::ItIt));
        assert_eq!(Locale::from_code("it"), Some(Locale::ItIt));
        assert_eq!(Locale::from_code("nl_NL"), Some(Locale::NlNl));
        assert_eq!(Locale::from_code("nl"), Some(Locale::NlNl));
        assert_eq!(Locale::from_code("sv_SE"), Some(Locale::SvSe));
        assert_eq!(Locale::from_code("sv"), Some(Locale::SvSe));
        assert_eq!(Locale::from_code("invalid"), None);
    }

    #[test]
    fn test_locale_code() {
        assert_eq!(Locale::EnUs.code(), "en_US");
        assert_eq!(Locale::NoNo.code(), "no_NO");
        assert_eq!(Locale::DeDe.code(), "de_DE");
        assert_eq!(Locale::FrFr.code(), "fr_FR");
        assert_eq!(Locale::EsEs.code(), "es_ES");
        assert_eq!(Locale::JaJp.code(), "ja_JP");
        assert_eq!(Locale::ZhCn.code(), "zh_CN");
        assert_eq!(Locale::PtBr.code(), "pt_BR");
        assert_eq!(Locale::ItIt.code(), "it_IT");
        assert_eq!(Locale::NlNl.code(), "nl_NL");
        assert_eq!(Locale::SvSe.code(), "sv_SE");
    }

    #[test]
    fn test_locale_data_en_us() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::EnUs;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_no_no() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::NoNo;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_de_de() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::DeDe;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_fr_fr() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::FrFr;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_es_es() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::EsEs;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_ja_jp() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::JaJp;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_zh_cn() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::ZhCn;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_pt_br() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::PtBr;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_it_it() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::ItIt;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_nl_nl() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::NlNl;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_locale_data_sv_se() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::SvSe;

        let first = locale.first_name(&mut rng);
        let last = locale.last_name(&mut rng);
        let phone = locale.phone(&mut rng);
        let city = locale.city(&mut rng);

        assert!(!first.is_empty());
        assert!(!last.is_empty());
        assert!(!phone.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_full_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let locale = Locale::EnUs;
        let name = locale.full_name(&mut rng);
        assert!(name.contains(' '));
    }

    #[test]
    fn test_all_locales() {
        let all = Locale::all();
        assert_eq!(all.len(), 11);
        assert!(all.contains(&Locale::EnUs));
        assert!(all.contains(&Locale::NoNo));
        assert!(all.contains(&Locale::DeDe));
        assert!(all.contains(&Locale::FrFr));
        assert!(all.contains(&Locale::EsEs));
        assert!(all.contains(&Locale::JaJp));
        assert!(all.contains(&Locale::ZhCn));
        assert!(all.contains(&Locale::PtBr));
        assert!(all.contains(&Locale::ItIt));
        assert!(all.contains(&Locale::NlNl));
        assert!(all.contains(&Locale::SvSe));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        let locale = Locale::EnUs;

        assert_eq!(locale.first_name(&mut rng1), locale.first_name(&mut rng2));
    }
}
