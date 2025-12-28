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
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::limited(10))
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

    /// Fetch JSON from a DHIS2 API endpoint.
    pub fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}/api/{}", self.base_url, endpoint);

        let response = self
            .client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .with_context(|| format!("Failed to fetch {}", url))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("HTTP {}: {}", status, body);
        }

        response
            .json()
            .with_context(|| format!("Failed to parse JSON from {}", url))
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
