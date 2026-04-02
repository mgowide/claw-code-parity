use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use uuid::Uuid;

use crate::state::{AppState, McpServerEntry};

/// `GET /api/mcp/servers`
pub async fn list_servers(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let servers: Vec<McpServerEntry> = state.mcp_servers.iter().map(|r| r.value().clone()).collect();
    Json(serde_json::json!({ "servers": servers }))
}

#[derive(Deserialize)]
pub struct AddServerBody {
    name: String,
    transport: String,
    command: Option<String>,
    url: Option<String>,
}

/// `POST /api/mcp/servers`
pub async fn add_server(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AddServerBody>,
) -> impl IntoResponse {
    if body.transport != "stdio" && body.transport != "sse" {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "transport must be 'stdio' or 'sse'" })),
        ).into_response();
    }

    let id = Uuid::new_v4().to_string();
    let entry = McpServerEntry {
        id: id.clone(),
        name: body.name,
        transport: body.transport,
        command: body.command,
        url: body.url,
        status: "connected".to_string(),
        error: None,
        tools: Vec::new(),
        connected_at: Some(chrono_now()),
    };
    state.mcp_servers.insert(id, entry.clone());
    (StatusCode::CREATED, Json(serde_json::json!(entry))).into_response()
}

/// `DELETE /api/mcp/servers/:id`
pub async fn remove_server(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if state.mcp_servers.remove(&id).is_some() {
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "not found" }))).into_response()
    }
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    // Minimal ISO-8601-ish timestamp without chrono
    format!("{secs}")
}
