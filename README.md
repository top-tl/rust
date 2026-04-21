# toptl

[![Crates.io](https://img.shields.io/crates/v/toptl.svg?color=3775a9)](https://crates.io/crates/toptl)
[![docs.rs](https://img.shields.io/docsrs/toptl/latest?color=3776ab)](https://docs.rs/toptl)
[![Downloads](https://img.shields.io/crates/d/toptl.svg?color=blue)](https://crates.io/crates/toptl)
[![License](https://img.shields.io/crates/l/toptl.svg?color=green)](https://github.com/top-tl/rust/blob/main/LICENSE)
[![TOP.TL](https://img.shields.io/badge/top.tl-developers-2ec4b6)](https://top.tl/developers)

The official Rust SDK for **[TOP.TL](https://top.tl)** — post bot stats, check votes, and manage vote webhooks from your Telegram bot.

## Install

```toml
[dependencies]
toptl = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Requires Rust 1.70+.

## Quick start

Get an API key at <https://top.tl/profile> → **API Keys**.

```rust
use toptl::{StatsPayload, TopTL};

#[tokio::main]
async fn main() -> Result<(), toptl::Error> {
    let client = TopTL::new("toptl_xxx");

    // Look up a listing.
    let listing = client.get_listing("durov").await?;
    println!("{} — {} votes", listing.title, listing.vote_count);

    // Post stats for a bot you own.
    client
        .post_stats(
            "mybot",
            &StatsPayload {
                member_count: Some(5_000),
                group_count: Some(1_200),
                channel_count: Some(300),
                bot_serves: None,
            },
        )
        .await?;

    // Reward users who voted.
    if client.has_voted("mybot", 123_456_789u64).await?.voted {
        // grant premium …
    }

    Ok(())
}
```

## Autoposter

Long-running bot? Register an autoposter that flushes stats on an interval:

```rust
use std::sync::Arc;
use std::time::Duration;
use toptl::{autoposter::Autoposter, StatsPayload, TopTL};

let client = TopTL::new("toptl_xxx");
let autoposter = Autoposter::new(client, "mybot")
    .interval(Duration::from_secs(30 * 60))
    .callback(Arc::new(|| StatsPayload {
        member_count: Some(current_user_count()),
        ..StatsPayload::default()
    }))
    .start();

// autoposter.stop().await; on shutdown
```

## Webhooks

```rust
use toptl::WebhookConfig;

client
    .set_webhook(
        "mybot",
        &WebhookConfig {
            url: "https://mybot.example.com/toptl-vote".into(),
            reward_title: Some("30-day premium".into()),
        },
    )
    .await?;

let result = client.test_webhook("mybot").await?;
assert!(result.success);
```

## Batch stats

Up to 25 listings per request:

```rust
use toptl::BatchStatsItem;

client
    .batch_post_stats(&[
        BatchStatsItem {
            username: "bot1".into(),
            member_count: Some(1_200),
            ..Default::default()
        },
        BatchStatsItem {
            username: "bot2".into(),
            member_count: Some(5_400),
            ..Default::default()
        },
    ])
    .await?;
```

## Error handling

All methods return `Result<T, toptl::Error>`:

```rust
use toptl::Error;

match client.post_stats("mybot", &stats).await {
    Ok(_) => {}
    Err(Error::Api { status: 401, .. }) => { /* bad key */ }
    Err(Error::Api { status: 404, .. }) => { /* no such listing */ }
    Err(Error::Api { status: 429, .. }) => { /* back off */ }
    Err(e) => eprintln!("transport: {e}"),
}
```

## License

MIT — see [`LICENSE`](LICENSE).
