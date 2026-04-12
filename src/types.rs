use serde::{Deserialize, Serialize};

/// Information about a listing on TOP.TL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    /// The unique username of the listing.
    pub username: String,
    /// Display name.
    #[serde(default)]
    pub name: Option<String>,
    /// Short description.
    #[serde(default)]
    pub description: Option<String>,
    /// Category the listing belongs to.
    #[serde(default)]
    pub category: Option<String>,
    /// Total number of votes.
    #[serde(default)]
    pub votes: Option<u64>,
    /// Number of members / subscribers.
    #[serde(default)]
    pub members: Option<u64>,
    /// Avatar / photo URL.
    #[serde(default)]
    pub avatar: Option<String>,
    /// Whether the listing is verified.
    #[serde(default)]
    pub verified: Option<bool>,
    /// Tags associated with this listing.
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

/// A single vote record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// The Telegram user ID of the voter.
    #[serde(alias = "userId")]
    pub user_id: Option<u64>,
    /// Timestamp of when the vote was cast (ISO 8601 or epoch).
    #[serde(default)]
    pub timestamp: Option<String>,
}

/// Response when fetching votes for a listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotesResponse {
    /// The list of votes.
    pub votes: Vec<Vote>,
    /// Total number of votes.
    #[serde(default)]
    pub total: Option<u64>,
}

/// Response when checking if a user has voted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasVotedResponse {
    /// Whether the user has voted.
    pub voted: bool,
}

/// Payload for posting stats to a listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsPayload {
    /// Server / group count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_count: Option<u64>,
    /// Member count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    /// Shard count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<u32>,
}

/// Response after posting stats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsPostResponse {
    /// Whether the stats were accepted.
    #[serde(default)]
    pub success: Option<bool>,
    /// Optional message from the API.
    #[serde(default)]
    pub message: Option<String>,
}

/// Global platform statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStats {
    /// Total number of listings.
    #[serde(default)]
    pub total_listings: Option<u64>,
    /// Total number of votes across the platform.
    #[serde(default)]
    pub total_votes: Option<u64>,
    /// Total number of registered users.
    #[serde(default)]
    pub total_users: Option<u64>,
}
