use std::time::Duration;

use crate::{error::AppError, routes};
use axum::{
    Router,
    http::StatusCode,
    routing::{any, get},
};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};

pub fn build() -> Router {
    Router::new()
        // ---- routes ----
        .route("/", get(routes::home::index))
        // ---- static files ----
        .fallback_service(axum::routing::get_service(
            tower_http::services::ServeDir::new("public")
                .fallback(any(|| async { AppError::NotFound })),
        ))
        // ---- middleware ----
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(10),
                )),
        )
}
