//! File and system data generation utilities.
//!
//! This module provides generators for:
//! - File names and extensions
//! - MIME types
//! - Directory and file paths
//! - Semantic version strings
//! - User agent strings
//!
//! # Example
//!
//! ```
//! use dx_datagen::file;
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//! let ext = file::file_extension(&mut rng);
//! let mime = file::mime_type(&mut rng);
//! let name = file::file_name(&mut rng);
//! ```

use rand::Rng;

/// Common file extensions by category
pub mod extensions {
    pub const DOCUMENT: &[&str] = &[
        "pdf", "doc", "docx", "txt", "rtf", "odt", "xls", "xlsx", "ppt", "pptx",
    ];
    pub const IMAGE: &[&str] = &[
        "jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico", "tiff",
    ];
    pub const VIDEO: &[&str] = &["mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "m4v"];
    pub const AUDIO: &[&str] = &["mp3", "wav", "flac", "aac", "ogg", "wma", "m4a"];
    pub const ARCHIVE: &[&str] = &["zip", "tar", "gz", "rar", "7z", "bz2", "xz"];
    pub const CODE: &[&str] = &[
        "rs", "py", "js", "ts", "java", "cpp", "c", "h", "go", "rb", "php", "html", "css",
    ];
    pub const DATA: &[&str] = &["json", "xml", "yaml", "yml", "csv", "toml", "ini"];
    pub const ALL: &[&str] = &[
        "pdf", "doc", "docx", "txt", "rtf", "odt", "xls", "xlsx", "ppt", "pptx", "jpg", "jpeg",
        "png", "gif", "bmp", "svg", "webp", "mp4", "avi", "mkv", "mov", "webm", "mp3", "wav",
        "flac", "aac", "ogg", "zip", "tar", "gz", "rar", "7z", "rs", "py", "js", "ts", "java",
        "go", "json", "xml", "yaml", "csv", "toml",
    ];
}

/// MIME types by category
pub mod mime_types {
    pub const APPLICATION: &[&str] = &[
        "application/json",
        "application/xml",
        "application/pdf",
        "application/zip",
        "application/gzip",
        "application/octet-stream",
        "application/javascript",
        "application/x-www-form-urlencoded",
        "application/x-tar",
        "application/x-rar-compressed",
        "application/x-7z-compressed",
        "application/vnd.ms-excel",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "application/msword",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    ];

    pub const TEXT: &[&str] = &[
        "text/plain",
        "text/html",
        "text/css",
        "text/javascript",
        "text/csv",
        "text/xml",
        "text/markdown",
        "text/calendar",
    ];

    pub const IMAGE: &[&str] = &[
        "image/jpeg",
        "image/png",
        "image/gif",
        "image/webp",
        "image/svg+xml",
        "image/bmp",
        "image/tiff",
        "image/x-icon",
    ];

    pub const AUDIO: &[&str] = &[
        "audio/mpeg",
        "audio/wav",
        "audio/ogg",
        "audio/flac",
        "audio/aac",
        "audio/webm",
        "audio/mp4",
    ];

    pub const VIDEO: &[&str] = &[
        "video/mp4",
        "video/webm",
        "video/ogg",
        "video/x-msvideo",
        "video/quicktime",
        "video/x-matroska",
        "video/x-flv",
    ];

    pub const ALL: &[&str] = &[
        "application/json",
        "application/xml",
        "application/pdf",
        "application/zip",
        "application/octet-stream",
        "text/plain",
        "text/html",
        "text/css",
        "text/csv",
        "image/jpeg",
        "image/png",
        "image/gif",
        "image/webp",
        "image/svg+xml",
        "audio/mpeg",
        "audio/wav",
        "audio/ogg",
        "video/mp4",
        "video/webm",
    ];
}

/// Common file name prefixes
const FILE_PREFIXES: &[&str] = &[
    "report",
    "document",
    "file",
    "data",
    "export",
    "backup",
    "log",
    "image",
    "photo",
    "screenshot",
    "video",
    "audio",
    "music",
    "presentation",
    "spreadsheet",
    "notes",
    "readme",
    "config",
    "archive",
    "download",
    "upload",
    "temp",
    "draft",
    "final",
];

/// User agent browser templates
const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPad; CPU OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (Linux; Android 14; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
];

/// File extension category
#[derive(Debug, Clone, Copy)]
pub enum ExtensionCategory {
    Document,
    Image,
    Video,
    Audio,
    Archive,
    Code,
    Data,
    Any,
}

/// MIME type category
#[derive(Debug, Clone, Copy)]
pub enum MimeCategory {
    Application,
    Text,
    Image,
    Audio,
    Video,
    Any,
}

/// Generate a random file extension.
///
/// # Example
/// ```
/// use dx_datagen::file::file_extension;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let ext = file_extension(&mut rng);
/// assert!(!ext.is_empty());
/// ```
pub fn file_extension<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..extensions::ALL.len());
    extensions::ALL[idx]
}

/// Generate a file extension by category.
///
/// # Example
/// ```
/// use dx_datagen::file::{file_extension_by_category, ExtensionCategory};
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let ext = file_extension_by_category(&mut rng, ExtensionCategory::Image);
/// assert!(["jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico", "tiff"].contains(&ext));
/// ```
pub fn file_extension_by_category<R: ?Sized + Rng>(
    rng: &mut R,
    category: ExtensionCategory,
) -> &'static str {
    let exts = match category {
        ExtensionCategory::Document => extensions::DOCUMENT,
        ExtensionCategory::Image => extensions::IMAGE,
        ExtensionCategory::Video => extensions::VIDEO,
        ExtensionCategory::Audio => extensions::AUDIO,
        ExtensionCategory::Archive => extensions::ARCHIVE,
        ExtensionCategory::Code => extensions::CODE,
        ExtensionCategory::Data => extensions::DATA,
        ExtensionCategory::Any => extensions::ALL,
    };
    let idx = rng.random_range(0..exts.len());
    exts[idx]
}

/// Generate a random MIME type.
///
/// # Example
/// ```
/// use dx_datagen::file::mime_type;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mime = mime_type(&mut rng);
/// assert!(mime.contains('/'));
/// ```
pub fn mime_type<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..mime_types::ALL.len());
    mime_types::ALL[idx]
}

/// Generate a MIME type by category.
///
/// # Example
/// ```
/// use dx_datagen::file::{mime_type_by_category, MimeCategory};
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mime = mime_type_by_category(&mut rng, MimeCategory::Image);
/// assert!(mime.starts_with("image/"));
/// ```
pub fn mime_type_by_category<R: ?Sized + Rng>(rng: &mut R, category: MimeCategory) -> &'static str {
    let types = match category {
        MimeCategory::Application => mime_types::APPLICATION,
        MimeCategory::Text => mime_types::TEXT,
        MimeCategory::Image => mime_types::IMAGE,
        MimeCategory::Audio => mime_types::AUDIO,
        MimeCategory::Video => mime_types::VIDEO,
        MimeCategory::Any => mime_types::ALL,
    };
    let idx = rng.random_range(0..types.len());
    types[idx]
}

/// Generate a random file name with extension.
///
/// # Example
/// ```
/// use dx_datagen::file::file_name;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let name = file_name(&mut rng);
/// assert!(name.contains('.'));
/// ```
pub fn file_name<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = FILE_PREFIXES[rng.random_range(0..FILE_PREFIXES.len())];
    let suffix: u32 = rng.random_range(1..1000);
    let ext = file_extension(rng);
    format!("{}_{}.{}", prefix, suffix, ext)
}

/// Generate a random file name with a specific extension.
pub fn file_name_with_extension<R: ?Sized + Rng>(rng: &mut R, ext: &str) -> String {
    let prefix = FILE_PREFIXES[rng.random_range(0..FILE_PREFIXES.len())];
    let suffix: u32 = rng.random_range(1..1000);
    format!("{}_{}.{}", prefix, suffix, ext)
}

/// Generate a random directory path (Unix-style).
///
/// # Example
/// ```
/// use dx_datagen::file::directory_path;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let path = directory_path(&mut rng);
/// assert!(path.starts_with('/'));
/// ```
pub fn directory_path<R: ?Sized + Rng>(rng: &mut R) -> String {
    const DIRS: &[&str] = &[
        "home",
        "var",
        "usr",
        "opt",
        "etc",
        "tmp",
        "documents",
        "downloads",
        "pictures",
        "music",
        "videos",
        "projects",
        "workspace",
        "src",
        "lib",
        "bin",
        "data",
        "logs",
        "cache",
        "config",
        "backup",
    ];

    let depth = rng.random_range(2..5);
    let mut path = String::from("/");

    for i in 0..depth {
        let dir = DIRS[rng.random_range(0..DIRS.len())];
        path.push_str(dir);
        if i < depth - 1 {
            path.push('/');
        }
    }

    path
}

/// Generate a random file path (Unix-style).
///
/// # Example
/// ```
/// use dx_datagen::file::file_path;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let path = file_path(&mut rng);
/// assert!(path.starts_with('/'));
/// assert!(path.contains('.'));
/// ```
pub fn file_path<R: ?Sized + Rng>(rng: &mut R) -> String {
    let dir = directory_path(rng);
    let name = file_name(rng);
    format!("{}/{}", dir, name)
}

/// Generate a random semantic version string.
///
/// # Example
/// ```
/// use dx_datagen::file::semver;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let version = semver(&mut rng);
/// assert!(version.matches('.').count() == 2);
/// ```
pub fn semver<R: ?Sized + Rng>(rng: &mut R) -> String {
    let major: u32 = rng.random_range(0..10);
    let minor: u32 = rng.random_range(0..20);
    let patch: u32 = rng.random_range(0..50);
    format!("{}.{}.{}", major, minor, patch)
}

/// Generate a semantic version with optional prerelease tag.
pub fn semver_with_prerelease<R: ?Sized + Rng>(rng: &mut R) -> String {
    let base = semver(rng);
    let has_prerelease = rng.random_bool(0.3);

    if has_prerelease {
        let tags = ["alpha", "beta", "rc", "dev"];
        let tag = tags[rng.random_range(0..tags.len())];
        let num: u32 = rng.random_range(1..10);
        format!("{}-{}.{}", base, tag, num)
    } else {
        base
    }
}

/// Generate a random user agent string.
///
/// # Example
/// ```
/// use dx_datagen::file::user_agent;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let ua = user_agent(&mut rng);
/// assert!(ua.starts_with("Mozilla/5.0"));
/// ```
pub fn user_agent<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    let idx = rng.random_range(0..USER_AGENTS.len());
    USER_AGENTS[idx]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_file_extension() {
        let mut rng = StdRng::seed_from_u64(42);
        let ext = file_extension(&mut rng);
        assert!(extensions::ALL.contains(&ext));
    }

    #[test]
    fn test_file_extension_by_category() {
        let mut rng = StdRng::seed_from_u64(42);
        let ext = file_extension_by_category(&mut rng, ExtensionCategory::Image);
        assert!(extensions::IMAGE.contains(&ext));
    }

    #[test]
    fn test_mime_type() {
        let mut rng = StdRng::seed_from_u64(42);
        let mime = mime_type(&mut rng);
        assert!(mime.contains('/'));
        assert!(mime_types::ALL.contains(&mime));
    }

    #[test]
    fn test_mime_type_by_category() {
        let mut rng = StdRng::seed_from_u64(42);
        let mime = mime_type_by_category(&mut rng, MimeCategory::Image);
        assert!(mime.starts_with("image/"));
    }

    #[test]
    fn test_file_name() {
        let mut rng = StdRng::seed_from_u64(42);
        let name = file_name(&mut rng);
        assert!(name.contains('.'));
        assert!(name.contains('_'));
    }

    #[test]
    fn test_directory_path() {
        let mut rng = StdRng::seed_from_u64(42);
        let path = directory_path(&mut rng);
        assert!(path.starts_with('/'));
    }

    #[test]
    fn test_file_path() {
        let mut rng = StdRng::seed_from_u64(42);
        let path = file_path(&mut rng);
        assert!(path.starts_with('/'));
        assert!(path.contains('.'));
    }

    #[test]
    fn test_semver() {
        let mut rng = StdRng::seed_from_u64(42);
        let version = semver(&mut rng);
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts.iter().all(|p| p.parse::<u32>().is_ok()));
    }

    #[test]
    fn test_user_agent() {
        let mut rng = StdRng::seed_from_u64(42);
        let ua = user_agent(&mut rng);
        assert!(ua.starts_with("Mozilla/5.0"));
    }

    #[test]
    fn test_determinism() {
        let mut rng1 = StdRng::seed_from_u64(123);
        let mut rng2 = StdRng::seed_from_u64(123);

        assert_eq!(file_extension(&mut rng1), file_extension(&mut rng2));
        assert_eq!(mime_type(&mut rng1), mime_type(&mut rng2));
        assert_eq!(semver(&mut rng1), semver(&mut rng2));
    }

    #[test]
    fn test_trait_object() {
        use rand::RngCore;
        let mut rng: Box<dyn RngCore> = Box::new(StdRng::seed_from_u64(42));
        let mime = mime_type(&mut *rng);
        assert!(mime.contains('/'));
    }
}
