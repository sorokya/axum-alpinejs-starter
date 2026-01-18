use axum::{
    extract::State,
    response::{Html, Redirect},
};
use axum_messages::{Level, Messages};
use tokio::sync::oneshot;
use tower_sessions::Session;

use crate::{
    app::{AppState, TodoCommand},
    error::AppError,
    render::render,
    views::home::HomePageView,
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
    messages: Messages,
    State(state): State<AppState>,
    axum::extract::Form(AddTodoRequest { description }): axum::extract::Form<AddTodoRequest>,
) -> Result<Redirect, AppError> {
    if description.trim().is_empty() {
        messages.error("Description can not be blank");
        return Ok(Redirect::to("/"));
    }

    if let Err(e) = state.todo_sender.send(TodoCommand::Add(description)) {
        tracing::error!("Failed to send Add command: {}", e);
        return Err(AppError::Internal(anyhow::anyhow!(
            "Failed to process request"
        )));
    }

    Ok(Redirect::to("/"))
}

pub async fn toggle_todo(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<usize>,
) -> Result<Redirect, AppError> {
    if let Err(e) = state.todo_sender.send(TodoCommand::Toggle(id)) {
        tracing::error!("Failed to send Toggle command: {}", e);
        return Err(AppError::Internal(anyhow::anyhow!(
            "Failed to process request"
        )));
    }

    Ok(Redirect::to("/"))
}

pub async fn clear_done(State(state): State<AppState>) -> Result<Redirect, AppError> {
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

    Ok(Redirect::to("/"))
}
