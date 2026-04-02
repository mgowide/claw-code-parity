use std::sync::Arc;

use axum::Json;
use axum::extract::State;

use crate::state::AppState;

pub async fn status(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "model": state.model,
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
