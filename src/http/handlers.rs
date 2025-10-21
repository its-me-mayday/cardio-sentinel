use axum::{extract::State, http::StatusCode, Json};
use tokio::time::timeout;

use crate::state::AppState;

pub async fn healthz() -> &'static str {
    "OK"
}

pub async fn readyz(State(state): State<AppState>) -> (StatusCode, &'static str) {
    let dur = state.request_timeout();
    match timeout(dur, state.is_ready()).await {
        Ok(ready) => {
            if ready {
                (StatusCode::OK, "READY")
            } else {
                (StatusCode::SERVICE_UNAVAILABLE, "NOT_READY")
            }
        }
        Err(_) => (StatusCode::REQUEST_TIMEOUT, "READY_TIMEOUT"),
    }
}

pub async fn metrics(State(state): State<AppState>) -> (StatusCode, ([(axum::http::header::HeaderName, String); 1], String)) {
    // Esponi text/plain in formato Prometheus
    let body = state.metrics().render();
    let headers = [(axum::http::header::CONTENT_TYPE, "text/plain; version=0.0.4".to_string())];
    (StatusCode::OK, (headers, body))
}

#[derive(serde::Serialize)]
pub(super) struct VersionResponse {
    name: &'static str,
    version: &'static str,
    git_sha: &'static str,
    build_time_unix: &'static str,
}

pub async fn version() -> Json<VersionResponse> {
    // Questi valori arrivano da Cargo o da build.rs (vedi step 5)
    Json(VersionResponse {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        git_sha: option_env!("GIT_SHA").unwrap_or("unknown"),
        build_time_unix: option_env!("BUILD_TIME_UNIX").unwrap_or("unknown"),
    })
}