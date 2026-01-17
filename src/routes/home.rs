use axum::response::Html;

use crate::{error::AppError, render::render, views::home::HomePageView};

pub async fn index() -> Result<Html<String>, AppError> {
    render(HomePageView::new())
}
