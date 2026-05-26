use axum::{Router, routing::{get, post}};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use std::path::Path;
use tokio::net::UnixListener;
use tower::Service;

use crate::routes::{images, services, system};

// During development this lives in /tmp so root isn't required.
// The NixOS service config will use /run/canopy/daemon.sock in production.
const SOCKET_PATH: &str = "/tmp/canopy-daemon.sock";

pub async fn start() -> anyhow::Result<()> {
    // Remove a stale socket file left over from a previous run.
    if Path::new(SOCKET_PATH).exists() {
        std::fs::remove_file(SOCKET_PATH)?;
    }

    let app = Router::new()
        .route("/images/search", get(images::search))
        .route("/images/pull",   post(images::pull))
        .route("/services",      get(services::list))
        .route("/system",        get(system::info));

    let listener = UnixListener::bind(SOCKET_PATH)?;
    tracing::info!("Canopy daemon listening on {}", SOCKET_PATH);

    // Axum's built-in `serve` only handles TCP, so we drive the Unix socket
    // manually using hyper-util. Each accepted connection gets its own task.
    loop {
        let (stream, _) = listener.accept().await?;
        let app = app.clone();

        tokio::spawn(async move {
            let io = TokioIo::new(stream);
            let svc = hyper::service::service_fn(move |req| {
                // `call` requires a mutable reference in tower 0.4
                let mut app = app.clone();
                async move { app.call(req).await }
            });
            Builder::new(TokioExecutor::new())
                .serve_connection(io, svc)
                .await
                .ok();
        });
    }
}
