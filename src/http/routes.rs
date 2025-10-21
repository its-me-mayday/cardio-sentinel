use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::{config::HttpCfg, state::AppState};

pub fn build_router(state: AppState, http_cfg: &HttpCfg) -> Router {
    let layers = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(http_cfg.body_limit_bytes))
        .layer(CorsLayer::permissive());

    Router::new()
        .route("/healthz", get(super::handlers::healthz))
        .route("/readyz", get(super::handlers::readyz))
        .with_state(state)
        .layer(layers)
}
