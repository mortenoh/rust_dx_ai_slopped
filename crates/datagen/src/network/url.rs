//! URL generation.
//!
//! Generate random URLs with various components.
//!
//! # Example
//!
//! ```
//! use dx_datagen::network::url::{url, url_https, url_with_path};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let u = url(&mut rng);
//! let secure = url_https(&mut rng);
//! let with_path = url_with_path(&mut rng, 3);
//! ```

use super::domain::{domain, subdomain};
use crate::generators::alphanumeric;
use crate::text::words::noun;
use rand::Rng;

/// Common URL path segments.
pub const PATH_SEGMENTS: &[&str] = &[
    "api",
    "v1",
    "v2",
    "users",
    "products",
    "orders",
    "items",
    "posts",
    "comments",
    "pages",
    "search",
    "auth",
    "login",
    "logout",
    "register",
    "profile",
    "settings",
    "dashboard",
    "admin",
    "data",
    "assets",
    "images",
    "files",
    "docs",
    "help",
    "about",
    "contact",
    "blog",
    "news",
];

/// Common file extensions for URLs.
pub const URL_EXTENSIONS: &[&str] = &[
    "html", "htm", "php", "asp", "aspx", "jsp", "json", "xml", "js", "css", "png", "jpg", "gif",
    "pdf", "txt",
];

/// Common query parameter names.
pub const QUERY_PARAMS: &[&str] = &[
    "id",
    "page",
    "limit",
    "offset",
    "sort",
    "order",
    "q",
    "query",
    "search",
    "filter",
    "type",
    "category",
    "status",
    "token",
    "key",
    "format",
    "lang",
    "locale",
    "ref",
    "source",
    "utm_source",
];

/// Generate a random URL (http or https).
pub fn url<R: ?Sized + Rng>(rng: &mut R) -> String {
    let protocol = if rng.random_bool(0.7) {
        "https"
    } else {
        "http"
    };
    format!("{}://{}", protocol, domain(rng))
}

/// Generate an HTTPS URL.
pub fn url_https<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("https://{}", domain(rng))
}

/// Generate an HTTP URL.
pub fn url_http<R: ?Sized + Rng>(rng: &mut R) -> String {
    format!("http://{}", domain(rng))
}

/// Generate a URL with a path.
pub fn url_with_path<R: ?Sized + Rng>(rng: &mut R, segments: usize) -> String {
    let base = url(rng);
    let path = generate_path(rng, segments);
    format!("{}{}", base, path)
}

/// Generate a URL with query parameters.
pub fn url_with_query<R: ?Sized + Rng>(rng: &mut R, param_count: usize) -> String {
    let path_segments = rng.random_range(0..3);
    let base = url_with_path(rng, path_segments);
    if param_count == 0 {
        return base;
    }

    let mut params: Vec<String> = Vec::with_capacity(param_count);
    for _ in 0..param_count {
        let name = QUERY_PARAMS[rng.random_range(0..QUERY_PARAMS.len())];
        let len = rng.random_range(3..10);
        let value = alphanumeric(rng, len);
        params.push(format!("{}={}", name, value));
    }

    format!("{}?{}", base, params.join("&"))
}

/// Generate a URL with a fragment/anchor.
pub fn url_with_fragment<R: ?Sized + Rng>(rng: &mut R) -> String {
    let path_segments = rng.random_range(1..3);
    let base = url_with_path(rng, path_segments);
    let fragment = noun(rng);
    format!("{}#{}", base, fragment)
}

/// Generate a URL path.
pub fn generate_path<R: ?Sized + Rng>(rng: &mut R, segments: usize) -> String {
    if segments == 0 {
        return String::new();
    }

    let path_parts: Vec<&str> = (0..segments)
        .map(|_| PATH_SEGMENTS[rng.random_range(0..PATH_SEGMENTS.len())])
        .collect();

    format!("/{}", path_parts.join("/"))
}

/// Generate a URL with all components.
pub fn full_url<R: ?Sized + Rng>(rng: &mut R) -> String {
    let protocol = if rng.random_bool(0.8) {
        "https"
    } else {
        "http"
    };
    let sub = if rng.random_bool(0.5) {
        format!("{}.", subdomain(rng))
    } else {
        String::new()
    };
    let dom = domain(rng);
    let path_segments = rng.random_range(0..4);
    let path = generate_path(rng, path_segments);

    let query = if rng.random_bool(0.3) {
        let count = rng.random_range(1..4);
        let mut params: Vec<String> = Vec::with_capacity(count);
        for _ in 0..count {
            let name = QUERY_PARAMS[rng.random_range(0..QUERY_PARAMS.len())];
            let len = rng.random_range(3..8);
            let value = alphanumeric(rng, len);
            params.push(format!("{}={}", name, value));
        }
        format!("?{}", params.join("&"))
    } else {
        String::new()
    };

    format!("{}://{}{}{}{}", protocol, sub, dom, path, query)
}

/// Generate an API endpoint URL.
pub fn api_url<R: ?Sized + Rng>(rng: &mut R) -> String {
    let dom = domain(rng);
    let version = rng.random_range(1..4);
    let resource = PATH_SEGMENTS[rng.random_range(0..10)]; // Use common API resources
    format!("https://api.{}/v{}/{}", dom, version, resource)
}

/// Generate a file URL.
pub fn file_url<R: ?Sized + Rng>(rng: &mut R) -> String {
    let path_segments = rng.random_range(1..3);
    let base = url_with_path(rng, path_segments);
    let filename = noun(rng);
    let ext = URL_EXTENSIONS[rng.random_range(0..URL_EXTENSIONS.len())];
    format!("{}/{}.{}", base, filename, ext)
}

/// Generate an image URL.
pub fn image_url<R: ?Sized + Rng>(rng: &mut R) -> String {
    let dom = domain(rng);
    let width = rng.random_range(100..2000);
    let height = rng.random_range(100..2000);
    format!("https://{}/images/{}x{}.jpg", dom, width, height)
}

/// Generate a localhost URL.
pub fn localhost_url<R: ?Sized + Rng>(rng: &mut R) -> String {
    let port = rng.random_range(3000..9000);
    format!("http://localhost:{}", port)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_url() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = url(&mut rng);
        assert!(u.starts_with("http://") || u.starts_with("https://"));
    }

    #[test]
    fn test_url_https() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = url_https(&mut rng);
        assert!(u.starts_with("https://"));
    }

    #[test]
    fn test_url_with_path() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = url_with_path(&mut rng, 3);
        let path_count = u.matches('/').count();
        assert!(path_count >= 4); // protocol:// + 3 path segments
    }

    #[test]
    fn test_url_with_query() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = url_with_query(&mut rng, 2);
        assert!(u.contains('?'));
        assert!(u.contains('='));
    }

    #[test]
    fn test_url_with_fragment() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = url_with_fragment(&mut rng);
        assert!(u.contains('#'));
    }

    #[test]
    fn test_api_url() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = api_url(&mut rng);
        assert!(u.starts_with("https://api."));
        assert!(u.contains("/v"));
    }

    #[test]
    fn test_file_url() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = file_url(&mut rng);
        // Should end with a file extension
        let has_ext = URL_EXTENSIONS
            .iter()
            .any(|ext| u.ends_with(&format!(".{}", ext)));
        assert!(has_ext, "URL should have extension: {}", u);
    }

    #[test]
    fn test_localhost_url() {
        let mut rng = StdRng::seed_from_u64(42);
        let u = localhost_url(&mut rng);
        assert!(u.starts_with("http://localhost:"));
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        assert_eq!(url(&mut rng1), url(&mut rng2));
    }

    #[test]
    fn test_trait_object_support() {
        let mut rng: Box<dyn rand::RngCore> = Box::new(StdRng::seed_from_u64(42));
        let u = url(&mut *rng);
        assert!(u.starts_with("http"));
    }
}
