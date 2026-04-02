use std::sync::Arc;

use axum::Json;
use axum::extract::State;

use crate::state::AppState;

pub async fn status(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let active = &state.model;
    Json(serde_json::json!({
        "status": "ok",
        "model": active,
        "version": env!("CARGO_PKG_VERSION"),
        "models": [
            { "id": "claude-sonnet-4-20250514", "provider": "anthropic", "active": active == "claude-sonnet-4-20250514" },
            { "id": "claude-opus-4-20250514",   "provider": "anthropic", "active": active == "claude-opus-4-20250514"   },
            { "id": "grok-3",                   "provider": "xai",       "active": active == "grok-3"                   },
            { "id": "gpt-4o",                   "provider": "openai-compat", "active": active == "gpt-4o"              },
        ]
    }))
}
