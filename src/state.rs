use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    ready: Arc<RwLock<bool>>,
    request_timeout: Duration,
}

impl AppState {
    pub fn new(is_ready: bool, request_timeout: Duration) -> Self {
        Self {
            ready: Arc::new(RwLock::new(is_ready)),
            request_timeout,
        }
    }

    pub async fn is_ready(&self) -> bool {
        *self.ready.read().await
    }

    #[allow(dead_code)]
    pub async fn set_ready(&self, v: bool) {
        *self.ready.write().await = v;
    }

    pub fn request_timeout(&self) -> Duration {
        self.request_timeout
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(true, Duration::from_secs(10))
    }
}