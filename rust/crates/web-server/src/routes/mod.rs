mod health;
mod sessions;

use std::sync::Arc;

use axum::Router;
use axum::routing::{delete, get, post};

use crate::state::AppState;
use crate::ws::handler::ws_upgrade;

pub fn build_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/status", get(health::status))
        .route("/api/sessions", post(sessions::create_session))
        .route("/api/sessions", get(sessions::list_sessions))
        .route("/api/sessions/{id}", get(sessions::get_session))
        .route("/api/sessions/{id}", delete(sessions::delete_session))
        .route("/api/sessions/{id}/export", get(sessions::export_session))
        .route("/ws", get(ws_upgrade))
        .with_state(state)
}
