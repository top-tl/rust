use std::fmt;

/// Errors that can occur when interacting with the TOP.TL API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An HTTP request failed.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// The API returned a non-success status code.
    #[error("API error (HTTP {status}): {message}")]
    Api {
        status: u16,
        message: String,
    },

    /// Failed to deserialize a response body.
    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    /// The autoposter callback is not set.
    #[error("Autoposter stats callback is not set")]
    MissingCallback,
}

/// A convenience type alias for results from this crate.
pub type Result<T> = std::result::Result<T, Error>;
