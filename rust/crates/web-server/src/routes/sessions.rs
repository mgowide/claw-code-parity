use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::error::AppError;
use crate::state::{AppState, SessionHandle};
use crate::ws::events::ServerEvent;

#[derive(serde::Deserialize)]
pub struct CreateSessionRequest {
    pub model: Option<String>,
}

pub async fn create_session(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateSessionRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let session_id = Uuid::new_v4().to_string();
    let model = body.model.unwrap_or_else(|| state.model.clone());
    let created_at = chrono_now();

    let (tx, _rx) = broadcast::channel::<ServerEvent>(256);

    let handle = SessionHandle {
        session_id: session_id.clone(),
        model: model.clone(),
        created_at: created_at.clone(),
        tx,
    };

    state.sessions.insert(session_id.clone(), handle);

    Ok(Json(serde_json::json!({
        "session_id": session_id,
        "model": model,
        "created_at": created_at,
    })))
}

pub async fn list_sessions(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let sessions: Vec<serde_json::Value> = state
        .sessions
        .iter()
        .map(|entry| {
            serde_json::json!({
                "session_id": entry.session_id,
                "model": entry.model,
                "created_at": entry.created_at,
            })
        })
        .collect();

    Json(serde_json::json!({ "sessions": sessions }))
}

fn chrono_now() -> String {
    // Simple ISO 8601 timestamp without pulling in the chrono crate
    let d = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", d.as_secs())
}
