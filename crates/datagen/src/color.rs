//! Color generation utilities.
//!
//! This module provides random color generation in various formats:
//! - Hex colors (#RRGGBB)
//! - RGB tuples
//! - RGBA tuples with alpha
//! - HSL values
//! - Named colors
//!
//! # Example
//!
//! ```
//! use dx_datagen::color;
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//! let hex = color::hex_color(&mut rng);
//! let (r, g, b) = color::rgb(&mut rng);
//! let name = color::color_name(&mut rng);
//! ```

use rand::Rng;

/// Named colors with their hex values
pub const COLORS: &[(&str, &str)] = &[
    ("Red", "#FF0000"),
    ("Green", "#00FF00"),
    ("Blue", "#0000FF"),
    ("Yellow", "#FFFF00"),
    ("Cyan", "#00FFFF"),
    ("Magenta", "#FF00FF"),
    ("Orange", "#FFA500"),
    ("Purple", "#800080"),
    ("Pink", "#FFC0CB"),
    ("Brown", "#A52A2A"),
    ("Black", "#000000"),
    ("White", "#FFFFFF"),
    ("Gray", "#808080"),
    ("Navy", "#000080"),
    ("Teal", "#008080"),
    ("Olive", "#808000"),
    ("Maroon", "#800000"),
    ("Lime", "#00FF00"),
    ("Aqua", "#00FFFF"),
    ("Silver", "#C0C0C0"),
    ("Gold", "#FFD700"),
    ("Coral", "#FF7F50"),
    ("Salmon", "#FA8072"),
    ("Turquoise", "#40E0D0"),
    ("Indigo", "#4B0082"),
    ("Violet", "#EE82EE"),
    ("Crimson", "#DC143C"),
    ("Khaki", "#F0E68C"),
    ("Lavender", "#E6E6FA"),
    ("Beige", "#F5F5DC"),
];

/// CSS color names (subset of web colors)
pub const CSS_COLORS: &[&str] = &[
    "aliceblue",
    "antiquewhite",
    "aqua",
    "aquamarine",
    "azure",
    "beige",
    "bisque",
    "black",
    "blanchedalmond",
    "blue",
    "blueviolet",
    "brown",
    "burlywood",
    "cadetblue",
    "chartreuse",
    "chocolate",
    "coral",
    "cornflowerblue",
    "cornsilk",
    "crimson",
    "cyan",
    "darkblue",
    "darkcyan",
    "darkgoldenrod",
    "darkgray",
    "darkgreen",
    "darkkhaki",
    "darkmagenta",
    "darkolivegreen",
    "darkorange",
    "darkorchid",
    "darkred",
    "darksalmon",
    "darkseagreen",
    "darkslateblue",
    "darkslategray",
    "darkturquoise",
    "darkviolet",
    "deeppink",
    "deepskyblue",
    "dimgray",
    "dodgerblue",
    "firebrick",
    "floralwhite",
    "forestgreen",
    "fuchsia",
    "gainsboro",
    "ghostwhite",
    "gold",
    "goldenrod",
    "gray",
    "green",
    "greenyellow",
    "honeydew",
    "hotpink",
    "indianred",
    "indigo",
    "ivory",
    "khaki",
    "lavender",
    "lavenderblush",
    "lawngreen",
    "lemonchiffon",
    "lightblue",
    "lightcoral",
    "lightcyan",
    "lightgoldenrodyellow",
    "lightgray",
    "lightgreen",
    "lightpink",
    "lightsalmon",
    "lightseagreen",
    "lightskyblue",
    "lightslategray",
    "lightsteelblue",
    "lightyellow",
    "lime",
    "limegreen",
    "linen",
    "magenta",
    "maroon",
    "mediumaquamarine",
    "mediumblue",
    "mediumorchid",
    "mediumpurple",
    "mediumseagreen",
    "mediumslateblue",
    "mediumspringgreen",
    "mediumturquoise",
    "mediumvioletred",
    "midnightblue",
    "mintcream",
    "mistyrose",
    "moccasin",
    "navajowhite",
    "navy",
    "oldlace",
    "olive",
    "olivedrab",
    "orange",
    "orangered",
    "orchid",
    "palegoldenrod",
    "palegreen",
    "paleturquoise",
    "palevioletred",
    "papayawhip",
    "peachpuff",
    "peru",
    "pink",
    "plum",
    "powderblue",
    "purple",
    "rebeccapurple",
    "red",
    "rosybrown",
    "royalblue",
    "saddlebrown",
    "salmon",
    "sandybrown",
    "seagreen",
    "seashell",
    "sienna",
    "silver",
    "skyblue",
    "slateblue",
    "slategray",
    "snow",
    "springgreen",
    "steelblue",
    "tan",
    "teal",
    "thistle",
    "tomato",
    "turquoise",
    "violet",
    "wheat",
    "white",
    "whitesmoke",
    "yellow",
    "yellowgreen",
];

/// Generate a random hex color string (#RRGGBB).
///
/// # Example
/// ```
/// use dx_datagen::color::hex_color;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let color = hex_color(&mut rng);
/// assert!(color.starts_with('#'));
/// assert_eq!(color.len(), 7);
/// ```
pub fn hex_color<R: ?Sized + Rng>(rng: &mut R) -> String {
    let r: u8 = rng.random();
    let g: u8 = rng.random();
    let b: u8 = rng.random();
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Generate a random hex color with alpha (#RRGGBBAA).
///
/// # Example
/// ```
/// use dx_datagen::color::hex_color_alpha;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let color = hex_color_alpha(&mut rng);
/// assert!(color.starts_with('#'));
/// assert_eq!(color.len(), 9);
/// ```
pub fn hex_color_alpha<R: ?Sized + Rng>(rng: &mut R) -> String {
    let r: u8 = rng.random();
    let g: u8 = rng.random();
    let b: u8 = rng.random();
    let a: u8 = rng.random();
    format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
}

/// Generate a random RGB color tuple.
///
/// # Example
/// ```
/// use dx_datagen::color::rgb;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let (r, g, b) = rgb(&mut rng);
/// assert!(r <= 255 && g <= 255 && b <= 255);
/// ```
pub fn rgb<R: ?Sized + Rng>(rng: &mut R) -> (u8, u8, u8) {
    (rng.random(), rng.random(), rng.random())
}

/// Generate a random RGBA color tuple with alpha.
///
/// # Example
/// ```
/// use dx_datagen::color::rgba;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let (r, g, b, a) = rgba(&mut rng);
/// assert!(a >= 0.0 && a <= 1.0);
/// ```
pub fn rgba<R: ?Sized + Rng>(rng: &mut R) -> (u8, u8, u8, f32) {
    (
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random_range(0.0..=1.0),
    )
}

/// Generate a random HSL color tuple.
///
/// Returns (hue: 0-360, saturation: 0-100, lightness: 0-100)
///
/// # Example
/// ```
/// use dx_datagen::color::hsl;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let (h, s, l) = hsl(&mut rng);
/// assert!(h <= 360);
/// assert!(s <= 100 && l <= 100);
/// ```
pub fn hsl<R: ?Sized + Rng>(rng: &mut R) -> (u16, u8, u8) {
    (
        rng.random_range(0..=360),
        rng.random_range(0..=100),
        rng.random_range(0..=100),
    )
}

/// Generate a random HSLA color tuple with alpha.
///
/// Returns (hue: 0-360, saturation: 0-100, lightness: 0-100, alpha: 0.0-1.0)
pub fn hsla<R: ?Sized + Rng>(rng: &mut R) -> (u16, u8, u8, f32) {
    (
        rng.random_range(0..=360),
        rng.random_range(0..=100),
        rng.random_range(0..=100),
        rng.random_range(0.0..=1.0),
    )
}

/// Get a random named color.
///
/// # Example
/// ```
/// use dx_datagen::color::color_name;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let name = color_name(&mut rng);
/// assert!(!name.is_empty());
/// ```
pub fn color_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..COLORS.len());
    COLORS[idx].0
}

/// Get a random CSS color name.
///
/// # Example
/// ```
/// use dx_datagen::color::css_color_name;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let name = css_color_name(&mut rng);
/// assert!(!name.is_empty());
/// ```
pub fn css_color_name<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..CSS_COLORS.len());
    CSS_COLORS[idx]
}

/// Generate an RGB CSS color string.
///
/// # Example
/// ```
/// use dx_datagen::color::css_rgb;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let color = css_rgb(&mut rng);
/// assert!(color.starts_with("rgb("));
/// ```
pub fn css_rgb<R: ?Sized + Rng>(rng: &mut R) -> String {
    let (r, g, b) = rgb(rng);
    format!("rgb({}, {}, {})", r, g, b)
}

/// Generate an RGBA CSS color string.
///
/// # Example
/// ```
/// use dx_datagen::color::css_rgba;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let color = css_rgba(&mut rng);
/// assert!(color.starts_with("rgba("));
/// ```
pub fn css_rgba<R: ?Sized + Rng>(rng: &mut R) -> String {
    let (r, g, b, a) = rgba(rng);
    format!("rgba({}, {}, {}, {:.2})", r, g, b, a)
}

/// Generate an HSL CSS color string.
///
/// # Example
/// ```
/// use dx_datagen::color::css_hsl;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let color = css_hsl(&mut rng);
/// assert!(color.starts_with("hsl("));
/// ```
pub fn css_hsl<R: ?Sized + Rng>(rng: &mut R) -> String {
    let (h, s, l) = hsl(rng);
    format!("hsl({}, {}%, {}%)", h, s, l)
}

/// Generate an HSLA CSS color string.
pub fn css_hsla<R: ?Sized + Rng>(rng: &mut R) -> String {
    let (h, s, l, a) = hsla(rng);
    format!("hsla({}, {}%, {}%, {:.2})", h, s, l, a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_hex_color() {
        let mut rng = StdRng::seed_from_u64(42);
        let color = hex_color(&mut rng);
        assert!(color.starts_with('#'));
        assert_eq!(color.len(), 7);
        // Verify hex chars
        assert!(color[1..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_hex_color_alpha() {
        let mut rng = StdRng::seed_from_u64(42);
        let color = hex_color_alpha(&mut rng);
        assert!(color.starts_with('#'));
        assert_eq!(color.len(), 9);
    }

    #[test]
    fn test_rgb() {
        let mut rng = StdRng::seed_from_u64(42);
        let (r, g, b) = rgb(&mut rng);
        // Just verify we get values (all u8 are valid)
        assert!(r <= 255 && g <= 255 && b <= 255);
    }

    #[test]
    fn test_rgba() {
        let mut rng = StdRng::seed_from_u64(42);
        let (r, g, b, a) = rgba(&mut rng);
        assert!(r <= 255 && g <= 255 && b <= 255);
        assert!((0.0..=1.0).contains(&a));
    }

    #[test]
    fn test_hsl() {
        let mut rng = StdRng::seed_from_u64(42);
        let (h, s, l) = hsl(&mut rng);
        assert!(h <= 360);
        assert!(s <= 100);
        assert!(l <= 100);
    }

    #[test]
    fn test_color_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = color_name(&mut rng);
        assert!(COLORS.iter().any(|(n, _)| *n == name));
    }

    #[test]
    fn test_css_rgb() {
        let mut rng = StdRng::seed_from_u64(42);
        let color = css_rgb(&mut rng);
        assert!(color.starts_with("rgb("));
        assert!(color.ends_with(')'));
    }

    #[test]
    fn test_determinism() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);

        assert_eq!(hex_color(&mut rng1), hex_color(&mut rng2));
        assert_eq!(rgb(&mut rng1), rgb(&mut rng2));
        assert_eq!(hsl(&mut rng1), hsl(&mut rng2));
    }

    #[test]
    fn test_trait_object() {
        use rand::RngCore;
        let mut rng: Box<dyn RngCore> = Box::new(StdRng::seed_from_u64(42));
        let color = hex_color(&mut *rng);
        assert!(color.starts_with('#'));
    }
}
