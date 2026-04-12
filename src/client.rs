use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;

use crate::error::{Error, Result};
use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://top.tl/api/v1";

/// The main client for interacting with the TOP.TL API.
///
/// # Examples
///
/// ```no_run
/// use toptl::TopTL;
///
/// #[tokio::main]
/// async fn main() {
///     let client = TopTL::new("your-api-key");
///     let listing = client.get_listing("mybotusername").await.unwrap();
///     println!("{:?}", listing);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TopTL {
    http: Client,
    base_url: String,
}

impl TopTL {
    /// Create a new TOP.TL client with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        TopTLBuilder::new(api_key).build()
    }

    /// Returns a [`TopTLBuilder`] for advanced configuration.
    pub fn builder(api_key: impl Into<String>) -> TopTLBuilder {
        TopTLBuilder::new(api_key)
    }

    /// Get information about a listing by its username.
    pub async fn get_listing(&self, username: &str) -> Result<Listing> {
        let url = format!("{}/listing/{}", self.base_url, username);
        self.get(&url).await
    }

    /// Get votes for a listing.
    pub async fn get_votes(&self, username: &str) -> Result<VotesResponse> {
        let url = format!("{}/listing/{}/votes", self.base_url, username);
        self.get(&url).await
    }

    /// Check whether a specific Telegram user has voted for a listing.
    pub async fn has_voted(&self, username: &str, user_id: u64) -> Result<HasVotedResponse> {
        let url = format!(
            "{}/listing/{}/has-voted/{}",
            self.base_url, username, user_id
        );
        self.get(&url).await
    }

    /// Post stats (server count, member count, etc.) for a listing.
    pub async fn post_stats(
        &self,
        username: &str,
        stats: &StatsPayload,
    ) -> Result<StatsPostResponse> {
        let url = format!("{}/listing/{}/stats", self.base_url, username);
        let response = self
            .http
            .post(&url)
            .json(stats)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            return Err(Error::Api { status, message });
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(Error::from)
    }

    /// Get global TOP.TL platform statistics.
    pub async fn get_global_stats(&self) -> Result<GlobalStats> {
        let url = format!("{}/stats", self.base_url);
        self.get(&url).await
    }

    /// Internal helper for GET requests.
    async fn get<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self.http.get(url).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            return Err(Error::Api { status, message });
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(Error::from)
    }
}

/// Builder for constructing a [`TopTL`] client with custom options.
pub struct TopTLBuilder {
    api_key: String,
    base_url: String,
}

impl TopTLBuilder {
    /// Create a new builder with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    /// Override the base URL (useful for testing).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Build the [`TopTL`] client.
    pub fn build(self) -> TopTL {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", self.api_key);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).expect("Invalid API key characters"),
        );

        let http = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        TopTL {
            http,
            base_url: self.base_url,
        }
    }
}
