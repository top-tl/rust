# toptl

[![Crates.io](https://img.shields.io/crates/v/toptl)](https://crates.io/crates/toptl)
[![Docs.rs](https://docs.rs/toptl/badge.svg)](https://docs.rs/toptl)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Official Rust SDK for the [TOP.TL](https://top.tl) Telegram directory API.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
toptl = "1"
```

## Quick start

```rust
use toptl::TopTL;

#[tokio::main]
async fn main() -> Result<(), toptl::Error> {
    let client = TopTL::new("your-api-key");

    // Get listing info
    let listing = client.get_listing("mybotusername").await?;
    println!("{:?}", listing);

    // Check if a user has voted
    let result = client.has_voted("mybotusername", 123456789).await?;
    println!("Voted: {}", result.voted);

    // Get votes
    let votes = client.get_votes("mybotusername").await?;
    println!("Total votes: {:?}", votes.total);

    // Post stats
    use toptl::StatsPayload;
    let stats = StatsPayload {
        server_count: Some(100),
        member_count: Some(5000),
        shard_count: None,
    };
    client.post_stats("mybotusername", &stats).await?;

    // Global stats
    let global = client.get_global_stats().await?;
    println!("{:?}", global);

    Ok(())
}
```

## Autoposter

Automatically post stats on a recurring interval:

```rust
use std::sync::Arc;
use std::time::Duration;
use toptl::{TopTL, StatsPayload};
use toptl::autoposter::Autoposter;

#[tokio::main]
async fn main() {
    let client = TopTL::new("your-api-key");

    let autoposter = Autoposter::new(client, "mybotusername")
        .interval(Duration::from_secs(900)) // every 15 minutes
        .callback(Arc::new(|| StatsPayload {
            server_count: Some(1234),
            member_count: Some(56789),
            shard_count: None,
        }))
        .start();

    // Runs in the background. Call autoposter.stop().await to stop.
}
```

## Builder pattern

```rust
use toptl::TopTL;

let client = TopTL::builder("your-api-key")
    .base_url("https://top.tl/api/v1") // default
    .build();
```

## License

MIT - see [LICENSE](LICENSE) for details.
