use askama::Template;

use crate::{app::TodoItem, views::todos::TodosView};

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomePageView {
    pub title: Option<&'static str>,
    js: bool,
    todos: TodosView,
    errors: Vec<String>,
}

impl HomePageView {
    pub fn new(todos: Vec<TodoItem>, js: bool, errors: Vec<String>) -> Self {
        Self {
            title: Some("Home"),
            js,
            todos: TodosView::new(todos),
            errors,
        }
    }
}
