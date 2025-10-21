use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct AppState {
    ready: Arc<RwLock<bool>>,
}

impl AppState {
    pub fn new(is_ready: bool) -> Self {
        Self { ready: Arc::new(RwLock::new(is_ready)) }
    }

    pub async fn is_ready(&self) -> bool {
        *self.ready.read().await
    }

    #[allow(dead_code)]
    pub async fn set_ready(&self, v: bool) {
        *self.ready.write().await = v;
    }
}