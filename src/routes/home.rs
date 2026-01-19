use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect, Response},
};
use axum_messages::{Level, Messages};
use tokio::sync::oneshot;
use tower_sessions::Session;

use crate::{
    alpine_request::AlpineRequest,
    app::{AppState, TodoCommand, TodoItem},
    error::AppError,
    render::render,
    views::{
        errors::ErrorsView, home::HomePageView, todo::TodoView, todos::TodosInnerView,
        todos::TodosOuterView,
    },
};

#[derive(serde::Deserialize)]
pub struct AddTodoRequest {
    description: String,
}

pub async fn index(
    messages: Messages,
    State(state): State<AppState>,
    session: Session,
) -> Result<Html<String>, AppError> {
    let (tx, rx) = oneshot::channel();
    if let Err(e) = state.todo_sender.send(TodoCommand::List { responder: tx }) {
        tracing::error!("Failed to send List command: {}", e);
        return Err(AppError::Internal(anyhow::anyhow!(
            "Failed to process request"
        )));
    }

    let items = rx.await.map_err(|e| {
        tracing::error!("Failed to receive todo items: {}", e);
        AppError::Internal(anyhow::anyhow!("Failed to process request"))
    })?;

    let js_enabled = session
        .get::<bool>("js_enabled")
        .await
        .unwrap_or(Some(true))
        .unwrap_or(true);

    let errors = messages
        .into_iter()
        .filter_map(|msg| {
            if msg.level == Level::Error {
                Some(msg.message)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    render(HomePageView::new(items, js_enabled, errors))
}

pub async fn toggle_js(session: Session) -> Result<Redirect, AppError> {
    let js_enabled = session
        .get::<bool>("js_enabled")
        .await
        .unwrap_or(Some(false))
        .unwrap_or(false);

    session
        .insert("js_enabled", !js_enabled)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update session: {}", e);
            AppError::Internal(anyhow::anyhow!("Failed to process request"))
        })?;

    Ok(Redirect::to("/"))
}

pub async fn add_todo(
    State(state): State<AppState>,
    messages: Messages,
    alpine: Option<AlpineRequest>,
    Form(AddTodoRequest { description }): Form<AddTodoRequest>,
) -> Result<Response, AppError> {
    let result = add_todo_inner(&state, description).await;

    if alpine.is_none() {
        if let Err(e) = &result {
            messages.error(e.get_message());
        }

        return Ok(Redirect::to("/").into_response());
    }

    let mut fragments = Vec::new();

    fragments.push(Fragment::Errors(
        render(ErrorsView::new(if let Err(e) = &result {
            vec![e.get_message()]
        } else {
            Vec::new()
        }))?
        .0,
    ));

    fragments.push(Fragment::TodosInner(
        render(TodosInnerView::new(if let Ok(item) = result {
            vec![item]
        } else {
            Vec::new()
        }))?
        .0,
    ));

    Ok(Html(select_fragments(fragments, &alpine.unwrap().targets)).into_response())
}

async fn add_todo_inner(state: &AppState, description: String) -> Result<TodoItem, AppError> {
    if description.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Description cannot be empty.".to_string(),
        ));
    }

    let (tx, rx) = oneshot::channel();
    if let Err(e) = state.todo_sender.send(TodoCommand::Add {
        description,
        responder: tx,
    }) {
        tracing::error!("Failed to send Add command: {}", e);
        return Err(AppError::Internal(anyhow::anyhow!(
            "Failed to process request"
        )));
    }

    let item = rx.await.map_err(|e| {
        tracing::error!("Failed to receive added todo item: {}", e);
        AppError::Internal(anyhow::anyhow!("Failed to process request"))
    })?;

    Ok(item)
}

pub async fn toggle_todo(
    State(state): State<AppState>,
    messages: Messages,
    axum::extract::Path(id): axum::extract::Path<usize>,
    alpine: Option<AlpineRequest>,
) -> Result<Response, AppError> {
    let result = toggle_todo_inner(&state, id).await;

    if alpine.is_none() {
        if let Err(e) = &result {
            messages.error(e.get_message());
        }

        return Ok(Redirect::to("/").into_response());
    }

    let mut fragments = Vec::new();

    fragments.push(Fragment::Errors(
        render(ErrorsView::new(if let Err(e) = &result {
            vec![e.get_message()]
        } else {
            Vec::new()
        }))?
        .0,
    ));

    if let Ok(Some(item)) = result {
        fragments.push(Fragment::Todo(id, render(TodoView::new(item))?.0));
    }

    Ok(Html(select_fragments(fragments, &alpine.unwrap().targets)).into_response())
}

async fn toggle_todo_inner(state: &AppState, id: usize) -> Result<Option<TodoItem>, AppError> {
    let (tx, rx) = oneshot::channel();
    if let Err(e) = state
        .todo_sender
        .send(TodoCommand::Toggle { id, responder: tx })
    {
        tracing::error!("Failed to send Toggle command: {}", e);
        return Err(AppError::Internal(anyhow::anyhow!(
            "Failed to process request"
        )));
    }

    let item = rx.await.map_err(|e| {
        tracing::error!("Failed to receive toggled todo item: {}", e);
        AppError::Internal(anyhow::anyhow!("Failed to process request"))
    })?;

    Ok(item)
}

pub async fn clear_done(
    State(state): State<AppState>,
    messages: Messages,
    alpine: Option<AlpineRequest>,
) -> Result<Response, AppError> {
    let result = clear_done_internal(&state).await;

    if alpine.is_none() {
        if let Err(e) = &result {
            messages.error(e.get_message());
        }

        return Ok(Redirect::to("/").into_response());
    }

    let mut fragments = Vec::new();

    fragments.push(Fragment::Errors(
        render(ErrorsView::new(if let Err(e) = &result {
            vec![e.get_message()]
        } else {
            Vec::new()
        }))?
        .0,
    ));

    if let Ok(todos) = result {
        fragments.push(Fragment::TodosOuter(render(TodosOuterView::new(todos))?.0));
    }

    Ok(Html(select_fragments(fragments, &alpine.unwrap().targets)).into_response())
}

async fn clear_done_internal(state: &AppState) -> Result<Vec<TodoItem>, AppError> {
    let (tx, rx) = oneshot::channel();
    if let Err(e) = state.todo_sender.send(TodoCommand::List { responder: tx }) {
        tracing::error!("Failed to send List command: {}", e);
        return Err(AppError::Internal(anyhow::anyhow!(
            "Failed to process request"
        )));
    }

    let items = rx.await.map_err(|e| {
        tracing::error!("Failed to receive todo items: {}", e);
        AppError::Internal(anyhow::anyhow!("Failed to process request"))
    })?;

    for item in items.into_iter().filter(|item| item.completed) {
        if let Err(e) = state.todo_sender.send(TodoCommand::Remove(item.id)) {
            tracing::error!("Failed to send Remove command: {}", e);
            return Err(AppError::Internal(anyhow::anyhow!(
                "Failed to process request"
            )));
        }
    }

    let (tx, rx) = oneshot::channel();
    if let Err(e) = state.todo_sender.send(TodoCommand::List { responder: tx }) {
        tracing::error!("Failed to send List command: {}", e);
        return Err(AppError::Internal(anyhow::anyhow!(
            "Failed to process request"
        )));
    }

    let remaining_items = rx.await.map_err(|e| {
        tracing::error!("Failed to receive todo items: {}", e);
        AppError::Internal(anyhow::anyhow!("Failed to process request"))
    })?;

    Ok(remaining_items)
}

enum Fragment {
    Errors(String),
    TodosOuter(String),
    TodosInner(String),
    Todo(usize, String),
}

impl Fragment {
    pub fn target(&self) -> String {
        match self {
            Fragment::Errors(_) => "error-messages".to_string(),
            Fragment::TodosOuter(_) => "todos-outer".to_string(),
            Fragment::TodosInner(_) => "todos-inner".to_string(),
            Fragment::Todo(id, _) => format!("todo-{}", id),
        }
    }

    pub fn html(&self) -> &str {
        match self {
            Fragment::Errors(html) => html,
            Fragment::TodosOuter(html) => html,
            Fragment::TodosInner(html) => html,
            Fragment::Todo(_, html) => html,
        }
    }
}

fn select_fragments(fragments: Vec<Fragment>, targets: &[String]) -> String {
    let mut html = String::new();

    for fragment in fragments {
        if targets.iter().any(|t| t == &fragment.target()) {
            html.push_str(fragment.html());
        }
    }

    html
}
