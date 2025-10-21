use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::TelemetryCfg;

pub fn init(tcfg: &TelemetryCfg) {
    let filter = EnvFilter::new(tcfg.log_level.clone());

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().compact())
        .init();
}
