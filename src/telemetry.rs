use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

use crate::config::TelemetryCfg;

pub fn init(tcfg: &TelemetryCfg) {
    let filter = EnvFilter::new(tcfg.log_level.clone());

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().compact())
        .init();
}

pub fn init_metrics() -> PrometheusHandle {
    PrometheusBuilder::new()
        .install_recorder()
        .expect("install prometheus recorder")
}
