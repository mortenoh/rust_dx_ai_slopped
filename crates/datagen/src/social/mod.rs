//! Social media data generation.
//!
//! Generates realistic social media handles, content, hashtags,
//! engagement metrics, and platform-specific data.
//!
//! # Examples
//!
//! ```
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//! use dx_datagen::social;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let handle = social::twitter_handle(&mut rng);
//! let hashtag = social::hashtag(&mut rng);
//! let followers = social::follower_count(&mut rng);
//! ```

use rand::Rng;

// =============================================================================
// Handles & Usernames
// =============================================================================

/// Generate a Twitter/X handle.
///
/// # Example
/// ```
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use dx_datagen::social::twitter_handle;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let handle = twitter_handle(&mut rng);
/// assert!(handle.starts_with('@'));
/// ```
pub fn twitter_handle<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("@{}", username(rng))
}

/// Generate an Instagram handle.
pub fn instagram_handle<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("@{}", username(rng))
}

/// Generate a TikTok handle.
pub fn tiktok_handle<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("@{}", username(rng))
}

/// Generate a LinkedIn URL.
pub fn linkedin_url<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("https://linkedin.com/in/{}", username(rng).to_lowercase())
}

/// Generate a GitHub username.
pub fn github_username<R: Rng + ?Sized>(rng: &mut R) -> String {
    username(rng)
}

/// Generate a GitHub URL.
pub fn github_url<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("https://github.com/{}", username(rng).to_lowercase())
}

/// Generate a Discord username (new format without discriminator).
pub fn discord_username<R: Rng + ?Sized>(rng: &mut R) -> String {
    username(rng).to_lowercase()
}

/// Generate a Discord username (legacy format with discriminator).
pub fn discord_username_legacy<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("{}#{:04}", username(rng), rng.random_range(0..10000u16))
}

/// Generate a YouTube channel name.
pub fn youtube_channel<R: Rng + ?Sized>(rng: &mut R) -> String {
    let formats = [
        format!("{} Official", random_name(rng)),
        format!("The {} Channel", random_name(rng)),
        format!("{} Gaming", username(rng)),
        format!("{} Vlogs", random_name(rng)),
        username(rng).to_string(),
    ];
    formats[rng.random_range(0..formats.len())].clone()
}

/// Generate a Twitch username.
pub fn twitch_username<R: Rng + ?Sized>(rng: &mut R) -> String {
    username(rng).to_lowercase()
}

/// Generate a Reddit username.
pub fn reddit_username<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("u/{}", username(rng))
}

/// Generate a basic social media username.
fn username<R: Rng + ?Sized>(rng: &mut R) -> String {
    const ADJECTIVES: &[&str] = &[
        "Cool", "Happy", "Dark", "Bright", "Swift", "Silent", "Golden", "Silver", "Wild", "Crazy",
        "Epic", "Super", "Ultra", "Mega", "Pro", "Elite", "Royal", "Cosmic", "Mystic", "Neon",
        "Cyber", "Digital", "Pixel", "Retro",
    ];
    const NOUNS: &[&str] = &[
        "Wolf", "Dragon", "Phoenix", "Tiger", "Eagle", "Bear", "Lion", "Hawk", "Storm", "Thunder",
        "Shadow", "Star", "Moon", "Sun", "Fire", "Ice", "Ninja", "Samurai", "Knight", "Wizard",
        "Gamer", "Coder", "Maker", "Artist",
    ];

    let style = rng.random_range(0..5);
    match style {
        0 => format!(
            "{}{}",
            ADJECTIVES[rng.random_range(0..ADJECTIVES.len())],
            NOUNS[rng.random_range(0..NOUNS.len())]
        ),
        1 => format!(
            "{}_{}",
            ADJECTIVES[rng.random_range(0..ADJECTIVES.len())].to_lowercase(),
            NOUNS[rng.random_range(0..NOUNS.len())].to_lowercase()
        ),
        2 => format!(
            "{}{}{}",
            ADJECTIVES[rng.random_range(0..ADJECTIVES.len())],
            NOUNS[rng.random_range(0..NOUNS.len())],
            rng.random_range(1..999)
        ),
        3 => format!(
            "x_{}_x",
            NOUNS[rng.random_range(0..NOUNS.len())].to_lowercase()
        ),
        _ => format!("The{}", NOUNS[rng.random_range(0..NOUNS.len())]),
    }
}

fn random_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const NAMES: &[&str] = &[
        "Alex", "Jordan", "Taylor", "Morgan", "Casey", "Riley", "Quinn", "Avery", "Charlie", "Sam",
        "Drew", "Blake", "Jamie", "Reese", "Skyler", "Dakota",
    ];
    NAMES[rng.random_range(0..NAMES.len())]
}

// =============================================================================
// Content
// =============================================================================

/// Generate a hashtag.
///
/// # Example
/// ```
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use dx_datagen::social::hashtag;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let tag = hashtag(&mut rng);
/// assert!(tag.starts_with('#'));
/// ```
pub fn hashtag<R: Rng + ?Sized>(rng: &mut R) -> String {
    const TAGS: &[&str] = &[
        "trending",
        "viral",
        "fyp",
        "foryou",
        "love",
        "instagood",
        "photooftheday",
        "fashion",
        "beautiful",
        "happy",
        "cute",
        "tbt",
        "like4like",
        "followme",
        "picoftheday",
        "selfie",
        "summer",
        "art",
        "instadaily",
        "friends",
        "repost",
        "nature",
        "smile",
        "style",
        "food",
        "instalike",
        "travel",
        "motivation",
        "fitness",
        "gaming",
        "music",
        "tech",
        "coding",
        "design",
        "photography",
        "lifestyle",
        "health",
        "business",
        "success",
        "mindset",
    ];
    format!("#{}", TAGS[rng.random_range(0..TAGS.len())])
}

/// Generate multiple hashtags.
pub fn hashtags<R: Rng + ?Sized>(rng: &mut R, count: usize) -> String {
    (0..count)
        .map(|_| hashtag(rng))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a mention.
pub fn mention<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("@{}", username(rng).to_lowercase())
}

/// Generate an emoji.
pub fn emoji<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const EMOJIS: &[&str] = &[
        "😀", "😂", "🥰", "😍", "🤩", "😎", "🥳", "🤔", "👍", "👏", "🙌", "💪", "🔥", "💯", "⭐",
        "✨", "💖", "❤️", "🎉", "🎊", "🌟", "💫", "🚀", "💡", "🎯", "🏆", "🥇", "🎮", "🎨", "📸",
        "🎵", "🎶", "☕", "🌈", "🌸", "🍕", "🎂", "🐶", "🐱", "🦄",
    ];
    EMOJIS[rng.random_range(0..EMOJIS.len())]
}

/// Generate multiple emojis.
pub fn emojis<R: Rng + ?Sized>(rng: &mut R, count: usize) -> String {
    (0..count).map(|_| emoji(rng)).collect()
}

/// Generate a short post/tweet text.
pub fn post_text<R: Rng + ?Sized>(rng: &mut R) -> String {
    const POSTS: &[&str] = &[
        "Just had the best day ever! 🎉",
        "Can't believe this happened 😱",
        "Feeling grateful today 🙏",
        "New content coming soon! Stay tuned 👀",
        "Who else loves weekends? 🙌",
        "This is absolutely incredible! 🔥",
        "Making memories that last forever ✨",
        "Living my best life 💫",
        "Big announcement coming soon! 🚀",
        "Thank you all for the support! ❤️",
        "Thoughts on this? Let me know below 👇",
        "POV: You're having the best time 😎",
        "This hit different 💯",
        "Can we normalize this? 🤔",
        "The vibes are immaculate ✨",
    ];
    POSTS[rng.random_range(0..POSTS.len())].to_string()
}

/// Generate a bio/profile description.
pub fn bio<R: Rng + ?Sized>(rng: &mut R) -> String {
    const BIOS: &[&str] = &[
        "Living life one day at a time ✨",
        "Digital creator | Coffee lover ☕",
        "Here for a good time, not a long time 🎉",
        "Making the world a better place 🌍",
        "Dreamer | Creator | Believer 💫",
        "Just a human being 🌟",
        "Spreading positivity everywhere I go ☀️",
        "Work hard, play harder 💪",
        "Adventure awaits 🏔️",
        "Professional overthinker 🧠",
        "Trying my best every day 🙌",
        "Creating content that matters 📸",
        "On a journey to success 🚀",
        "Living proof that dreams come true ⭐",
        "Making memories, not excuses 📍",
    ];
    BIOS[rng.random_range(0..BIOS.len())].to_string()
}

/// Generate a post caption.
pub fn caption<R: Rng + ?Sized>(rng: &mut R) -> String {
    let post = post_text(rng);
    let tag_count = rng.random_range(2..5);
    let tags = hashtags(rng, tag_count);
    format!("{}\n\n{}", post, tags)
}

// =============================================================================
// Engagement Metrics
// =============================================================================

/// Generate a follower count.
///
/// # Example
/// ```
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use dx_datagen::social::follower_count;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let followers = follower_count(&mut rng);
/// assert!(followers >= 0);
/// ```
pub fn follower_count<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    // Weighted distribution - most accounts are small
    let tier = rng.random_range(0..100);
    if tier < 50 {
        rng.random_range(0..1000) // 50% have <1k
    } else if tier < 75 {
        rng.random_range(1000..10000) // 25% have 1k-10k
    } else if tier < 90 {
        rng.random_range(10000..100000) // 15% have 10k-100k
    } else if tier < 98 {
        rng.random_range(100000..1000000) // 8% have 100k-1M
    } else {
        rng.random_range(1000000..100000000) // 2% have 1M+
    }
}

/// Generate a following count.
pub fn following_count<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    rng.random_range(10..5000)
}

/// Generate a like count for a post.
pub fn like_count<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    let tier = rng.random_range(0..100);
    if tier < 60 {
        rng.random_range(0..100)
    } else if tier < 85 {
        rng.random_range(100..1000)
    } else if tier < 95 {
        rng.random_range(1000..10000)
    } else {
        rng.random_range(10000..1000000)
    }
}

/// Generate a comment count.
pub fn comment_count<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    let tier = rng.random_range(0..100);
    if tier < 70 {
        rng.random_range(0..20)
    } else if tier < 90 {
        rng.random_range(20..100)
    } else {
        rng.random_range(100..5000)
    }
}

/// Generate a share/retweet count.
pub fn share_count<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    let tier = rng.random_range(0..100);
    if tier < 80 {
        rng.random_range(0..50)
    } else if tier < 95 {
        rng.random_range(50..500)
    } else {
        rng.random_range(500..50000)
    }
}

/// Generate a view count.
pub fn view_count<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    let tier = rng.random_range(0..100);
    if tier < 50 {
        rng.random_range(100..5000)
    } else if tier < 80 {
        rng.random_range(5000..50000)
    } else if tier < 95 {
        rng.random_range(50000..500000)
    } else {
        rng.random_range(500000..100000000)
    }
}

/// Generate an engagement rate (percentage).
pub fn engagement_rate<R: Rng + ?Sized>(rng: &mut R) -> f64 {
    let rate: f64 = rng.random_range(0.1..15.0);
    (rate * 100.0).round() / 100.0
}

/// Generate a formatted follower count string.
pub fn follower_count_formatted<R: Rng + ?Sized>(rng: &mut R) -> String {
    let count = follower_count(rng);
    format_count(count)
}

/// Format a number with K/M/B suffixes.
fn format_count(count: u64) -> String {
    if count >= 1_000_000_000 {
        format!("{:.1}B", count as f64 / 1_000_000_000.0)
    } else if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

// =============================================================================
// Platform-Specific
// =============================================================================

/// Generate a social media platform name.
pub fn platform<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const PLATFORMS: &[&str] = &[
        "Twitter",
        "Instagram",
        "TikTok",
        "Facebook",
        "LinkedIn",
        "YouTube",
        "Reddit",
        "Discord",
        "Twitch",
        "Pinterest",
        "Snapchat",
        "Threads",
        "Mastodon",
        "Bluesky",
        "BeReal",
        "Telegram",
    ];
    PLATFORMS[rng.random_range(0..PLATFORMS.len())]
}

/// Generate a post ID.
pub fn post_id<R: Rng + ?Sized>(rng: &mut R) -> String {
    (0..19)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect()
}

/// Generate a post URL.
pub fn post_url<R: Rng + ?Sized>(rng: &mut R) -> String {
    let platforms = [
        format!(
            "https://twitter.com/{}/status/{}",
            username(rng).to_lowercase(),
            post_id(rng)
        ),
        format!("https://instagram.com/p/{}", short_code(rng)),
        format!(
            "https://tiktok.com/@{}/video/{}",
            username(rng).to_lowercase(),
            post_id(rng)
        ),
    ];
    platforms[rng.random_range(0..platforms.len())].clone()
}

fn short_code<R: Rng + ?Sized>(rng: &mut R) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";
    (0..11)
        .map(|_| CHARS[rng.random_range(0..CHARS.len())] as char)
        .collect()
}

/// Generate a verification status.
pub fn is_verified<R: Rng + ?Sized>(rng: &mut R) -> bool {
    rng.random_bool(0.05) // 5% are verified
}

/// Generate an account type.
pub fn account_type<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const TYPES: &[&str] = &[
        "personal",
        "creator",
        "business",
        "brand",
        "public_figure",
        "artist",
        "media",
        "sports",
        "government",
        "nonprofit",
    ];
    TYPES[rng.random_range(0..TYPES.len())]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn test_rng() -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(42)
    }

    #[test]
    fn test_twitter_handle() {
        let mut rng = test_rng();
        let handle = twitter_handle(&mut rng);
        assert!(handle.starts_with('@'));
    }

    #[test]
    fn test_hashtag() {
        let mut rng = test_rng();
        let tag = hashtag(&mut rng);
        assert!(tag.starts_with('#'));
    }

    #[test]
    fn test_hashtags() {
        let mut rng = test_rng();
        let tags = hashtags(&mut rng, 3);
        assert_eq!(tags.split_whitespace().count(), 3);
    }

    #[test]
    fn test_emoji() {
        let mut rng = test_rng();
        let e = emoji(&mut rng);
        assert!(!e.is_empty());
    }

    #[test]
    fn test_follower_count() {
        let mut rng = test_rng();
        for _ in 0..100 {
            let count = follower_count(&mut rng);
            assert!(count < 100_000_000);
        }
    }

    #[test]
    fn test_format_count() {
        assert_eq!(format_count(500), "500");
        assert_eq!(format_count(1500), "1.5K");
        assert_eq!(format_count(1_500_000), "1.5M");
        assert_eq!(format_count(1_500_000_000), "1.5B");
    }

    #[test]
    fn test_post_id() {
        let mut rng = test_rng();
        let id = post_id(&mut rng);
        assert_eq!(id.len(), 19);
    }

    #[test]
    fn test_discord_username_legacy() {
        let mut rng = test_rng();
        let username = discord_username_legacy(&mut rng);
        assert!(username.contains('#'));
    }
}
