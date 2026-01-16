// Avoid musl's default allocator due to lackluster performance
// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::net::SocketAddr;

use tokio::{net::TcpListener, signal};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod app;
mod error;
mod render;
mod routes;
mod views;

#[tokio::main]
async fn main() {
    // ---- tracing ----
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    tokio::spawn(async move {
        // ---- config ----
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

        // ---- build app ----
        let app = app::build();

        // ---- run server ----
        tracing::info!("listening on {}", addr);
        tracing::info!("Visit http://localhost:3000");
        tracing::info!("Press CTRL+C to stop the server");

        let listener = match TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(err) => {
                panic!("Failed to bind to address {}: {}", addr, err);
            }
        };

        if let Err(e) = axum::serve(listener, app).await {
            tracing::error!("Server error: {}", e);
        }
    });

    tokio::select! {
        ctrl_c = signal::ctrl_c() => match ctrl_c {
            Ok(()) => {},
            Err(err) => {
                tracing::error!("Unable to listen for shutdown signal: {}", err);
            }
        },
        close = close() => match close {
            Ok(()) => {},
            Err(err) => {
                tracing::error!("Unable to listen for shutdown signal: {}", err);
            }
        }
    }

    tracing::info!("Shutting down gracefully...");
}

#[cfg(windows)]
async fn close() -> Result<(), Box<dyn std::error::Error>> {
    let mut close_stream = signal::windows::ctrl_close()?;
    close_stream.recv().await;
    Ok(())
}

#[cfg(unix)]
async fn close() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = signal::unix::signal(signal::unix::SignalKind::terminate())?;
    let _ = stream.recv().await;
    Ok(())
}
