use axum::{routing::get, Router};
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(super::handlers::healthz))
        .route("/readyz", get(super::handlers::readyz))
        .with_state(state)
}