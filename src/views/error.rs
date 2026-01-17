use askama::Template;

#[derive(Template)]
#[template(path = "pages/error.html")]
pub struct ErrorPageView {
    pub status: u16,
    pub message: String,
    title: Option<&'static str>,
}

impl ErrorPageView {
    pub fn new(status: u16, message: String) -> Self {
        Self {
            status,
            message,
            title: Some("Error"),
        }
    }
}
