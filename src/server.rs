use axum::Router;
use std::net::SocketAddr;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub async fn serve_with_graceful_shutdown(app: Router, addr: SocketAddr) -> Result<(), BoxError> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("install ctrl-c handler");
    };
    #[cfg(unix)]
    let sigterm = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut term = signal(SignalKind::terminate()).expect("install sigterm handler");
        term.recv().await;
    };
    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::warn!("shutdown: ctrl-c"),
        _ = sigterm => tracing::warn!("shutdown: sigterm"),
    }
}
