//! # toptl
//!
//! Official Rust SDK for the [TOP.TL](https://top.tl) Telegram directory API.
//!
//! ## Quick start
//!
//! ```no_run
//! use toptl::TopTL;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), toptl::Error> {
//!     let client = TopTL::new("your-api-key");
//!
//!     // Get a listing
//!     let listing = client.get_listing("mybotusername").await?;
//!     println!("Name: {:?}", listing.name);
//!
//!     // Check if a user voted
//!     let result = client.has_voted("mybotusername", 123456789).await?;
//!     println!("Voted: {}", result.voted);
//!
//!     Ok(())
//! }
//! ```

pub mod autoposter;
pub mod client;
pub mod error;
pub mod types;

pub use client::{TopTL, TopTLBuilder};
pub use error::Error;
pub use types::*;
