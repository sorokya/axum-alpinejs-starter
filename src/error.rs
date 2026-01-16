use askama::Template;
use axum::response::{Html, IntoResponse};
use thiserror::Error;

use crate::views::error::ErrorPageView;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,

    #[error("Template Rendering Error")]
    Template(#[from] askama::Error),

    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            AppError::NotFound => (axum::http::StatusCode::NOT_FOUND, self.to_string()),
            AppError::Template(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Template Error".to_string(),
            ),
            AppError::Internal(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };

        tracing::error!(error = %self);

        let html = ErrorPageView {
            status: status.as_u16(),
            message,
        }
        .render()
        .unwrap_or_else(|_| "<h1>Internal Server Error</h1>".into());

        (status, Html(html)).into_response()
    }
}
