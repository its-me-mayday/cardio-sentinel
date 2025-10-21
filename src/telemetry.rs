use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init() {
    let env_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into());

    tracing_subscriber::registry()
        .with(EnvFilter::new(env_filter))
        .with(tracing_subscriber::fmt::layer().compact())
        .init();
}