use askama::Template;

#[derive(Template)]
#[template(path = "pages/error.html")]
pub struct ErrorPageView {
    pub status: u16,
    pub message: String,
}
