use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::error::AppError;
use crate::state::{AppState, SessionHandle};
use crate::ws::events::ServerEvent;

#[derive(serde::Deserialize)]
pub struct CreateSessionRequest {
    pub model: Option<String>,
    pub name: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>,
}

pub async fn create_session(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateSessionRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let session_id = Uuid::new_v4().to_string();
    let model = body.model.unwrap_or_else(|| state.model.clone());
    let now = unix_now_str();

    let (tx, _rx) = broadcast::channel::<ServerEvent>(256);

    let handle = SessionHandle {
        session_id: session_id.clone(),
        name: body.name.unwrap_or_else(|| "New chat".to_string()),
        model: model.clone(),
        created_at: now.clone(),
        updated_at: now.clone(),
        messages: Vec::new(),
        usage: Default::default(),
        tx,
    };

    state.sessions.insert(session_id.clone(), handle);

    Ok(Json(serde_json::json!({
        "session_id": session_id,
        "model": model,
        "created_at": now,
    })))
}

pub async fn list_sessions(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let mut sessions: Vec<serde_json::Value> = state
        .sessions
        .iter()
        .map(|entry| {
            serde_json::json!({
                "id": entry.session_id,
                "name": entry.name,
                "model": entry.model,
                "created_at": entry.created_at,
                "updated_at": entry.updated_at,
                "message_count": entry.messages.len(),
                "input_tokens": entry.usage.input_tokens,
                "output_tokens": entry.usage.output_tokens,
                "cost": entry.usage.cost,
            })
        })
        .collect();

    // Sort by updated_at descending (newest first)
    sessions.sort_by(|a, b| {
        let ta = a["updated_at"].as_str().unwrap_or("");
        let tb = b["updated_at"].as_str().unwrap_or("");
        tb.cmp(ta)
    });

    Json(serde_json::json!({ "sessions": sessions }))
}

pub async fn get_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let entry = state
        .sessions
        .get(&id)
        .ok_or_else(|| AppError::SessionNotFound(id.clone()))?;

    Ok(Json(serde_json::json!({
        "id": entry.session_id,
        "name": entry.name,
        "model": entry.model,
        "created_at": entry.created_at,
        "updated_at": entry.updated_at,
        "messages": entry.messages,
        "usage": {
            "input_tokens": entry.usage.input_tokens,
            "output_tokens": entry.usage.output_tokens,
            "cache_hits": entry.usage.cache_hits,
            "cost": entry.usage.cost,
        }
    })))
}

pub async fn delete_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    if state.sessions.remove(&id).is_none() {
        return Err(AppError::SessionNotFound(id));
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn export_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<ExportQuery>,
) -> Result<(axum::http::HeaderMap, String), AppError> {
    let entry = state
        .sessions
        .get(&id)
        .ok_or_else(|| AppError::SessionNotFound(id.clone()))?;

    let format = params.format.as_deref().unwrap_or("md");

    let mut headers = axum::http::HeaderMap::new();

    if format == "json" {
        let body = serde_json::to_string_pretty(&serde_json::json!({
            "id": entry.session_id,
            "name": entry.name,
            "model": entry.model,
            "created_at": entry.created_at,
            "messages": entry.messages,
            "usage": entry.usage,
        }))
        .unwrap_or_default();

        headers.insert(
            axum::http::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"session-{id}.json\"").parse().unwrap(),
        );
        Ok((headers, body))
    } else {
        // Markdown export
        let body = build_markdown_export(&entry);
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            "text/markdown; charset=utf-8".parse().unwrap(),
        );
        headers.insert(
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"session-{id}.md\"").parse().unwrap(),
        );
        Ok((headers, body))
    }
}

fn build_markdown_export(entry: &crate::state::SessionHandle) -> String {
    let mut md = format!(
        "# Session: {}\n\n**Model**: {} | **Created**: {} | **Cost**: ${:.4}\n\n---\n\n",
        entry.name, entry.model, entry.created_at, entry.usage.cost
    );

    for msg in &entry.messages {
        let role = if msg.role == "user" { "## User" } else { "## Assistant" };
        md.push_str(&format!("{}\n\n{}\n\n---\n\n", role, msg.content));
    }

    md
}

fn unix_now_str() -> String {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", d.as_secs())
}
