use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{Error, Result};
use crate::types::*;

/// Default base URL — the `/api` prefix is included so per-method paths
/// only need to supply `/v1/...`.
const DEFAULT_BASE_URL: &str = "https://top.tl/api";
const DEFAULT_USER_AGENT: &str = concat!("toptl-rust/", env!("CARGO_PKG_VERSION"));

/// Async client for the TOP.TL public API.
#[derive(Debug, Clone)]
pub struct TopTL {
    http: Client,
    base_url: String,
}

impl TopTL {
    /// Create a client with the given API key. See [`TopTLBuilder`] for
    /// base-URL / timeout overrides.
    pub fn new(api_key: impl Into<String>) -> Self {
        TopTLBuilder::new(api_key).build()
    }

    pub fn builder(api_key: impl Into<String>) -> TopTLBuilder {
        TopTLBuilder::new(api_key)
    }

    // ---- Listings ------------------------------------------------------

    pub async fn get_listing(&self, username: &str) -> Result<Listing> {
        self.request(Method::GET, &format!("/v1/listing/{username}"), None::<&()>)
            .await
    }

    /// Recent voters for a listing. `limit` defaults server-side to 20 if
    /// `None` is passed.
    pub async fn get_votes(&self, username: &str, limit: Option<u32>) -> Result<Vec<Voter>> {
        let mut path = format!("/v1/listing/{username}/votes");
        if let Some(n) = limit {
            path.push_str(&format!("?limit={n}"));
        }
        // The server returns either a bare array or `{items: [...]}` depending
        // on version; accept both shapes.
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum VotesBody {
            List(Vec<Voter>),
            Wrapped { items: Vec<Voter> },
        }
        let body: VotesBody = self
            .request(Method::GET, &path, None::<&()>)
            .await?;
        Ok(match body {
            VotesBody::List(v) => v,
            VotesBody::Wrapped { items } => items,
        })
    }

    pub async fn has_voted(
        &self,
        username: &str,
        user_id: impl Into<UserId>,
    ) -> Result<VoteCheck> {
        let user_id = user_id.into();
        self.request(
            Method::GET,
            &format!("/v1/listing/{username}/has-voted/{user_id}"),
            None::<&()>,
        )
        .await
    }

    // ---- Stats ---------------------------------------------------------

    pub async fn post_stats(
        &self,
        username: &str,
        stats: &StatsPayload,
    ) -> Result<StatsResult> {
        self.request(
            Method::POST,
            &format!("/v1/listing/{username}/stats"),
            Some(stats),
        )
        .await
    }

    /// Post stats for up to 25 listings in one call.
    pub async fn batch_post_stats(
        &self,
        items: &[BatchStatsItem],
    ) -> Result<Vec<StatsResult>> {
        self.request(Method::POST, "/v1/stats/batch", Some(items)).await
    }

    pub async fn get_global_stats(&self) -> Result<GlobalStats> {
        self.request(Method::GET, "/v1/stats", None::<&()>).await
    }

    // ---- Webhooks ------------------------------------------------------

    pub async fn set_webhook(&self, username: &str, config: &WebhookConfig) -> Result<WebhookConfig> {
        self.request(
            Method::PUT,
            &format!("/v1/listing/{username}/webhook"),
            Some(config),
        )
        .await
    }

    pub async fn test_webhook(&self, username: &str) -> Result<WebhookTestResult> {
        self.request(
            Method::POST,
            &format!("/v1/listing/{username}/webhook/test"),
            None::<&()>,
        )
        .await
    }

    // ---- Internal ------------------------------------------------------

    async fn request<B, T>(&self, method: Method, path: &str, body: Option<&B>) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.http.request(method, &url);
        if let Some(b) = body {
            req = req.json(b);
        }
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            // Prefer server's error message when it's JSON with a `message` key.
            let message = serde_json::from_str::<serde_json::Value>(&text)
                .ok()
                .and_then(|v| v.get("message").and_then(|m| m.as_str()).map(str::to_owned))
                .unwrap_or(text);
            return Err(Error::Api {
                status: status.as_u16(),
                message,
            });
        }
        serde_json::from_str(&text).map_err(Error::from)
    }
}

/// Small wrapper so `has_voted` accepts both numeric Telegram IDs and
/// pre-stringified values.
pub struct UserId(String);

impl From<u64> for UserId {
    fn from(v: u64) -> Self { UserId(v.to_string()) }
}
impl From<i64> for UserId {
    fn from(v: i64) -> Self { UserId(v.to_string()) }
}
impl From<&str> for UserId {
    fn from(v: &str) -> Self { UserId(v.to_owned()) }
}
impl From<String> for UserId {
    fn from(v: String) -> Self { UserId(v) }
}
impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Builder for constructing a [`TopTL`] client with custom options.
pub struct TopTLBuilder {
    api_key: String,
    base_url: String,
    user_agent: Option<String>,
}

impl TopTLBuilder {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_owned(),
            user_agent: None,
        }
    }

    /// Override the base URL (useful for staging / self-hosted).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into().trim_end_matches('/').to_owned();
        self
    }

    /// Append a custom user-agent suffix (e.g. `"mybot/1.0"`).
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    pub fn build(self) -> TopTL {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .expect("Invalid API key characters"),
        );
        let ua = match self.user_agent {
            Some(suffix) => format!("{DEFAULT_USER_AGENT} {suffix}"),
            None => DEFAULT_USER_AGENT.to_owned(),
        };
        headers.insert(USER_AGENT, HeaderValue::from_str(&ua).expect("bad UA"));

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
