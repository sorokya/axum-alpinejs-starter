use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomePageView {
    pub title: Option<&'static str>,
}

impl HomePageView {
    pub fn new() -> Self {
        Self {
            title: Some("Home"),
        }
    }
}
