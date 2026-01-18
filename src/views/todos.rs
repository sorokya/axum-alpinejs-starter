use askama::Template;

use crate::app::TodoItem;

#[derive(Template)]
#[template(path = "partials/todos.html")]
pub struct TodosView {
    todos: Vec<TodoItem>,
}

impl TodosView {
    pub fn new(todos: Vec<TodoItem>) -> Self {
        Self { todos }
    }
}
