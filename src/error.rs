use askama::Template;
use axum::response::{Html, IntoResponse};
use thiserror::Error;

use crate::views::error::ErrorPageView;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,

    #[error("Bad Request")]
    BadRequest(String),

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
            AppError::BadRequest(message) => (axum::http::StatusCode::BAD_REQUEST, message.clone()),
        };

        tracing::error!(error = %self);

        let html = ErrorPageView::new(status.as_u16(), message)
            .render()
            .unwrap_or_else(|_| "<h1>Internal Server Error</h1>".into());

        (status, Html(html)).into_response()
    }
}

impl AppError {
    pub fn get_message(&self) -> String {
        match self {
            AppError::NotFound => "Not Found".to_string(),
            AppError::BadRequest(msg) => format!("Bad Request: {}", msg),
            AppError::Template(err) => format!("Template Error: {}", err),
            AppError::Internal(err) => format!("Internal Server Error: {}", err),
        }
    }
}
