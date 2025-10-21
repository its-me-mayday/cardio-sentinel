//! Avvio minimale + orchestrazione (niente logica)
mod config;
mod telemetry;
mod state;
mod server;
mod http;

use crate::config::Config;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let cfg = Config::from_env();

    telemetry::init();

    tracing::info!(app=%cfg.app_name, "boot: starting");

    let app_state = AppState::new(true);

    let router = http::build_router(app_state.clone());

    let addr = cfg.addr();
    tracing::info!(%addr, "http listen");
    if let Err(e) = server::serve_with_graceful_shutdown(router, addr).await {
        tracing::error!(error=%e, "server error");
        std::process::exit(1);
    }

    tracing::info!("shutdown complete");
}
