use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{self, Duration};

use crate::client::TopTL;
use crate::types::StatsPayload;

/// A callback that returns the current stats to post.
///
/// The callback is invoked on each interval tick to fetch fresh stats
/// before posting them to the API.
pub type StatsCallback = Arc<dyn Fn() -> StatsPayload + Send + Sync>;

/// Automatically posts stats to the TOP.TL API on a recurring interval.
///
/// # Examples
///
/// ```no_run
/// use std::sync::Arc;
/// use std::time::Duration;
/// use toptl::{TopTL, StatsPayload};
/// use toptl::autoposter::Autoposter;
///
/// #[tokio::main]
/// async fn main() {
///     let client = TopTL::new("your-api-key");
///
///     let autoposter = Autoposter::new(client, "mybotusername")
///         .interval(Duration::from_secs(900))
///         .callback(Arc::new(|| StatsPayload {
///             server_count: Some(1234),
///             member_count: Some(56789),
///             shard_count: None,
///         }))
///         .start();
///
///     // The autoposter runs in the background until stopped.
///     // autoposter.stop().await;
/// }
/// ```
pub struct Autoposter {
    client: TopTL,
    username: String,
    interval: Duration,
    callback: Option<StatsCallback>,
    handle: Option<JoinHandle<()>>,
    running: Arc<Mutex<bool>>,
}

impl Autoposter {
    /// Create a new autoposter for the given listing.
    pub fn new(client: TopTL, username: impl Into<String>) -> Self {
        Self {
            client,
            username: username.into(),
            interval: Duration::from_secs(1800), // 30 minutes default
            callback: None,
            handle: None,
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Set the posting interval. Default is 30 minutes.
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Set the callback that provides stats on each tick.
    pub fn callback(mut self, cb: StatsCallback) -> Self {
        self.callback = Some(cb);
        self
    }

    /// Start the autoposter background task. Returns `self` for chaining.
    ///
    /// # Panics
    ///
    /// Panics if no callback has been set.
    pub fn start(mut self) -> Self {
        let callback = self
            .callback
            .clone()
            .expect("A stats callback must be set before starting the autoposter");

        let client = self.client.clone();
        let username = self.username.clone();
        let interval = self.interval;
        let running = self.running.clone();

        let handle = tokio::spawn(async move {
            *running.lock().await = true;
            let mut ticker = time::interval(interval);

            loop {
                ticker.tick().await;

                if !*running.lock().await {
                    break;
                }

                let stats = (callback)();
                match client.post_stats(&username, &stats).await {
                    Ok(_) => {
                        // Stats posted successfully.
                    }
                    Err(e) => {
                        eprintln!("[toptl::autoposter] Failed to post stats: {e}");
                    }
                }
            }
        });

        self.handle = Some(handle);
        self
    }

    /// Stop the autoposter gracefully.
    pub async fn stop(self) {
        *self.running.lock().await = false;
        if let Some(handle) = self.handle {
            handle.abort();
            let _ = handle.await;
        }
    }
}
