mod config;
mod telemetry;
mod state;
mod server;
mod http;

use crate::config::Config;
use crate::state::AppState;

#[tokio::main]
async fn main() {
     let cfg = match Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("config error: {e}");
            std::process::exit(78);
        }
    };

    telemetry::init(&cfg.telemetry);
    tracing::info!(app=%cfg.app_name, http=?cfg.http, "boot: starting");


    let app_state = AppState::new(true);

    let router = http::build_router(app_state.clone(), &cfg.http);

    let addr = cfg.http.bind_addr();
    tracing::info!(%addr, "http listen");
    if let Err(e) = server::serve_with_graceful_shutdown(router, addr).await {
        tracing::error!(error=%e, "server error");
        std::process::exit(1);
    }

    tracing::info!("shutdown complete");
}
