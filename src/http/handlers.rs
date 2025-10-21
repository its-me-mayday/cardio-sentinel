use axum::{extract::State, http::StatusCode};
use crate::state::AppState;

pub async fn healthz() -> &'static str {
    "OK"
}

pub async fn readyz(State(state): State<AppState>) -> (StatusCode, &'static str) {
    if state.is_ready().await {
        (StatusCode::OK, "READY")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "NOT_READY")
    }
}