mod health;
mod sessions;

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};

use crate::state::AppState;
use crate::ws::handler::ws_upgrade;

pub fn build_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/status", get(health::status))
        .route("/api/sessions", post(sessions::create_session))
        .route("/api/sessions", get(sessions::list_sessions))
        .route("/ws", get(ws_upgrade))
        .with_state(state)
}
