use time::Duration;

use crate::{error::AppError, routes};
use axum::{
    Router,
    http::StatusCode,
    routing::{any, get, post},
};
use tokio::sync::{mpsc::UnboundedReceiver, oneshot};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

pub fn build() -> Router {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let todo_service = TodoService::new(rx);
    let state = AppState { todo_sender: tx };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(30)));

    tokio::spawn(run_todo_service(todo_service));

    Router::new()
        // ---- routes ----
        .route("/", get(routes::home::index))
        .route("/todos", post(routes::home::add_todo))
        .route("/todos/clear-done", post(routes::home::clear_done))
        .route("/todos/{id}/toggle", post(routes::home::toggle_todo))
        .route("/toggle-js", post(routes::home::toggle_js))
        // ---- static files ----
        .fallback_service(axum::routing::get_service(
            tower_http::services::ServeDir::new("public")
                .fallback(any(|| async { AppError::NotFound })),
        ))
        // ---- middleware ----
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::seconds(10).unsigned_abs(),
                )),
        )
        .layer(session_layer)
        .with_state(state)
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub todo_sender: tokio::sync::mpsc::UnboundedSender<TodoCommand>,
}

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug)]
pub enum TodoCommand {
    Add(String),
    Remove(usize),
    Toggle(usize),
    List {
        responder: oneshot::Sender<Vec<TodoItem>>,
    },
}

pub struct TodoService {
    pub receiver: UnboundedReceiver<TodoCommand>,
    items: Vec<TodoItem>,
    next_id: usize,
}

impl TodoService {
    pub fn new(receiver: UnboundedReceiver<TodoCommand>) -> Self {
        Self {
            receiver,
            items: vec![
                TodoItem {
                    id: 1,
                    description: "Tell someone I code in rust".to_string(),
                    completed: false,
                },
                TodoItem {
                    id: 2,
                    description: "Build awesome web apps".to_string(),
                    completed: true,
                },
                TodoItem {
                    id: 3,
                    description: "Profit".to_string(),
                    completed: false,
                },
            ],
            next_id: 4,
        }
    }

    pub async fn handle_command(&mut self, command: TodoCommand) {
        match command {
            TodoCommand::Add(description) => {
                let item = TodoItem {
                    id: self.next_id,
                    description,
                    completed: false,
                };
                self.items.push(item);
                self.next_id += 1;
            }
            TodoCommand::Toggle(id) => {
                if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
                    item.completed = !item.completed;
                }
            }
            TodoCommand::Remove(id) => {
                self.items.retain(|item| item.id != id);
            }
            TodoCommand::List { responder } => {
                let _ = responder.send(self.items.clone());
            }
        }
    }
}

async fn run_todo_service(mut service: TodoService) {
    loop {
        if let Some(command) = service.receiver.recv().await {
            service.handle_command(command).await;
        }
    }
}
