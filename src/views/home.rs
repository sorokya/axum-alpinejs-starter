use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomePageView;
