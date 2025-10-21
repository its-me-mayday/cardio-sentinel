use axum::{
    body::Body,
    extract::MatchedPath,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderName, HeaderValue, Method, Request,
    },
    middleware::{from_fn, Next},
    response::Response,
    routing::get,
    Router,
};
use std::time::Instant;
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
    let cors = make_cors_layer(http_cfg);

let layers = ServiceBuilder::new()
    .layer(TraceLayer::new_for_http())
    .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
    .layer(PropagateRequestIdLayer::x_request_id())
    .layer(from_fn(metrics_middleware)) 
    .layer(CompressionLayer::new())
    .layer(RequestBodyLimitLayer::new(http_cfg.body_limit_bytes))
    .layer(cors);  

    Router::new()
        .route("/healthz", get(super::handlers::healthz))
        .route("/readyz", get(super::handlers::readyz))
        .route("/metrics", get(super::handlers::metrics))
        .route("/version", get(super::handlers::version))
        .with_state(state)
        .layer(layers)
}

fn make_cors_layer(http_cfg: &HttpCfg) -> CorsLayer {
    if http_cfg.allowed_origins.iter().any(|o| o == "*") {
        return CorsLayer::permissive();
    }
    let mut origins: Vec<HeaderValue> = Vec::new();
    for o in &http_cfg.allowed_origins {
        match HeaderValue::from_str(o) {
            Ok(v) => origins.push(v),
            Err(_) => tracing::warn!(origin=%o, "invalid CORS origin ignored"),
        }
    }
    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .expose_headers([HeaderName::from_static("x-request-id")])
}

async fn metrics_middleware(req: Request<Body>, next: Next) -> Response {
    let method = req.method().as_str().to_string();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| req.uri().path().to_string());

    let start = Instant::now();
    let resp = next.run(req).await;

    let status = resp.status().as_u16();
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

    let c = metrics::counter!("http_requests_total");
    c.increment(1);

    tracing::debug!(%method, %route, status, elapsed_ms, "request completed");

    resp
}
