use askama::Template;

#[derive(Template)]
#[template(path = "partials/errors.html")]
pub struct ErrorsView {
    pub errors: Vec<String>,
}

impl ErrorsView {
    pub fn new(errors: Vec<String>) -> Self {
        Self { errors }
    }
}
