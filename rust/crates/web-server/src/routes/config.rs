use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::state::AppState;

fn config_path(state: &AppState) -> std::path::PathBuf {
    state.workspace_root.join(".claude.json")
}

fn default_config() -> serde_json::Value {
    serde_json::json!({
        "model": "claude-sonnet-4-20250514",
        "permission_mode": "default",
        "allowed_tools": ["bash", "read_file", "edit_file", "write_file", "list_dir"],
        "hooks": {
            "pre_tool_call": "",
            "post_session": ""
        },
        "mcp_servers": [],
        "sandbox": { "enabled": false }
    })
}

/// `GET /api/config` — read .claude.json (or return defaults if not present).
pub async fn get_config(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let path = config_path(&state);
    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(parsed) = serde_json::from_str(&content) {
            return Json(parsed);
        }
    }
    Json(default_config())
}

/// `PUT /api/config` — write .claude.json (with backup).
pub async fn put_config(
    State(state): State<Arc<AppState>>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    // Basic validation: must be an object
    if !body.is_object() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "config must be a JSON object" })),
        ).into_response();
    }

    let path = config_path(&state);
    // Backup existing
    if path.exists() {
        let bak = path.with_extension("json.bak");
        let _ = std::fs::copy(&path, &bak);
    }

    let formatted = match serde_json::to_string_pretty(&body) {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    };

    if let Err(e) = std::fs::write(&path, formatted) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response();
    }

    Json(body).into_response()
}
