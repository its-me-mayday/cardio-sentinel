use axum::{extract::State, http::StatusCode};
use crate::state::AppState;
use tokio::time::timeout;

pub async fn healthz() -> &'static str {
    "OK"
}

pub async fn readyz(State(state): State<AppState>) -> (StatusCode, &'static str) {
    // Applichiamo un timeout alla verifica di readiness
    let dur = state.request_timeout();

    match timeout(dur, state.is_ready()).await {
        Ok(ready) => {
            if ready {
                (StatusCode::OK, "READY")
            } else {
                (StatusCode::SERVICE_UNAVAILABLE, "NOT_READY")
            }
        }
        Err(_elapsed) => {
            // Timeout scattato → 408 per questa route
            (StatusCode::REQUEST_TIMEOUT, "READY_TIMEOUT")
        }
    }
}