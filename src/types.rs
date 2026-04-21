use serde::{Deserialize, Serialize};

/// A TOP.TL listing (channel, group, or bot). Field names mirror the
/// JSON the API returns; anything unknown is tolerated via `#[serde(default)]`
/// so the crate keeps compiling when the server adds new keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    /// `"CHANNEL" | "GROUP" | "BOT"`.
    pub r#type: String,
    #[serde(default)]
    pub member_count: u64,
    #[serde(default)]
    pub vote_count: u64,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub photo_url: Option<String>,
    /// Additional tags associated with the listing.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// One voter record returned by `get_votes`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Voter {
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default, alias = "createdAt")]
    pub voted_at: Option<String>,
}

/// Response for `has_voted`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteCheck {
    #[serde(default, alias = "hasVoted")]
    pub voted: bool,
    #[serde(default)]
    pub voted_at: Option<String>,
}

/// Payload for `post_stats`. All fields are optional — the server only
/// updates the ones you send, so you can push a member-count-only
/// update without zeroing out group/channel counts.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_count: Option<u64>,
    /// For bots that operate inside specific groups/channels, a list of
    /// their usernames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_serves: Option<Vec<String>>,
}

/// Response for `post_stats` / `batch_post_stats` items.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsResult {
    #[serde(default = "default_true")]
    pub success: bool,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

fn default_true() -> bool {
    true
}

/// One entry in a `batch_post_stats` request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchStatsItem {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_serves: Option<Vec<String>>,
}

/// Body for `set_webhook`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookConfig {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_title: Option<String>,
}

/// Response from `test_webhook`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookTestResult {
    #[serde(default)]
    pub success: bool,
    #[serde(default, alias = "status")]
    pub status_code: Option<u16>,
    #[serde(default, alias = "error")]
    pub message: Option<String>,
}

/// Site-wide totals from `get_global_stats`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStats {
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub channels: u64,
    #[serde(default)]
    pub groups: u64,
    #[serde(default)]
    pub bots: u64,
}
