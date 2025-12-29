//! Chinese (Simplified) locale data (zh_CN).
//!
//! Provides Chinese-specific names, addresses, phone numbers, and more.

use rand::Rng;

/// Chinese locale marker type.
pub struct ZhCn;

/// Common male first names in China (romanized).
pub const MALE_FIRST_NAMES: &[&str] = &[
    "Wei", "Fang", "Lei", "Jun", "Ming", "Qiang", "Jie", "Tao", "Ping", "Gang", "Chao", "Bo",
    "Peng", "Long", "Hui", "Yang", "Feng", "Chen", "Hao", "Kai", "Xiang", "Liang", "Dong", "Bin",
    "Yong", "Jian", "Wen", "Xin", "Zhi", "Yu", "Zhong", "Sheng", "Lin", "Da", "Guo", "Hai", "Shan",
    "Xiao", "Yan", "Ke",
];

/// Common female first names in China (romanized).
pub const FEMALE_FIRST_NAMES: &[&str] = &[
    "Fang", "Min", "Jing", "Li", "Yan", "Juan", "Xia", "Hong", "Hui", "Mei", "Lan", "Ying", "Hua",
    "Ping", "Qin", "Na", "Lin", "Xiu", "Ling", "Yun", "Qing", "Xiao", "Rong", "Fei", "Dan", "Jie",
    "Wen", "Xue", "Yue", "Zhen", "Shan", "Ning", "Yi", "Qian", "Lu", "Lei", "Yu", "Meng", "Shuang",
    "Ting",
];

/// Common last names in China (romanized).
pub const LAST_NAMES: &[&str] = &[
    "Wang", "Li", "Zhang", "Liu", "Chen", "Yang", "Huang", "Zhao", "Wu", "Zhou", "Xu", "Sun", "Ma",
    "Zhu", "Hu", "Guo", "He", "Lin", "Gao", "Luo", "Zheng", "Liang", "Xie", "Tang", "Xu", "Feng",
    "Deng", "Cao", "Peng", "Zeng", "Xiao", "Tian", "Dong", "Pan", "Yuan", "Cai", "Jiang", "Yu",
    "Du", "Ye", "Cheng", "Wei", "Su", "Lu", "Ding", "Ren", "Shen", "Yao", "Lu", "Gu",
];

/// Chinese cities.
pub const CITIES: &[&str] = &[
    "Beijing",
    "Shanghai",
    "Guangzhou",
    "Shenzhen",
    "Chengdu",
    "Hangzhou",
    "Wuhan",
    "Xi'an",
    "Nanjing",
    "Tianjin",
    "Suzhou",
    "Chongqing",
    "Qingdao",
    "Dalian",
    "Ningbo",
    "Xiamen",
    "Shenyang",
    "Changsha",
    "Zhengzhou",
    "Dongguan",
    "Foshan",
    "Hefei",
    "Wuxi",
    "Kunming",
    "Harbin",
    "Jinan",
    "Fuzhou",
    "Changchun",
    "Nanchang",
    "Nanning",
];

/// Chinese provinces.
pub const PROVINCES: &[&str] = &[
    "Beijing",
    "Shanghai",
    "Tianjin",
    "Chongqing",
    "Hebei",
    "Shanxi",
    "Liaoning",
    "Jilin",
    "Heilongjiang",
    "Jiangsu",
    "Zhejiang",
    "Anhui",
    "Fujian",
    "Jiangxi",
    "Shandong",
    "Henan",
    "Hubei",
    "Hunan",
    "Guangdong",
    "Hainan",
    "Sichuan",
    "Guizhou",
    "Yunnan",
    "Shaanxi",
    "Gansu",
    "Qinghai",
    "Taiwan",
    "Inner Mongolia",
    "Guangxi",
    "Tibet",
    "Ningxia",
    "Xinjiang",
    "Hong Kong",
    "Macau",
];

/// Street suffixes in Chinese addresses.
pub const STREET_SUFFIXES: &[&str] = &[
    "Lu", "Jie", "Dao", "Xiang", "Hutong", "Dadao", "Zhonglu", "Beilu", "Nanlu", "Donglu",
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

/// Generate a full name (Chinese order: last name first).
pub fn full_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{} {}", last_name(rng), first_name(rng))
}

/// Generate a Chinese phone number in +86 XXX XXXX XXXX format.
pub fn phone<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefixes = [
        "130", "131", "132", "133", "135", "136", "137", "138", "139", "150", "151", "152", "153",
        "155", "156", "157", "158", "159", "186", "187", "188", "189",
    ];
    let prefix = prefixes[rng.random_range(0..prefixes.len())];
    let suffix: u32 = rng.random_range(10000000..99999999);
    format!("{} {}", prefix, suffix)
}

/// Get a random city.
pub fn city<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    CITIES[rng.random_range(0..CITIES.len())]
}

/// Get a random province.
pub fn province<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    PROVINCES[rng.random_range(0..PROVINCES.len())]
}

/// Get a random street suffix.
pub fn street_suffix<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    STREET_SUFFIXES[rng.random_range(0..STREET_SUFFIXES.len())]
}

/// Generate a street address.
pub fn street_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let number = rng.random_range(1..500);
    let suffix = street_suffix(rng);
    format!("{} Hao, {} {}", number, rng.random_range(1..100), suffix)
}

/// Generate a postal code (Chinese format: 6 digits).
pub fn postal_code<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("{:06}", rng.random_range(100000..999999))
}

/// Generate a full address.
pub fn full_address<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prov = province(rng);
    let city_name = city(rng);
    let street = street_address(rng);
    let postal = postal_code(rng);
    format!("{} {} {} {}", prov, city_name, street, postal)
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
        assert!(!phone_num.is_empty());
    }

    #[test]
    fn test_postal_code() {
        let mut rng = StdRng::seed_from_u64(42);
        let code = postal_code(&mut rng);
        assert_eq!(code.len(), 6);
    }
}
