use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;
use metrics_exporter_prometheus::PrometheusHandle;

#[derive(Clone)]
pub struct AppState {
    ready: Arc<RwLock<bool>>,
    request_timeout: Duration,
    metrics_handle: PrometheusHandle,
}

impl AppState {
    pub fn new(is_ready: bool, request_timeout: Duration, metrics_handle: PrometheusHandle) -> Self {
        Self {
            ready: Arc::new(RwLock::new(is_ready)),
            request_timeout,
            metrics_handle,
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

    pub fn metrics(&self) -> &PrometheusHandle {
        &self.metrics_handle
    }
}