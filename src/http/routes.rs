use axum::{routing::get, Router};
use axum::http::{header::{AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method};
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
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(http_cfg.body_limit_bytes))
        // CORS deve essere l’ultimo (più vicino alle route)
        .layer(cors);

    Router::new()
        .route("/healthz", get(super::handlers::healthz))
        .route("/readyz", get(super::handlers::readyz))
        .with_state(state)
        .layer(layers)
}

// Crea un CorsLayer restrittivo partendo dalle origin in config.
// Se tra le origin c'è "*", diventa permissivo (comodo in dev).
fn make_cors_layer(http_cfg: &HttpCfg) -> CorsLayer {
    let has_wildcard = http_cfg.allowed_origins.iter().any(|o| o == "*");
    if has_wildcard {
        return CorsLayer::permissive();
    }

    // Converte le origin valide in HeaderValue (scarta quelle non valide).
    let mut origins: Vec<HeaderValue> = Vec::new();
    for o in &http_cfg.allowed_origins {
        if let Ok(v) = HeaderValue::from_str(o) {
            origins.push(v);
        } else {
            tracing::warn!(origin=%o, "invalid CORS origin ignored");
        }
    }

    CorsLayer::new()
        .allow_origin(origins) // solo queste origin
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .expose_headers([axum::http::HeaderName::from_static("x-request-id")])
}
