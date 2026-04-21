//! # toptl
//!
//! Official Rust SDK for the [TOP.TL](https://top.tl) public API.
//!
//! ## Quick start
//!
//! ```no_run
//! use toptl::{StatsPayload, TopTL};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), toptl::Error> {
//!     let client = TopTL::new("toptl_xxx");
//!
//!     // Look up a listing
//!     let listing = client.get_listing("durov").await?;
//!     println!("{} — {} votes", listing.title, listing.vote_count);
//!
//!     // Post stats for a listing you own
//!     client
//!         .post_stats(
//!             "mybot",
//!             &StatsPayload {
//!                 member_count: Some(5_000),
//!                 group_count: Some(1_200),
//!                 channel_count: Some(300),
//!                 bot_serves: None,
//!             },
//!         )
//!         .await?;
//!
//!     // Reward voters
//!     let check = client.has_voted("mybot", 123_456_789u64).await?;
//!     if check.voted {
//!         // grant premium ...
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Autoposter
//!
//! See [`autoposter::Autoposter`] for the recurring-post helper.

pub mod autoposter;
pub mod client;
pub mod error;
pub mod types;

pub use client::{TopTL, TopTLBuilder, UserId};
pub use error::Error;
pub use types::*;
