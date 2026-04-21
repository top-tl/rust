use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{self, Duration};

use crate::client::TopTL;
use crate::types::StatsPayload;

/// Callback that produces the current stats to post. Invoked on every
/// autoposter tick.
pub type StatsCallback = Arc<dyn Fn() -> StatsPayload + Send + Sync>;

/// Background task that calls [`TopTL::post_stats`] on an interval.
///
/// # Example
///
/// ```no_run
/// use std::sync::Arc;
/// use std::time::Duration;
/// use toptl::{StatsPayload, TopTL};
/// use toptl::autoposter::Autoposter;
///
/// #[tokio::main]
/// async fn main() {
///     let client = TopTL::new("toptl_xxx");
///
///     let autoposter = Autoposter::new(client, "mybot")
///         .interval(Duration::from_secs(30 * 60))
///         .callback(Arc::new(|| StatsPayload {
///             member_count: Some(5_000),
///             group_count: Some(1_200),
///             channel_count: Some(300),
///             bot_serves: None,
///         }))
///         .start();
///
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
    pub fn new(client: TopTL, username: impl Into<String>) -> Self {
        Self {
            client,
            username: username.into(),
            interval: Duration::from_secs(1800), // 30 min
            callback: None,
            handle: None,
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    pub fn callback(mut self, cb: StatsCallback) -> Self {
        self.callback = Some(cb);
        self
    }

    /// Start the background task. Panics if no callback is set.
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
                if let Err(e) = client.post_stats(&username, &stats).await {
                    // Keep going — transient errors (network, 5xx) shouldn't
                    // kill the loop. Consumers can log their own way via the
                    // `log` / `tracing` crate by wrapping post_stats themselves.
                    eprintln!("[toptl::autoposter] post_stats failed: {e}");
                }
            }
        });

        self.handle = Some(handle);
        self
    }

    pub async fn stop(self) {
        *self.running.lock().await = false;
        if let Some(handle) = self.handle {
            handle.abort();
            let _ = handle.await;
        }
    }
}
