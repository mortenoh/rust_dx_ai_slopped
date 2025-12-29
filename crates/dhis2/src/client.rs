//! DHIS2 API client.

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use std::time::Duration;

/// DHIS2 API client with authentication.
pub struct Dhis2Client {
    client: Client,
    base_url: String,
    username: String,
    password: String,
}

impl Dhis2Client {
    /// Create a new DHIS2 client.
    pub fn new(server: &str, username: &str, password: &str) -> Result<Self> {
        // Don't auto-follow redirects - we'll handle them manually to preserve auth
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        // Ensure base URL doesn't have trailing slash
        let base_url = server.trim_end_matches('/').to_string();

        Ok(Self {
            client,
            base_url,
            username: username.to_string(),
            password: password.to_string(),
        })
    }

    /// Fetch JSON from a DHIS2 API endpoint, following redirects with auth.
    pub fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let mut url = format!("{}/api/{}", self.base_url, endpoint);

        // Follow redirects manually (up to 10) to preserve auth across hosts
        for _ in 0..10 {
            let response = self
                .client
                .get(&url)
                .basic_auth(&self.username, Some(&self.password))
                .send()
                .with_context(|| format!("Failed to fetch {}", url))?;

            if response.status().is_redirection() {
                if let Some(location) = response.headers().get("location") {
                    let location_str = location.to_str().unwrap_or("");
                    // Handle relative and absolute redirects
                    if location_str.starts_with("http") {
                        url = location_str.to_string();
                    } else if location_str.starts_with('/') {
                        // Absolute path - extract host from current URL
                        if let Ok(parsed) = reqwest::Url::parse(&url) {
                            url = format!(
                                "{}://{}{}",
                                parsed.scheme(),
                                parsed.host_str().unwrap_or(""),
                                location_str
                            );
                        }
                    } else {
                        // Relative path
                        url = format!(
                            "{}/{}",
                            url.rsplit_once('/').map(|(base, _)| base).unwrap_or(&url),
                            location_str
                        );
                    }
                    continue;
                }
            }

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().unwrap_or_default();
                anyhow::bail!("HTTP {}: {}", status, body);
            }

            return response
                .json()
                .with_context(|| format!("Failed to parse JSON from {}", url));
        }

        anyhow::bail!("Too many redirects")
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
