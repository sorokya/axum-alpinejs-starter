use askama::Template;
use axum::response::Html;

use crate::error::AppError;

pub fn render<T: Template>(template: T) -> Result<Html<String>, AppError> {
    let html = template.render()?;
    Ok(Html(html))
}
