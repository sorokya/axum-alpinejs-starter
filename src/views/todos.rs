use askama::Template;

use crate::{app::TodoItem, views::todo::TodoView};

#[derive(Template)]
#[template(path = "partials/todos_inner.html")]
pub struct TodosInnerView {
    todos: Vec<TodoView>,
}

impl TodosInnerView {
    pub fn new(todos: Vec<TodoItem>) -> Self {
        Self {
            todos: todos.into_iter().map(TodoView::new).collect(),
        }
    }
}

#[derive(Template)]
#[template(path = "partials/todos_outer.html")]
pub struct TodosOuterView {
    todos: TodosInnerView,
}

impl TodosOuterView {
    pub fn new(todos: Vec<TodoItem>) -> Self {
        Self {
            todos: TodosInnerView::new(todos),
        }
    }
}
