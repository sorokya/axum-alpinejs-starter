use askama::Template;

use crate::app::TodoItem;

#[derive(Template)]
#[template(path = "partials/todo.html")]
pub struct TodoView {
    todo: TodoItem,
}

impl TodoView {
    pub fn new(todo: TodoItem) -> Self {
        Self { todo }
    }
}
