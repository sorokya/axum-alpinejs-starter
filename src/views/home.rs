use askama::Template;

use crate::{
    app::TodoItem,
    views::{errors::ErrorsView, todos::TodosOuterView},
};

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomePageView {
    pub title: Option<&'static str>,
    js: bool,
    todos: TodosOuterView,
    errors: ErrorsView,
}

impl HomePageView {
    pub fn new(todos: Vec<TodoItem>, js: bool, errors: Vec<String>) -> Self {
        Self {
            title: Some("Home"),
            js,
            todos: TodosOuterView::new(todos),
            errors: ErrorsView::new(errors),
        }
    }
}
